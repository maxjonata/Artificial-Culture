#include "artificial_culture/cognition.hpp"

#include <cstdlib>
#include <iostream>

namespace {

void require(bool condition, const char* message) {
    if (!condition) {
        std::cerr << "Relationship-influenced choice failure: " << message << '\n';
        std::exit(EXIT_FAILURE);
    }
}

artificial_culture::ActionIntent run_choice(bool partner_two_has_positive_history) {
    using namespace artificial_culture;

    flecs::world world;
    CognitionModule cognition(world);
    cognition.add_agent(1, NeedState{0.8F, 0.8F, 0.8F, 0.8F, 0.1F});
    cognition.add_agent(2);
    cognition.add_agent(3);
    require(cognition.hear_testimony(1, Testimony{
                8101, 9101, 2, 2, 0, "A warning about the old bridge.", 0.8F, 20.0}),
            "the NPC starts with the same testimony in both scenario variants");
    require(cognition.observe(1, PerceptionSnapshot{{}, {{3, 2.0F}, {2, 2.0F}}}),
            "the same two potential conversation partners are visible");

    const AgentId positive_partner = partner_two_has_positive_history ? 2 : 3;
    const AgentId negative_partner = partner_two_has_positive_history ? 3 : 2;
    require(cognition.remember_social_episode(1, SocialEpisode{
                8201, positive_partner, SocialEpisodeKind::promise_kept, 9201, 0,
                "This person kept a prior promise.", 18.0}),
            "the NPC remembers positive history with one partner");
    require(cognition.remember_social_episode(1, SocialEpisode{
                8202, negative_partner, SocialEpisodeKind::promise_broken, 9202, 0,
                "This person broke a prior promise.", 19.0}),
            "the NPC remembers negative history with the other partner");

    cognition.step(0.0F);
    const auto intents = cognition.take_intents();
    require(intents.size() == 1, "the NPC proposes a single social action");
    return intents.front();
}

} // namespace

int main() {
    const auto prefers_two = run_choice(true);
    const auto prefers_three = run_choice(false);

    require(prefers_two.action == artificial_culture::ActionKind::share_testimony &&
                prefers_three.action == artificial_culture::ActionKind::share_testimony,
            "the same remembered claim produces a sharing proposal in both variants");
    require(prefers_two.testimony_id == prefers_three.testimony_id,
            "the scenario changes relationship history only, not the claim being shared");
    require(prefers_two.target_id == 2 && prefers_three.target_id == 3,
            "reversing the remembered relationship history reverses the chosen conversation partner");

    return EXIT_SUCCESS;
}
