#pragma once

#include <array>
#include <cstdint>
#include <optional>
#include <unordered_map>
#include <vector>

#include <flecs.h>

namespace artificial_culture {

using AgentId = std::uint64_t;
using WorldEntityId = std::uint64_t;

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

// The GotH server supplies only entities and facts this NPC is allowed to perceive.
struct PerceptionSnapshot {
    std::vector<ResourceObservation> visible_resources;
    std::vector<AgentObservation> visible_agents;
};

struct PendingAction {
    bool awaiting_result = false;
    bool dispatched = false;
    std::uint64_t sequence = 0;
    ActionKind action = ActionKind::find_food;
    WorldEntityId target_id = 0;
};

// Flecs component used as the cognitive module's action outbox.
struct ActionIntent {
    bool ready = false;
    AgentId agent_id = 0;
    std::uint64_t sequence = 0;
    ActionKind action = ActionKind::find_food;
    WorldEntityId target_id = 0;
    float utility = 0.0F;
};

struct ActionResult {
    AgentId agent_id = 0;
    std::uint64_t sequence = 0;
    bool accepted = false;
    NeedKind affected_need = NeedKind::count;
    float satisfaction_delta = 0.0F;
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
};

class CognitionModule {
public:
    explicit CognitionModule(flecs::world& world, CognitionTuning tuning = {});

    flecs::entity add_agent(AgentId id, NeedState initial_needs = {});
    bool observe(AgentId id, PerceptionSnapshot perception);
    void step(float delta_seconds);
    std::vector<ActionIntent> take_intents();
    bool report_result(const ActionResult& result);

    std::optional<NeedState> needs_for(AgentId id) const;
    std::optional<ActionLearning> learning_for(AgentId id) const;

private:
    flecs::world& world_;
    CognitionTuning tuning_;
    std::unordered_map<AgentId, flecs::entity> agents_;
    flecs::query<NpcIdentity, NeedState> need_query_;
    flecs::query<NpcIdentity, NeedState, PerceptionSnapshot, PendingAction, ActionIntent> decision_query_;

    void decay_needs(float delta_seconds);
    void choose_actions();
};

const char* to_string(ActionKind action) noexcept;

} // namespace artificial_culture
