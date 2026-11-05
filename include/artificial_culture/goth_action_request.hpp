#pragma once

#include "artificial_culture/cognition.hpp"

namespace artificial_culture::goth {

enum class RequestKind { move_to, use_action };

// Local prototype. The destination or interaction target is a world ID.
struct ActionRequest {
    AgentId actor_id;
    std::uint64_t sequence;
    RequestKind kind;
    ActionKind action;
    WorldEntityId target_id;
    TestimonyId testimony_id;
    WorldEventId event_id;
};

[[nodiscard]] inline std::optional<ActionRequest> translate(const ActionIntent& intent) noexcept {
    if (!intent.ready || intent.agent_id == 0 || intent.sequence == 0 || intent.target_id == 0 ||
        intent.action >= ActionKind::count) {
        return std::nullopt;
    }

    const auto kind = intent.action == ActionKind::find_safety ? RequestKind::move_to : RequestKind::use_action;
    return ActionRequest{intent.agent_id, intent.sequence, kind, intent.action, intent.target_id,
                         intent.testimony_id, intent.event_id};
}

} // namespace artificial_culture::goth
