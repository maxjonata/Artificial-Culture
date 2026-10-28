#include "artificial_culture/cognition.hpp"

#include <algorithm>
#include <cmath>
#include <cstdint>
#include <stdexcept>
#include <string>
#include <utility>

namespace artificial_culture {
namespace {

struct Candidate {
    ActionKind action;
    NeedKind need;
    WorldEntityId target_id;
    float utility;
    float distance;
};

float& need_value(NeedState& needs, NeedKind kind) {
    switch (kind) {
    case NeedKind::hunger: return needs.hunger;
    case NeedKind::thirst: return needs.thirst;
    case NeedKind::rest: return needs.rest;
    case NeedKind::safety: return needs.safety;
    case NeedKind::social: return needs.social;
    case NeedKind::count: break;
    }
    throw std::invalid_argument("NeedKind::count is not a need value");
}

float need_utility(float satisfaction, float activation_threshold, float priority) {
    const float max_urgency = std::max(1.0F - std::min(activation_threshold, 0.999F), 0.001F);
    return ((1.0F - satisfaction) / max_urgency) * priority;
}

bool valid_perception(const PerceptionSnapshot& perception) {
    for (const ResourceObservation& resource : perception.visible_resources) {
        if (resource.id == 0 || !std::isfinite(resource.distance) || resource.distance < 0.0F ||
            !std::isfinite(resource.availability) || resource.availability < 0.0F || resource.availability > 1.0F) {
            return false;
        }
    }
    for (const AgentObservation& agent : perception.visible_agents) {
        if (agent.id == 0 || !std::isfinite(agent.distance) || agent.distance < 0.0F) {
            return false;
        }
    }
    return true;
}

bool valid_event_observation(const EventObservation& observation) {
    const auto kind = static_cast<std::uint8_t>(observation.perceived_kind);
    return observation.observation_id != 0 && observation.event_id != 0 &&
        kind <= static_cast<std::uint8_t>(PerceivedEventKind::resource_discovery) &&
        std::isfinite(observation.confidence) && observation.confidence >= 0.0F && observation.confidence <= 1.0F &&
        std::isfinite(observation.observed_at_seconds) && observation.observed_at_seconds >= 0.0;
}

bool valid_testimony(const Testimony& testimony) {
    return testimony.id != 0 && testimony.event_id != 0 && testimony.speaker_id != 0 &&
        testimony.parent_testimony_id != testimony.id &&
        !testimony.statement.empty() && testimony.statement.find_first_not_of(" \t\n\r") != std::string::npos &&
        std::isfinite(testimony.confidence) && testimony.confidence >= 0.0F && testimony.confidence <= 1.0F &&
        std::isfinite(testimony.heard_at_seconds) && testimony.heard_at_seconds >= 0.0;
}

bool valid_social_episode(const SocialEpisode& episode) {
    return episode.id != 0 && episode.partner_id != 0 &&
        static_cast<std::uint8_t>(episode.perceived_kind) <= static_cast<std::uint8_t>(SocialEpisodeKind::rejection) &&
        !episode.perceived_summary.empty() &&
        episode.perceived_summary.find_first_not_of(" \t\n\r") != std::string::npos &&
        std::isfinite(episode.remembered_at_seconds) && episode.remembered_at_seconds >= 0.0;
}

bool valid_personality(const Personality& personality) {
    const std::array<float, 5> traits{
        personality.openness,
        personality.extraversion,
        personality.agreeableness,
        personality.conscientiousness,
        personality.neuroticism,
    };
    return std::all_of(traits.begin(), traits.end(), [](float value) {
        return std::isfinite(value) && value >= 0.0F && value <= 1.0F;
    });
}

bool valid_needs(const NeedState& needs) {
    const std::array<float, 5> values{
        needs.hunger,
        needs.thirst,
        needs.rest,
        needs.safety,
        needs.social,
    };
    return std::all_of(values.begin(), values.end(), [](float value) {
        return std::isfinite(value) && value >= 0.0F && value <= 1.0F;
    });
}

bool valid_tuning(const CognitionTuning& tuning) {
    const std::array<float, 7> decay_rates{
        tuning.hunger_decay_per_second,
        tuning.thirst_decay_per_second,
        tuning.rest_decay_per_second,
        tuning.safety_decay_per_second,
        tuning.social_decay_per_second,
        tuning.rumor_decay_per_second,
        tuning.rumor_injection_delay_seconds,
    };
    const bool decays_valid = std::all_of(decay_rates.begin(), decay_rates.end(), [](float value) {
        return std::isfinite(value) && value >= 0.0F;
    });
    const auto unit_interval = [](float value) {
        return std::isfinite(value) && value >= 0.0F && value <= 1.0F;
    };
    return decays_valid &&
        unit_interval(tuning.activation_threshold) &&
        unit_interval(tuning.rumor_injection_threshold) &&
        std::isfinite(tuning.rumor_social_distance) && tuning.rumor_social_distance >= 0.0F &&
        unit_interval(tuning.rumor_base_transmission_rate) &&
        unit_interval(tuning.rumor_transmission_belief_factor) &&
        unit_interval(tuning.rumor_removal_threshold);
}

} // namespace

CognitionModule::CognitionModule(flecs::world& world, CognitionTuning tuning)
    : world_(world), tuning_(tuning), random_(tuning.random_seed) {
    if (!valid_tuning(tuning_)) {
        throw std::invalid_argument("Cognition tuning values are outside their valid ranges");
    }
    world_.component<NpcIdentity>();
    world_.component<NeedState>();
    world_.component<PerceptionSnapshot>();
    world_.component<PendingAction>();
    world_.component<ActionIntent>();
    world_.component<ActionLearning>();
    world_.component<Personality>();
    world_.component<KnowledgeBase>();
    world_.component<ResourceMemory>();
    world_.component<EventMemory>();
    world_.component<TestimonyMemory>();
    world_.component<RelationshipMemory>();

    need_query_ = world_.query<NpcIdentity, NeedState>();
    decision_query_ = world_.query<NpcIdentity, NeedState, PerceptionSnapshot, ResourceMemory, PendingAction, ActionIntent>();
}

flecs::entity CognitionModule::add_agent(AgentId id, NeedState initial_needs, Personality personality) {
    if (id == 0) {
        throw std::invalid_argument("Agent IDs must be non-zero");
    }
    if (!valid_personality(personality)) {
        throw std::invalid_argument("Personality traits must be finite values in [0, 1]");
    }
    if (!valid_needs(initial_needs)) {
        throw std::invalid_argument("Need satisfaction values must be finite values in [0, 1]");
    }
    if (const auto existing = agents_.find(id); existing != agents_.end()) {
        return existing->second;
    }

    auto agent = world_.entity((std::string("npc_") + std::to_string(id)).c_str())
        .set<NpcIdentity>({id, 1})
        .set<NeedState>(initial_needs)
        .set<PerceptionSnapshot>({})
        .set<PendingAction>({})
        .set<ActionIntent>({})
        .set<ActionLearning>({})
        .set<Personality>(personality)
        .set<KnowledgeBase>({})
        .set<ResourceMemory>({})
        .set<EventMemory>({})
        .set<TestimonyMemory>({})
        .set<RelationshipMemory>({});
    agents_.emplace(id, agent);
    return agent;
}

bool CognitionModule::remove_agent(AgentId id) {
    const auto found = agents_.find(id);
    if (found == agents_.end()) {
        return false;
    }

    const flecs::entity entity = found->second;
    agents_.erase(found);
    std::erase_if(social_contacts_, [id](const auto& contact) {
        return contact.first == id || contact.second == id;
    });
    entity.destruct();
    return true;
}

bool CognitionModule::observe(AgentId id, PerceptionSnapshot perception) {
    const auto found = agents_.find(id);
    if (found == agents_.end() || !valid_perception(perception)) {
        return false;
    }
    ResourceMemory& memory = found->second.get_mut<ResourceMemory>();
    for (const ResourceObservation& resource : perception.visible_resources) {
        memory.known_resources.insert_or_assign(resource.id, resource);
    }
    found->second.set<PerceptionSnapshot>(std::move(perception));
    return true;
}

bool CognitionModule::observe_event(AgentId id, EventObservation observation) {
    const auto found = agents_.find(id);
    if (found == agents_.end() || !valid_event_observation(observation)) {
        return false;
    }

    EventMemory& memory = found->second.get_mut<EventMemory>();
    return memory.observations.emplace(observation.observation_id, std::move(observation)).second;
}

bool CognitionModule::hear_testimony(AgentId listener_id, Testimony testimony) {
    const auto listener = agents_.find(listener_id);
    if (listener == agents_.end() || testimony.speaker_id == listener_id ||
        !agents_.contains(testimony.speaker_id) || !valid_testimony(testimony)) {
        return false;
    }

    TestimonyMemory& memory = listener->second.get_mut<TestimonyMemory>();
    return memory.received.emplace(testimony.id, std::move(testimony)).second;
}

bool CognitionModule::remember_social_episode(AgentId observer_id, SocialEpisode episode) {
    const auto observer = agents_.find(observer_id);
    if (observer == agents_.end() || episode.partner_id == observer_id ||
        !agents_.contains(episode.partner_id) || !valid_social_episode(episode)) {
        return false;
    }

    RelationshipMemory& memory = observer->second.get_mut<RelationshipMemory>();
    return memory.episodes.emplace(episode.id, std::move(episode)).second;
}

bool CognitionModule::report_social_contact(AgentId first, AgentId second) {
    if (first == second || !agents_.contains(first) || !agents_.contains(second)) {
        return false;
    }
    social_contacts_.emplace_back(first, second);
    return true;
}

void CognitionModule::step(float delta_seconds) {
    if (!std::isfinite(delta_seconds) || delta_seconds < 0.0F) {
        throw std::invalid_argument("delta_seconds must be finite and non-negative");
    }
    decay_needs(delta_seconds);
    choose_actions();
    process_social_contacts();
    inject_initial_rumor(delta_seconds);
    decay_rumors(delta_seconds);
}

void CognitionModule::decay_needs(float delta_seconds) {
    need_query_.each([this, delta_seconds](NpcIdentity&, NeedState& needs) {
        needs.hunger = std::clamp(needs.hunger - tuning_.hunger_decay_per_second * delta_seconds, 0.0F, 1.0F);
        needs.thirst = std::clamp(needs.thirst - tuning_.thirst_decay_per_second * delta_seconds, 0.0F, 1.0F);
        needs.rest = std::clamp(needs.rest - tuning_.rest_decay_per_second * delta_seconds, 0.0F, 1.0F);
        needs.safety = std::clamp(needs.safety - tuning_.safety_decay_per_second * delta_seconds, 0.0F, 1.0F);
        needs.social = std::clamp(needs.social - tuning_.social_decay_per_second * delta_seconds, 0.0F, 1.0F);
    });
}

void CognitionModule::choose_actions() {
    decision_query_.each([this](NpcIdentity& identity, NeedState& needs, PerceptionSnapshot& perception,
                                ResourceMemory& resource_memory, PendingAction& pending, ActionIntent& outbox) {
        if (pending.awaiting_result || outbox.ready) {
            return;
        }

        std::optional<Candidate> best;
        const auto consider_resource = [&](ResourceKind kind, NeedKind need, ActionKind action, float priority) {
            const float satisfaction = need_value(needs, need);
            if (satisfaction >= tuning_.activation_threshold) {
                return;
            }
            for (const auto& [resource_id, resource] : resource_memory.known_resources) {
                if (resource.kind != kind || resource.availability <= 0.0F) {
                    continue;
                }
                const float utility = need_utility(satisfaction, tuning_.activation_threshold, priority);
                if (!best || utility > best->utility ||
                    (best->action == action && utility == best->utility && resource.distance < best->distance)) {
                    best = Candidate{action, need, resource_id, utility, resource.distance};
                }
            }
        };

        // Legacy utility weights are retained, with deterministic tie-breaking by this order.
        consider_resource(ResourceKind::safe_zone, NeedKind::safety, ActionKind::find_safety, 0.75F);
        consider_resource(ResourceKind::water, NeedKind::thirst, ActionKind::find_water, 1.0F);
        consider_resource(ResourceKind::food, NeedKind::hunger, ActionKind::find_food, 0.85F);
        consider_resource(ResourceKind::rest, NeedKind::rest, ActionKind::rest, 0.6F);

        const float social_satisfaction = needs.social;
        if (social_satisfaction < tuning_.activation_threshold && !perception.visible_agents.empty()) {
            const AgentObservation& target = *std::min_element(
                perception.visible_agents.begin(),
                perception.visible_agents.end(),
                [](const AgentObservation& lhs, const AgentObservation& rhs) {
                    return lhs.distance < rhs.distance;
                });
            const float utility = need_utility(social_satisfaction, tuning_.activation_threshold, 0.3F);
            if (!best || utility > best->utility) {
                best = Candidate{ActionKind::socialize, NeedKind::social, target.id, utility, target.distance};
            }
        }

        if (!best) {
            return;
        }

        const std::uint64_t sequence = identity.next_action_sequence++;
        pending.awaiting_result = true;
        pending.dispatched = false;
        pending.sequence = sequence;
        pending.action = best->action;
        pending.target_id = best->target_id;

        outbox.ready = true;
        outbox.agent_id = identity.id;
        outbox.sequence = sequence;
        outbox.action = best->action;
        outbox.target_id = best->target_id;
        outbox.utility = best->utility;
    });
}

std::vector<ActionIntent> CognitionModule::take_intents() {
    std::vector<ActionIntent> intents;
    intents.reserve(agents_.size());
    for (auto& [id, agent] : agents_) {
        (void)id;
        PendingAction& pending = agent.get_mut<PendingAction>();
        const ActionIntent& outbox = agent.get<ActionIntent>();
        if (!outbox.ready || pending.dispatched) {
            continue;
        }
        intents.push_back(outbox);
        pending.dispatched = true;
    }
    return intents;
}

std::vector<RumorEvent> CognitionModule::take_rumor_events() {
    std::vector<RumorEvent> events = std::move(rumor_events_);
    rumor_events_.clear();
    return events;
}

void CognitionModule::process_social_contacts() {
    std::vector<RumorTransmissionAttempt> attempts;
    for (const auto& [first, second] : social_contacts_) {
        collect_rumor_transmission_attempts(first, second, attempts);
        collect_rumor_transmission_attempts(second, first, attempts);
    }
    social_contacts_.clear();

    std::uniform_real_distribution<float> chance(0.0F, 1.0F);
    for (const RumorTransmissionAttempt& attempt : attempts) {
        const bool succeeded = chance(random_) < attempt.probability;
        rumor_events_.push_back(RumorEvent{
            RumorEvent::Kind::spread_attempt,
            attempt.sender_id,
            attempt.receiver_id,
            attempt.content,
            attempt.probability,
            0.0F,
            succeeded,
        });
        if (!succeeded) {
            continue;
        }

        const float new_belief = attempt.belief_strength * tuning_.rumor_transmission_belief_factor;
        agents_.at(attempt.receiver_id).get_mut<KnowledgeBase>().known_rumors.insert_or_assign(
            attempt.content,
            new_belief);
        rumor_events_.push_back(RumorEvent{
            RumorEvent::Kind::spread,
            attempt.sender_id,
            attempt.receiver_id,
            attempt.content,
            attempt.probability,
            new_belief,
            true,
        });
    }
}

void CognitionModule::collect_rumor_transmission_attempts(
    AgentId sender_id,
    AgentId receiver_id,
    std::vector<RumorTransmissionAttempt>& attempts) const {
    flecs::entity sender = agents_.at(sender_id);
    flecs::entity receiver = agents_.at(receiver_id);
    const Personality& sender_personality = sender.get<Personality>();
    const Personality& receiver_personality = receiver.get<Personality>();
    const KnowledgeBase& sender_knowledge = sender.get<KnowledgeBase>();
    KnowledgeBase& receiver_knowledge = receiver.get_mut<KnowledgeBase>();

    const float distance_factor = 1.0F / (1.0F + tuning_.rumor_social_distance * 0.01F);
    const float sender_openness_factor = 0.5F + sender_personality.openness * 0.5F;
    const float receiver_openness_factor = 0.5F + receiver_personality.openness * 0.5F;
    const float probability = std::clamp(
        tuning_.rumor_base_transmission_rate * distance_factor * sender_openness_factor * receiver_openness_factor,
        0.0F,
        1.0F);

    for (const auto& [content, belief_strength] : sender_knowledge.known_rumors) {
        if (receiver_knowledge.known_rumors.contains(content)) {
            continue;
        }
        attempts.push_back(RumorTransmissionAttempt{
            sender_id,
            receiver_id,
            content,
            belief_strength,
            probability,
        });
    }
}

void CognitionModule::inject_initial_rumor(float delta_seconds) {
    if (rumor_injection_finished_) {
        return;
    }
    rumor_injection_elapsed_ += delta_seconds;
    if (rumor_injection_elapsed_ < tuning_.rumor_injection_delay_seconds) {
        return;
    }
    rumor_injection_finished_ = true;

    std::uniform_real_distribution<float> chance(0.0F, 1.0F);
    std::uniform_int_distribution<std::uint32_t> rumor_number(0, 999);
    for (const auto& [id, entity] : agents_) {
        const Personality& personality = entity.get<Personality>();
        const float injection_probability = (
            personality.openness * 0.4F +
            personality.extraversion * 0.3F +
            (1.0F - personality.agreeableness) * 0.3F) * tuning_.rumor_injection_threshold;
        if (chance(random_) >= std::clamp(injection_probability, 0.0F, 1.0F)) {
            continue;
        }

        const std::string content = "Rumor_" + std::to_string(rumor_number(random_));
        entity.get_mut<KnowledgeBase>().known_rumors.insert_or_assign(content, 1.0F);
        rumor_events_.push_back(RumorEvent{
            RumorEvent::Kind::injected,
            id,
            id,
            content,
            1.0F,
            1.0F,
            true,
        });
        break;
    }
}

void CognitionModule::decay_rumors(float delta_seconds) {
    for (const auto& [id, entity] : agents_) {
        (void)id;
        KnowledgeBase& knowledge = entity.get_mut<KnowledgeBase>();
        for (auto rumor = knowledge.known_rumors.begin(); rumor != knowledge.known_rumors.end();) {
            const float belief = std::clamp(
                rumor->second - tuning_.rumor_decay_per_second * delta_seconds,
                0.0F,
                1.0F);
            if (belief < tuning_.rumor_removal_threshold) {
                rumor = knowledge.known_rumors.erase(rumor);
            } else {
                rumor->second = belief;
                ++rumor;
            }
        }
    }
}

bool CognitionModule::report_result(const ActionResult& result) {
    const auto found = agents_.find(result.agent_id);
    if (found == agents_.end() || !std::isfinite(result.satisfaction_delta)) {
        return false;
    }

    const PendingAction& pending = found->second.get<PendingAction>();
    if (!pending.awaiting_result || !pending.dispatched || pending.sequence != result.sequence) {
        return false;
    }

    const ActionKind action = pending.action;
    ActionLearning& learning = found->second.get_mut<ActionLearning>();
    auto& counts = result.accepted ? learning.accepted : learning.rejected;
    ++counts[static_cast<std::size_t>(action)];

    if (result.accepted && result.affected_need != NeedKind::count) {
        NeedState& needs = found->second.get_mut<NeedState>();
        float& value = need_value(needs, result.affected_need);
        value = std::clamp(value + result.satisfaction_delta, 0.0F, 1.0F);
    }

    PendingAction& mutable_pending = found->second.get_mut<PendingAction>();
    mutable_pending.awaiting_result = false;
    mutable_pending.dispatched = false;
    mutable_pending.sequence = 0;
    mutable_pending.target_id = 0;

    ActionIntent& outbox = found->second.get_mut<ActionIntent>();
    outbox.ready = false;
    return true;
}

std::optional<NeedState> CognitionModule::needs_for(AgentId id) const {
    const auto found = agents_.find(id);
    if (found == agents_.end()) {
        return std::nullopt;
    }
    return found->second.get<NeedState>();
}

std::optional<ActionLearning> CognitionModule::learning_for(AgentId id) const {
    const auto found = agents_.find(id);
    if (found == agents_.end()) {
        return std::nullopt;
    }
    return found->second.get<ActionLearning>();
}

std::optional<KnowledgeBase> CognitionModule::knowledge_for(AgentId id) const {
    const auto found = agents_.find(id);
    if (found == agents_.end()) {
        return std::nullopt;
    }
    return found->second.get<KnowledgeBase>();
}

std::optional<EventMemory> CognitionModule::event_memory_for(AgentId id) const {
    const auto found = agents_.find(id);
    if (found == agents_.end()) {
        return std::nullopt;
    }
    return found->second.get<EventMemory>();
}

std::optional<TestimonyMemory> CognitionModule::testimony_memory_for(AgentId id) const {
    const auto found = agents_.find(id);
    if (found == agents_.end()) {
        return std::nullopt;
    }
    return found->second.get<TestimonyMemory>();
}

std::optional<RelationshipMemory> CognitionModule::relationship_memory_for(AgentId id) const {
    const auto found = agents_.find(id);
    if (found == agents_.end()) {
        return std::nullopt;
    }
    return found->second.get<RelationshipMemory>();
}

const char* to_string(ActionKind action) noexcept {
    switch (action) {
    case ActionKind::find_food: return "find_food";
    case ActionKind::find_water: return "find_water";
    case ActionKind::find_safety: return "find_safety";
    case ActionKind::rest: return "rest";
    case ActionKind::socialize: return "socialize";
    case ActionKind::count: break;
    }
    return "unknown";
}

} // namespace artificial_culture
