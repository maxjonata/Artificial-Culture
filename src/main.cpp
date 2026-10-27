#include "artificial_culture/cognition.hpp"

#include <iostream>

int main() {
    using namespace artificial_culture;

    // Local demonstration adapter. In GotH, the authoritative world supplies
    // observations and reports the actual result of the requested action.
    flecs::world world;
    CognitionTuning tuning;
    tuning.rumor_injection_threshold = 1.0F;
    tuning.rumor_base_transmission_rate = 1.0F;
    tuning.rumor_social_distance = 0.0F;
    CognitionModule cognition(world, tuning);

    constexpr AgentId npc_id = 1;
    const Personality outgoing_personality{1.0F, 1.0F, 0.0F, 0.5F, 0.5F};
    cognition.add_agent(npc_id, NeedState{0.25F, 0.82F, 0.74F, 0.85F, 0.55F}, outgoing_personality);
    constexpr AgentId listener_id = 2;
    cognition.add_agent(listener_id, {}, outgoing_personality);
    cognition.observe(npc_id, PerceptionSnapshot{{{101, ResourceKind::food, 4.0F, 1.0F}}, {}});
    cognition.observe(npc_id, PerceptionSnapshot{}); // The known target remains in cognitive memory.
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

    cognition.step(2.9F);
    const std::vector<RumorEvent> injected_events = cognition.take_rumor_events();
    if (injected_events.empty()) {
        std::cerr << "No initial rumor was injected.\n";
        return 1;
    }

    if (!cognition.report_social_contact(npc_id, listener_id)) {
        std::cerr << "The social contact did not match registered NPCs.\n";
        return 1;
    }
    cognition.step(0.1F);
    const std::vector<RumorEvent> social_events = cognition.take_rumor_events();
    const auto listener_knowledge = cognition.knowledge_for(listener_id);
    std::cout << "Rumor contact emitted " << social_events.size() << " event(s); listener knows a rumor: "
              << (listener_knowledge && listener_knowledge->knows_rumor() ? "yes" : "no") << ".\n";
    return 0;
}
