#include "artificial_culture/cognition.hpp"

#include <cstdlib>
#include <iostream>

namespace {

void require(bool condition, const char* message) {
    if (!condition) {
        std::cerr << "Testimony provenance failure: " << message << '\n';
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
    cognition.add_agent(4);

    const Testimony direct{
        5001, 9001, 1, 1, 0,
        "I saw the guard move first.", 0.9F, 13.0};
    require(cognition.hear_testimony(2, direct), "a listener stores a host-confirmed speaker's account");

    const Testimony retold{
        5002, 9001, 2, 1, 5001,
        "The guard started the confrontation, according to Mara.", 0.6F, 14.0};
    require(cognition.hear_testimony(3, retold), "a retelling can link to the testimony version it cites");

    const Testimony contradiction{
        5003, 9001, 1, 1, 0,
        "The visitor moved first; I only stepped between them.", 0.8F, 15.0};
    require(cognition.hear_testimony(3, contradiction), "a listener retains a conflicting account of the event");
    require(!cognition.hear_testimony(3, retold), "replayed testimony IDs are not duplicated");

    const auto listener_memory = cognition.testimony_memory_for(3);
    const auto absent_memory = cognition.testimony_memory_for(4);
    require(listener_memory && listener_memory->received.size() == 2, "the listener preserves both account versions");
    require(absent_memory && absent_memory->received.empty(), "unrelated NPCs do not receive testimony automatically");

    const Testimony& received_retelling = listener_memory->received.at(5002);
    const Testimony& received_conflict = listener_memory->received.at(5003);
    require(received_retelling.speaker_id == 2, "the immediate speaker remains distinguishable from the claimed origin");
    require(received_retelling.attributed_origin_id == 1, "the attributed origin is retained as reported provenance");
    require(received_retelling.parent_testimony_id == direct.id, "the version lineage remains inspectable");
    require(received_retelling.statement != received_conflict.statement, "conflicting accounts are not overwritten into one string");
    require(received_retelling.event_id == received_conflict.event_id, "both accounts remain attached to the same host event ID");

    Testimony unknown_speaker = direct;
    unknown_speaker.id = 5004;
    unknown_speaker.speaker_id = 99;
    require(!cognition.hear_testimony(2, unknown_speaker), "testimony from an unregistered conversation partner is rejected");

    return EXIT_SUCCESS;
}
