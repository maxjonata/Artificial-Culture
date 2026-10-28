#include "artificial_culture/cognition.hpp"

#include <cstdlib>
#include <iostream>
#include <map>

namespace {

void require(bool condition, const char* message) {
    if (!condition) {
        std::cerr << "Social memory action failure: " << message << '\n';
        std::exit(EXIT_FAILURE);
    }
}

} // namespace

int main() {
    using namespace artificial_culture;

    flecs::world world;
    CognitionModule cognition(world);
    cognition.add_agent(1, NeedState{0.8F, 0.8F, 0.8F, 0.8F, 0.1F});
    cognition.add_agent(2);
    cognition.add_agent(3, NeedState{0.8F, 0.8F, 0.8F, 0.8F, 0.1F});

    const Testimony heard{
        8101, 9101, 2, 2, 0,
        "The east bridge is unsafe after dark.", 0.8F, 20.0};
    require(cognition.hear_testimony(1, heard), "an NPC can receive a testimony before deciding");
    require(cognition.observe(1, PerceptionSnapshot{{}, {{2, 3.0F}}}), "the informed NPC sees a conversation partner");
    require(cognition.observe(3, PerceptionSnapshot{{}, {{2, 3.0F}}}), "the uninformed NPC sees the same partner");

    cognition.step(0.0F);
    std::map<AgentId, ActionIntent> intents;
    for (const ActionIntent& intent : cognition.take_intents()) {
        intents.emplace(intent.agent_id, intent);
    }

    require(intents.size() == 2, "both NPCs produce a social proposal under the same visible conditions");
    require(intents.at(1).action == ActionKind::share_testimony, "stored testimony changes the proposed social action");
    require(intents.at(1).target_id == 2 && intents.at(1).testimony_id == heard.id,
            "the proposal identifies its target and the testimony to share");
    require(intents.at(3).action == ActionKind::socialize && intents.at(3).testimony_id == 0,
            "without testimony the other NPC proposes ordinary socializing");

    const ActionResult shared{1, intents.at(1).sequence, true, NeedKind::social, 0.2F};
    require(cognition.report_result(shared), "the host can confirm the social action through the normal result path");
    require(!cognition.report_result(shared), "a duplicate social result cannot be applied twice");

    return EXIT_SUCCESS;
}
