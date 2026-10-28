#include "artificial_culture/cognition.hpp"

#include <cstdlib>
#include <iostream>

namespace {

void require(bool condition, const char* message) {
    if (!condition) {
        std::cerr << "Relationship memory failure: " << message << '\n';
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

    const SocialEpisode first_perspective{
        7001, 2, SocialEpisodeKind::promise_broken, 9002, 5003,
        "I asked for help and the promise was not kept.", 21.0};
    const SocialEpisode second_perspective{
        7002, 1, SocialEpisodeKind::promise_kept, 9002, 5003,
        "I returned after handling the emergency and kept the promise.", 21.0};

    require(cognition.remember_social_episode(1, first_perspective), "an NPC records its perspective on an interaction");
    require(cognition.remember_social_episode(2, second_perspective), "the other participant records a separate perspective");

    const auto first_memory = cognition.relationship_memory_for(1);
    const auto second_memory = cognition.relationship_memory_for(2);
    const auto absent_memory = cognition.relationship_memory_for(3);
    require(first_memory && first_memory->episodes.size() == 1, "the first NPC has one remembered episode");
    require(second_memory && second_memory->episodes.size() == 1, "the second NPC has one remembered episode");
    require(absent_memory && absent_memory->episodes.empty(), "an uninvolved NPC receives no relationship memory automatically");
    require(first_memory->episodes.at(7001).partner_id == 2, "the episode is attached to the remembered partner");
    require(first_memory->episodes.at(7001).event_id == second_memory->episodes.at(7002).event_id,
            "different memories can refer to the same host event");
    require(first_memory->episodes.at(7001).perceived_kind != second_memory->episodes.at(7002).perceived_kind,
            "the two people can interpret the interaction differently");
    require(!cognition.remember_social_episode(1, first_perspective), "replaying an episode ID does not duplicate it");

    SocialEpisode self_memory = first_perspective;
    self_memory.id = 7003;
    self_memory.partner_id = 1;
    require(!cognition.remember_social_episode(1, self_memory), "an NPC cannot record itself as its relationship partner");

    return EXIT_SUCCESS;
}
