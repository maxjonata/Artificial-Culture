#include "artificial_culture/cognition.hpp"

#include <cstdlib>
#include <iostream>

namespace {

void require(bool condition, const char* message) {
    if (!condition) {
        std::cerr << "Closed-room scenario failed: " << message << '\n';
        std::exit(EXIT_FAILURE);
    }
}

} // namespace

int main() {
    using namespace artificial_culture;

    flecs::world world;
    CognitionModule cognition(world);
    for (AgentId id = 1; id <= 6; ++id) {
        cognition.add_agent(id);
    }

    constexpr WorldEventId event_id = 9101;
    const EventObservation witness_a{
        1101, event_id, PerceivedEventKind::assistance, 1, 2,
        "The guard stepped between the visitor and the crowd.", 0.85F, 30.0};
    const EventObservation witness_b{
        1102, event_id, PerceivedEventKind::conflict, 2, 1,
        "The visitor moved toward the guard before contact.", 0.7F, 30.0};
    const EventObservation quiet_witness{
        1103, event_id, PerceivedEventKind::unknown, 0, 0,
        "A loud exchange occurred near the door.", 0.45F, 30.0};

    require(cognition.observe_event(1, witness_a), "witness A records only its own view");
    require(cognition.observe_event(2, witness_b), "witness B records a different view");
    require(cognition.observe_event(3, quiet_witness), "the quiet witness records the event without sharing it");

    const Testimony from_a{
        5101, event_id, 1, 1, 0,
        "The guard protected the visitor.", 0.8F, 35.0};
    const Testimony from_b{
        5102, event_id, 2, 2, 0,
        "The visitor approached the guard first.", 0.65F, 36.0};
    require(cognition.hear_testimony(4, from_a), "the host delivers A's account to a close contact");
    require(cognition.hear_testimony(5, from_b), "the host delivers B's account along a separate contact route");

    const auto memory_a = cognition.event_memory_for(1);
    const auto memory_b = cognition.event_memory_for(2);
    const auto memory_quiet = cognition.event_memory_for(3);
    const auto memory_friend = cognition.event_memory_for(4);
    const auto memory_rival = cognition.event_memory_for(5);
    const auto memory_absent = cognition.event_memory_for(6);
    require(memory_a && memory_b && memory_quiet && memory_friend && memory_rival && memory_absent,
            "each registered agent has separately queryable event memory");
    require(memory_a->observations.size() == 1 && memory_b->observations.size() == 1 &&
                memory_quiet->observations.size() == 1,
            "only the three present witnesses have firsthand observations");
    require(memory_a->observations.at(1101).event_id == memory_b->observations.at(1102).event_id &&
                memory_a->observations.at(1101).perceived_kind != memory_b->observations.at(1102).perceived_kind,
            "the witnesses retain different perspectives on the same host event");
    require(memory_friend->observations.empty() && memory_rival->observations.empty() &&
                memory_absent->observations.empty(),
            "listeners and an absent agent do not gain firsthand memory");

    const auto testimony_friend = cognition.testimony_memory_for(4);
    const auto testimony_rival = cognition.testimony_memory_for(5);
    const auto testimony_quiet = cognition.testimony_memory_for(3);
    const auto testimony_absent = cognition.testimony_memory_for(6);
    require(testimony_friend && testimony_friend->received.size() == 1 &&
                testimony_friend->received.at(5101).speaker_id == 1,
            "the first contact retains only the testimony actually delivered to them");
    require(testimony_rival && testimony_rival->received.size() == 1 &&
                testimony_rival->received.at(5102).speaker_id == 2,
            "the separate route preserves its own speaker and account");
    require(testimony_quiet && testimony_quiet->received.empty() &&
                testimony_absent && testimony_absent->received.empty(),
            "silence and absence do not create automatic testimony transmission");

    std::cout << "30:00 three people witness event " << event_id << " and preserve separate perspectives.\n"
              << "35:00 witness 1 tells contact 4; 36:00 witness 2 tells contact 5.\n"
              << "The quiet witness and absent agent do not transmit or acquire the accounts.\n";
    return EXIT_SUCCESS;
}
