#include "artificial_culture/cognition.hpp"

#include <cstdlib>
#include <iostream>

namespace {

void require(bool condition, const char* message) {
    if (!condition) {
        std::cerr << "Confirmed social feedback failure: " << message << '\n';
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
    cognition.add_agent(3);

    const Testimony warning{8101, 9101, 2, 2, 0, "The west path is dangerous.", 0.8F, 20.0};
    require(cognition.hear_testimony(1, warning), "the listener receives the same initial testimony");
    require(cognition.observe(1, PerceptionSnapshot{{}, {{2, 2.0F}, {3, 2.0F}}}),
            "both possible partners are visible at the same distance");

    cognition.step(0.0F);
    auto intents = cognition.take_intents();
    require(intents.size() == 1 && intents.front().target_id == 2, "stable tie-breaking selects the first partner");
    require(intents.front().event_id == warning.event_id && intents.front().testimony_id == warning.id,
            "the intent carries the event and testimony references to the host");

    ActionResult disputed;
    disputed.agent_id = 1;
    disputed.sequence = intents.front().sequence;
    disputed.accepted = true;
    disputed.social_outcome = SocialOutcomeKind::testimony_disputed;
    disputed.social_episode_id = 8201;
    disputed.social_outcome_time_seconds = 21.0;
    require(cognition.report_result(disputed), "the host's confirmed disagreement resolves the intent");

    const auto memory_after_dispute = cognition.relationship_memory_for(1);
    require(memory_after_dispute && memory_after_dispute->episodes.size() == 1,
            "the outcome becomes one relationship episode for the acting NPC");
    const SocialEpisode& episode = memory_after_dispute->episodes.at(8201);
    require(episode.partner_id == 2 && episode.event_id == warning.event_id && episode.testimony_id == warning.id,
            "the episode links back to the actual partner, event, and testimony");
    require(episode.perceived_kind == SocialEpisodeKind::testimony_disputed,
            "the remembered episode retains the reported social response");
    require(!cognition.report_result(disputed), "a duplicate result cannot duplicate the episode");

    cognition.step(0.0F);
    intents = cognition.take_intents();
    require(intents.size() == 1 && intents.front().target_id == 3,
            "the next sharing proposal reflects the disputed history with partner 2");

    ActionResult impossible_outcome;
    impossible_outcome.agent_id = 1;
    impossible_outcome.sequence = intents.front().sequence;
    impossible_outcome.accepted = false;
    impossible_outcome.social_outcome = SocialOutcomeKind::conversation_rejected;
    impossible_outcome.social_episode_id = 8202;
    impossible_outcome.social_outcome_time_seconds = 22.0;
    require(!cognition.report_result(impossible_outcome),
            "a host-rejected action cannot also claim that a conversation outcome occurred");

    ActionResult rejected_by_host;
    rejected_by_host.agent_id = 1;
    rejected_by_host.sequence = intents.front().sequence;
    rejected_by_host.accepted = false;
    require(cognition.report_result(rejected_by_host), "the host rejection resolves the pending proposal");
    const auto memory_after_rejection = cognition.relationship_memory_for(1);
    require(memory_after_rejection && memory_after_rejection->episodes.size() == 1,
            "an action the host never executed does not become a remembered interaction");

    return EXIT_SUCCESS;
}
