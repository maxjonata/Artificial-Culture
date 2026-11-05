#include "artificial_culture/goth_action_request.hpp"

#include <cstdlib>
#include <iostream>

namespace {

void require(bool condition, const char* message) {
    if (!condition) {
        std::cerr << "Adapter contract failure: " << message << '\n';
        std::exit(EXIT_FAILURE);
    }
}

} // namespace

int main() {
    using namespace artificial_culture;

    static_assert(host_adapter_contract_version == 2);

    flecs::world world;
    const flecs::entity host_owned_entity = world.entity("host_owned_entity");
    CognitionModule cognition(world);

    const flecs::entity npc = cognition.add_agent(1, NeedState{0.1F, 0.8F, 0.8F, 0.8F, 0.8F});
    require(cognition.observe(1, PerceptionSnapshot{{{101, ResourceKind::food, 2.0F, 1.0F}}, {}}),
            "a registered NPC accepts a host perception snapshot");

    cognition.step(0.0F);
    const std::vector<ActionIntent> intents = cognition.take_intents();
    require(intents.size() == 1, "one pending host action is exposed as an intent");
    require(intents.front().sequence == 1, "the first intent uses the initial sequence number");
    const auto food_request = goth::translate(intents.front());
    require(food_request && food_request->kind == goth::RequestKind::use_action &&
                food_request->action == ActionKind::find_food && food_request->target_id == 101 &&
                food_request->actor_id == 1 && food_request->sequence == intents.front().sequence,
            "the food intent becomes a correlated request to use the target");
    require(cognition.take_intents().empty(), "an intent is dispatched at most once before a result");

    const ActionResult accepted{1, intents.front().sequence, true, NeedKind::hunger, 0.1F};
    require(cognition.report_result(accepted), "the matching host result resolves a dispatched intent");
    require(!cognition.report_result(accepted), "a duplicate host result is ignored");

    cognition.add_agent(2);
    require(cognition.report_social_contact(1, 2), "a contact between registered NPCs is accepted");
    require(cognition.remove_agent(1), "the host can remove a registered NPC");
    require(!npc.is_alive(), "removing an NPC deletes its cognition entity from Flecs");
    require(!cognition.remove_agent(1), "removing the same NPC twice is harmlessly rejected");
    require(!cognition.report_social_contact(1, 2), "a removed NPC cannot create new social contacts");
    require(!cognition.report_result(accepted), "a late result for a removed NPC is rejected");
    require(cognition.take_intents().empty(), "removal does not leave the NPC's intent in the outbox");
    require(host_owned_entity.is_alive(), "removal does not delete entities owned by the host");

    cognition.add_agent(3, NeedState{1.0F, 1.0F, 1.0F, 0.1F, 1.0F});
    require(cognition.observe(3, PerceptionSnapshot{{{202, ResourceKind::safe_zone, 5.0F, 1.0F}}, {}}),
            "the NPC can observe a safe destination");
    cognition.step(0.0F);
    const auto safety_intents = cognition.take_intents();
    require(safety_intents.size() == 1, "the NPC requests safety");
    const auto move_request = goth::translate(safety_intents.front());
    require(move_request && move_request->kind == goth::RequestKind::move_to && move_request->target_id == 202,
            "the safety intent becomes a movement request toward the observed destination");

    return EXIT_SUCCESS;
}
