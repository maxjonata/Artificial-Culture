#include "artificial_culture/cognition.hpp"

#include <algorithm>
#include <cmath>
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

} // namespace

CognitionModule::CognitionModule(flecs::world& world, CognitionTuning tuning)
    : world_(world), tuning_(tuning) {
    world_.component<NpcIdentity>();
    world_.component<NeedState>();
    world_.component<PerceptionSnapshot>();
    world_.component<PendingAction>();
    world_.component<ActionIntent>();
    world_.component<ActionLearning>();

    need_query_ = world_.query<NpcIdentity, NeedState>();
    decision_query_ = world_.query<NpcIdentity, NeedState, PerceptionSnapshot, PendingAction, ActionIntent>();
}

flecs::entity CognitionModule::add_agent(AgentId id, NeedState initial_needs) {
    if (id == 0) {
        throw std::invalid_argument("Agent IDs must be non-zero");
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
        .set<ActionLearning>({});
    agents_.emplace(id, agent);
    return agent;
}

bool CognitionModule::observe(AgentId id, PerceptionSnapshot perception) {
    const auto found = agents_.find(id);
    if (found == agents_.end() || !valid_perception(perception)) {
        return false;
    }
    found->second.set<PerceptionSnapshot>(std::move(perception));
    return true;
}

void CognitionModule::step(float delta_seconds) {
    if (!std::isfinite(delta_seconds) || delta_seconds < 0.0F) {
        throw std::invalid_argument("delta_seconds must be finite and non-negative");
    }
    decay_needs(delta_seconds);
    choose_actions();
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
                                PendingAction& pending, ActionIntent& outbox) {
        if (pending.awaiting_result || outbox.ready) {
            return;
        }

        std::optional<Candidate> best;
        const auto consider_resource = [&](ResourceKind kind, NeedKind need, ActionKind action, float priority) {
            const float satisfaction = need_value(needs, need);
            if (satisfaction >= tuning_.activation_threshold) {
                return;
            }
            for (const ResourceObservation& resource : perception.visible_resources) {
                if (resource.kind != kind || resource.availability <= 0.0F) {
                    continue;
                }
                const float utility = need_utility(satisfaction, tuning_.activation_threshold, priority);
                if (!best || utility > best->utility) {
                    best = Candidate{action, need, resource.id, utility};
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
            const AgentObservation& target = perception.visible_agents.front();
            const float utility = need_utility(social_satisfaction, tuning_.activation_threshold, 0.3F);
            if (!best || utility > best->utility) {
                best = Candidate{ActionKind::socialize, NeedKind::social, target.id, utility};
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
        auto* pending = agent.get_mut<PendingAction>();
        const auto* outbox = agent.get<ActionIntent>();
        if (!pending || !outbox || !outbox->ready || pending->dispatched) {
            continue;
        }
        intents.push_back(*outbox);
        pending->dispatched = true;
    }
    return intents;
}

bool CognitionModule::report_result(const ActionResult& result) {
    const auto found = agents_.find(result.agent_id);
    if (found == agents_.end() || !std::isfinite(result.satisfaction_delta)) {
        return false;
    }

    const auto* pending = found->second.get<PendingAction>();
    if (!pending || !pending->awaiting_result || !pending->dispatched || pending->sequence != result.sequence) {
        return false;
    }

    const ActionKind action = pending->action;
    if (auto* learning = found->second.get_mut<ActionLearning>()) {
        auto& counts = result.accepted ? learning->accepted : learning->rejected;
        ++counts[static_cast<std::size_t>(action)];
    }

    if (result.accepted && result.affected_need != NeedKind::count) {
        if (auto* needs = found->second.get_mut<NeedState>()) {
            float& value = need_value(*needs, result.affected_need);
            value = std::clamp(value + result.satisfaction_delta, 0.0F, 1.0F);
        }
    }

    if (auto* mutable_pending = found->second.get_mut<PendingAction>()) {
        mutable_pending->awaiting_result = false;
        mutable_pending->dispatched = false;
        mutable_pending->sequence = 0;
        mutable_pending->target_id = 0;
    }
    if (auto* outbox = found->second.get_mut<ActionIntent>()) {
        outbox->ready = false;
    }
    return true;
}

std::optional<NeedState> CognitionModule::needs_for(AgentId id) const {
    const auto found = agents_.find(id);
    if (found == agents_.end()) {
        return std::nullopt;
    }
    const NeedState* needs = found->second.get<NeedState>();
    return needs ? std::optional<NeedState>(*needs) : std::nullopt;
}

std::optional<ActionLearning> CognitionModule::learning_for(AgentId id) const {
    const auto found = agents_.find(id);
    if (found == agents_.end()) {
        return std::nullopt;
    }
    const ActionLearning* learning = found->second.get<ActionLearning>();
    return learning ? std::optional<ActionLearning>(*learning) : std::nullopt;
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
