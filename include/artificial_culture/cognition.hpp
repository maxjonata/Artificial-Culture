#pragma once

#include <array>
#include <cstdint>
#include <map>
#include <optional>
#include <random>
#include <string>
#include <vector>

#include <flecs.h>

namespace artificial_culture {

inline constexpr std::uint32_t host_adapter_contract_version = 2;

using AgentId = std::uint64_t;
using WorldEntityId = std::uint64_t;
using WorldEventId = std::uint64_t;
using ObservationId = std::uint64_t;
using TestimonyId = std::uint64_t;
using SocialEpisodeId = std::uint64_t;

enum class NeedKind : std::uint8_t {
    hunger,
    thirst,
    rest,
    safety,
    social,
    count,
};

enum class ResourceKind : std::uint8_t {
    food,
    water,
    rest,
    safe_zone,
};

enum class ActionKind : std::uint8_t {
    find_food,
    find_water,
    find_safety,
    rest,
    socialize,
    share_testimony,
    count,
};

struct NpcIdentity {
    AgentId id = 0;
    std::uint64_t next_action_sequence = 1;
};

// Values are normalized: 0 means unsatisfied and 1 means satisfied.
struct NeedState {
    float hunger = 0.75F;
    float thirst = 0.8F;
    float rest = 0.7F;
    float safety = 0.85F;
    float social = 0.55F;
};

// Big Five values from the legacy Bevy model, normalized to [0, 1].
struct Personality {
    float openness = 0.5F;
    float extraversion = 0.5F;
    float agreeableness = 0.5F;
    float conscientiousness = 0.5F;
    float neuroticism = 0.5F;
};

// Rumor beliefs are private cognitive state owned by Artificial Culture.
struct KnowledgeBase {
    std::map<std::string, float> known_rumors;

    bool knows_rumor() const noexcept { return !known_rumors.empty(); }
};

struct ResourceObservation {
    WorldEntityId id = 0;
    ResourceKind kind = ResourceKind::food;
    float distance = 0.0F;
    float availability = 1.0F;
};

struct AgentObservation {
    AgentId id = 0;
    float distance = 0.0F;
};

enum class PerceivedEventKind : std::uint8_t {
    unknown,
    assistance,
    conflict,
    exchange,
    promise,
    resource_discovery,
};

// This is an agent-specific account supplied by the host, not the authoritative world event.
struct EventObservation {
    ObservationId observation_id = 0;
    WorldEventId event_id = 0;
    PerceivedEventKind perceived_kind = PerceivedEventKind::unknown;
    AgentId perceived_actor = 0;
    AgentId perceived_target = 0;
    std::string perceived_details;
    float confidence = 1.0F;
    double observed_at_seconds = 0.0;
};

struct EventMemory {
    std::map<ObservationId, EventObservation> observations;
};

// A host-confirmed utterance about an event. Attribution and parent refer to
// what the speaker says they heard; speaker_id is the actual conversation partner.
struct Testimony {
    TestimonyId id = 0;
    WorldEventId event_id = 0;
    AgentId speaker_id = 0;
    AgentId attributed_origin_id = 0;
    TestimonyId parent_testimony_id = 0;
    std::string statement;
    float confidence = 0.5F;
    double heard_at_seconds = 0.0;
};

struct TestimonyMemory {
    std::map<TestimonyId, Testimony> received;
};

enum class SocialEpisodeKind : std::uint8_t {
    assistance,
    harm,
    exchange,
    promise_kept,
    promise_broken,
    testimony_confirmed,
    testimony_disputed,
    rejection,
};

struct SocialEpisode {
    SocialEpisodeId id = 0;
    AgentId partner_id = 0;
    SocialEpisodeKind perceived_kind = SocialEpisodeKind::assistance;
    WorldEventId event_id = 0;
    TestimonyId testimony_id = 0;
    std::string perceived_summary;
    double remembered_at_seconds = 0.0;
};

struct RelationshipMemory {
    std::map<SocialEpisodeId, SocialEpisode> episodes;
};

enum class SocialOutcomeKind : std::uint8_t {
    none,
    conversation_reciprocated,
    conversation_rejected,
    testimony_accepted,
    testimony_disputed,
};

// The GotH server supplies only entities and facts this NPC is allowed to perceive.
struct PerceptionSnapshot {
    std::vector<ResourceObservation> visible_resources;
    std::vector<AgentObservation> visible_agents;
};

// Last known resource facts are learned only from GotH-provided observations.
struct ResourceMemory {
    std::map<WorldEntityId, ResourceObservation> known_resources;
};

struct PendingAction {
    bool awaiting_result = false;
    bool dispatched = false;
    std::uint64_t sequence = 0;
    ActionKind action = ActionKind::find_food;
    WorldEntityId target_id = 0;
    TestimonyId testimony_id = 0;
    WorldEventId event_id = 0;
};

// Flecs component used as the cognitive module's action outbox.
struct ActionIntent {
    bool ready = false;
    AgentId agent_id = 0;
    std::uint64_t sequence = 0;
    ActionKind action = ActionKind::find_food;
    WorldEntityId target_id = 0;
    TestimonyId testimony_id = 0;
    WorldEventId event_id = 0;
    float utility = 0.0F;
};

struct ActionResult {
    AgentId agent_id = 0;
    std::uint64_t sequence = 0;
    bool accepted = false;
    NeedKind affected_need = NeedKind::count;
    float satisfaction_delta = 0.0F;
    SocialOutcomeKind social_outcome = SocialOutcomeKind::none;
    SocialEpisodeId social_episode_id = 0;
    double social_outcome_time_seconds = 0.0;
};

struct ActionLearning {
    std::array<std::uint32_t, static_cast<std::size_t>(ActionKind::count)> accepted{};
    std::array<std::uint32_t, static_cast<std::size_t>(ActionKind::count)> rejected{};
};

struct CognitionTuning {
    float activation_threshold = 0.7F;
    float hunger_decay_per_second = 0.008F;
    float thirst_decay_per_second = 0.015F;
    float rest_decay_per_second = 0.005F;
    float safety_decay_per_second = 0.012F;
    float social_decay_per_second = 0.003F;
    float rumor_injection_delay_seconds = 3.0F;
    float rumor_injection_threshold = 0.3F;
    float rumor_social_distance = 100.0F;
    float rumor_base_transmission_rate = 0.5F;
    float rumor_decay_per_second = 0.01F;
    float rumor_transmission_belief_factor = 0.8F;
    float rumor_removal_threshold = 0.1F;
    std::uint32_t random_seed = 1;
};

struct RumorEvent {
    enum class Kind : std::uint8_t { injected, spread_attempt, spread };

    Kind kind = Kind::injected;
    AgentId sender_id = 0;
    AgentId receiver_id = 0;
    std::string content;
    float probability = 0.0F;
    float belief_strength = 0.0F;
    bool succeeded = false;
};

class CognitionModule {
public:
    explicit CognitionModule(flecs::world& world, CognitionTuning tuning = {});

    flecs::entity add_agent(AgentId id, NeedState initial_needs = {}, Personality personality = {});
    bool remove_agent(AgentId id);
    bool observe(AgentId id, PerceptionSnapshot perception);
    bool observe_event(AgentId id, EventObservation observation);
    bool hear_testimony(AgentId listener_id, Testimony testimony);
    bool remember_social_episode(AgentId observer_id, SocialEpisode episode);
    bool report_social_contact(AgentId first, AgentId second);
    void step(float delta_seconds);
    std::vector<ActionIntent> take_intents();
    std::vector<RumorEvent> take_rumor_events();
    bool report_result(const ActionResult& result);

    std::optional<NeedState> needs_for(AgentId id) const;
    std::optional<ActionLearning> learning_for(AgentId id) const;
    std::optional<KnowledgeBase> knowledge_for(AgentId id) const;
    std::optional<EventMemory> event_memory_for(AgentId id) const;
    std::optional<TestimonyMemory> testimony_memory_for(AgentId id) const;
    std::optional<RelationshipMemory> relationship_memory_for(AgentId id) const;

private:
    struct RumorTransmissionAttempt {
        AgentId sender_id;
        AgentId receiver_id;
        std::string content;
        float belief_strength;
        float probability;
    };

    flecs::world& world_;
    CognitionTuning tuning_;
    std::map<AgentId, flecs::entity> agents_;
    flecs::query<NpcIdentity, NeedState> need_query_;
    flecs::query<NpcIdentity, NeedState, PerceptionSnapshot, ResourceMemory, PendingAction, ActionIntent,
                 TestimonyMemory, RelationshipMemory> decision_query_;
    std::mt19937 random_;
    float rumor_injection_elapsed_ = 0.0F;
    bool rumor_injection_finished_ = false;
    std::vector<std::pair<AgentId, AgentId>> social_contacts_;
    std::vector<RumorEvent> rumor_events_;

    void decay_needs(float delta_seconds);
    void choose_actions();
    void process_social_contacts();
    void inject_initial_rumor(float delta_seconds);
    void decay_rumors(float delta_seconds);
    void collect_rumor_transmission_attempts(
        AgentId sender_id,
        AgentId receiver_id,
        std::vector<RumorTransmissionAttempt>& attempts) const;
};

const char* to_string(ActionKind action) noexcept;

} // namespace artificial_culture
