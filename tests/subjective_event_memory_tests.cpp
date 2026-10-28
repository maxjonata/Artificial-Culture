#include "artificial_culture/cognition.hpp"

#include <cstdlib>
#include <iostream>

namespace {

void require(bool condition, const char* message) {
    if (!condition) {
        std::cerr << "Subjective event memory failure: " << message << '\n';
        std::exit(EXIT_FAILURE);
    }
}

} // namespace

int main() {
    using namespace artificial_culture;

    flecs::world world;
    CognitionModule cognition(world);
    cognition.add_agent(1);
    cognition.add_agent(2);
    cognition.add_agent(3);

    const EventObservation first_account{
        101, 9001, PerceivedEventKind::conflict, 2, 1,
        "The visitor raised a weapon before anyone moved.", 0.7F, 12.0};
    const EventObservation second_account{
        202, 9001, PerceivedEventKind::assistance, 1, 2,
        "The guard stepped between the visitor and the crowd.", 0.9F, 12.0};

    require(cognition.observe_event(1, first_account), "the first witness records an observation");
    require(cognition.observe_event(2, second_account), "the second witness records a separate observation");

    const auto first_memory = cognition.event_memory_for(1);
    const auto second_memory = cognition.event_memory_for(2);
    const auto absent_memory = cognition.event_memory_for(3);
    require(first_memory && first_memory->observations.size() == 1, "the first witness has one private observation");
    require(second_memory && second_memory->observations.size() == 1, "the second witness has one private observation");
    require(absent_memory && absent_memory->observations.empty(), "an absent NPC does not learn the event automatically");

    const EventObservation& first = first_memory->observations.at(101);
    const EventObservation& second = second_memory->observations.at(202);
    require(first.event_id == second.event_id, "different accounts can refer to the same host event");
    require(first.perceived_kind != second.perceived_kind, "the same event can have different perceived meanings");
    require(first.perceived_details != second.perceived_details, "agent-specific details remain distinct");
    require(!cognition.observe_event(1, first_account), "replaying an observation ID does not duplicate it");

    EventObservation invalid = first_account;
    invalid.observation_id = 303;
    invalid.confidence = 1.5F;
    require(!cognition.observe_event(1, invalid), "invalid host confidence values are rejected");
    require(!cognition.event_memory_for(99), "unknown agents have no accessible event memory");

    return EXIT_SUCCESS;
}
