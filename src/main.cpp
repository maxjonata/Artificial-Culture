#include "artificial_culture/cognition.hpp"

#include <iostream>

int main() {
    using namespace artificial_culture;

    // Local demonstration adapter. In GotH, the authoritative world supplies
    // observations and reports the actual result of the requested action.
    flecs::world world;
    CognitionModule cognition(world);

    constexpr AgentId npc_id = 1;
    cognition.add_agent(npc_id, NeedState{0.25F, 0.82F, 0.74F, 0.85F, 0.55F});
    cognition.observe(npc_id, PerceptionSnapshot{{{101, ResourceKind::food, 4.0F, 1.0F}}, {}});
    cognition.step(0.1F);

    const std::vector<ActionIntent> intents = cognition.take_intents();
    if (intents.empty()) {
        std::cout << "No action is justified by this observation.\n";
        return 0;
    }

    const ActionIntent& intent = intents.front();
    std::cout << "NPC " << intent.agent_id << " proposes " << to_string(intent.action)
              << " for target " << intent.target_id << " (sequence " << intent.sequence << ").\n";

    // This demo adapter stands in for GotH's validation and action execution.
    const ActionResult result{npc_id, intent.sequence, true, NeedKind::hunger, 0.4F};
    if (!cognition.report_result(result)) {
        std::cerr << "The result did not match a pending action.\n";
        return 1;
    }

    const auto updated_needs = cognition.needs_for(npc_id);
    if (updated_needs) {
        std::cout << "GotH-confirmed result updated hunger to " << updated_needs->hunger << ".\n";
    }
    return 0;
}
