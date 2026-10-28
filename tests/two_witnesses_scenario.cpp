#include "artificial_culture/cognition.hpp"

#include <cstdlib>
#include <iostream>

namespace {

void require(bool condition, const char* message) {
    if (!condition) {
        std::cerr << "Two-witness scenario failed: " << message << '\n';
        std::exit(EXIT_FAILURE);
    }
}

} // namespace

int main() {
    using namespace artificial_culture;

    flecs::world world;
    CognitionModule cognition(world);
    cognition.add_agent(1, NeedState{0.8F, 0.8F, 0.8F, 0.8F, 0.8F});
    cognition.add_agent(2, NeedState{0.8F, 0.8F, 0.8F, 0.8F, 0.8F});
    cognition.add_agent(3, NeedState{0.8F, 0.8F, 0.8F, 0.8F, 0.1F});

    const EventObservation witness_a{
        101, 9001, PerceivedEventKind::assistance, 1, 2,
        "The guard stepped between the visitor and the crowd.", 0.8F, 12.0};
    const EventObservation witness_b{
        202, 9001, PerceivedEventKind::conflict, 2, 1,
        "The visitor reached for a weapon before the guard moved.", 0.7F, 12.0};
    require(cognition.observe_event(1, witness_a), "witness A records its view of the event");
    require(cognition.observe_event(2, witness_b), "witness B records a different view of the same event");

    const Testimony original{
        4001, 9001, 1, 1, 0,
        "The guard intervened to protect the visitor.", 0.8F, 13.0};
    require(cognition.hear_testimony(2, original), "witness B hears A's account");

    const Testimony retelling{
        4002, 9001, 2, 1, original.id,
        "Mara says the guard intervened because the visitor was threatened.", 0.6F, 16.0};
    require(cognition.hear_testimony(3, retelling), "a third NPC hears B's changed retelling");

    const auto absent_event_memory = cognition.event_memory_for(3);
    require(absent_event_memory && absent_event_memory->observations.empty(),
            "hearsay does not become a firsthand event observation");
    require(cognition.observe(3, PerceptionSnapshot{{}, {{1, 4.0F}}}), "the third NPC sees witness A");
    cognition.step(0.0F);
    const auto intents = cognition.take_intents();
    require(intents.size() == 1 && intents.front().agent_id == 3, "the third NPC proposes one social action");
    require(intents.front().action == ActionKind::share_testimony && intents.front().target_id == 1 &&
                intents.front().testimony_id == retelling.id,
            "the action is traceable to the account the third NPC received");

    const ActionResult result{3, intents.front().sequence, true, NeedKind::social, 0.1F};
    require(cognition.report_result(result), "the host confirms the social action");
    const Testimony returned{
        4003, 9001, 3, 1, retelling.id,
        "I heard Mara say the guard stepped in to protect someone.", 0.4F, 20.0};
    require(cognition.hear_testimony(1, returned), "the host records what A actually heard from C");
    require(cognition.remember_social_episode(1, SocialEpisode{
                7001, 3, SocialEpisodeKind::testimony_disputed, 9001, returned.id,
                "C repeated the story with less certainty.", 20.0}),
            "A records its own perspective on the conversation");

    const auto memory_a = cognition.event_memory_for(1);
    const auto memory_b = cognition.event_memory_for(2);
    const auto memory_c = cognition.event_memory_for(3);
    const auto testimony_a = cognition.testimony_memory_for(1);
    const auto testimony_c = cognition.testimony_memory_for(3);
    require(memory_a->observations.at(101).perceived_kind != memory_b->observations.at(202).perceived_kind,
            "the two original interpretations remain different");
    require(memory_c->observations.empty(), "C still has no firsthand account after the conversation");
    require(testimony_c->received.at(4002).parent_testimony_id == original.id,
            "the retelling retains its claimed lineage");
    require(testimony_a->received.at(4003).speaker_id == 3,
            "the returned story identifies the person who actually told A");

    std::cout << "12:00 event 9001; A and B record different observations.\n"
              << "13:00 A tells B; 16:00 B retells the account to C.\n"
              << "20:00 C proposes sharing testimony 4002 with A; GotH confirms it.\n"
              << "A receives C's retelling; C still has no firsthand event observation.\n";
    return EXIT_SUCCESS;
}
