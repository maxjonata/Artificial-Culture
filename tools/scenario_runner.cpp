#include "artificial_culture/cognition.hpp"

#include <algorithm>
#include <charconv>
#include <cstdint>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <random>
#include <string_view>
#include <string>
#include <ostream>
#include <vector>

namespace {

bool parse_number(std::string_view text, std::uint32_t& value) {
    const char* begin = text.data();
    const char* end = begin + text.size();
    const auto result = std::from_chars(begin, end, value);
    return result.ec == std::errc{} && result.ptr == end;
}

bool require(bool condition, std::uint32_t run, const char* message) {
    if (condition) {
        return true;
    }
    std::cerr << "Run " << run << " failed: " << message << '\n';
    return false;
}

bool run_scenario(std::uint32_t run, std::uint32_t seed, std::ostream& transcript) {
    using namespace artificial_culture;

    std::mt19937 random(seed);
    std::uniform_int_distribution<int> variant(0, 1);
    std::uniform_real_distribution<float> distance(1.0F, 12.0F);
    const int perspective_variant = variant(random);
    const AgentId preferred_partner = variant(random) == 0 ? 1 : 2;
    const AgentId other_partner = preferred_partner == 1 ? 2 : 1;
    const WorldEventId event_id = 100000U + run;
    const ObservationId observation_a_id = 1000U + run * 10U;
    const ObservationId observation_b_id = observation_a_id + 1U;
    const TestimonyId direct_id = 2000U + run * 10U;
    const TestimonyId retelling_id = direct_id + 1U;
    const TestimonyId returned_id = direct_id + 2U;

    flecs::world world;
    CognitionTuning tuning;
    tuning.random_seed = seed;
    CognitionModule cognition(world, tuning);
    cognition.add_agent(1);
    cognition.add_agent(2);
    cognition.add_agent(3, NeedState{0.8F, 0.8F, 0.8F, 0.8F, 0.1F});
    cognition.add_agent(4); // Present in the world, but outside the event and contact path.

    EventObservation account_a{
        observation_a_id, event_id,
        perspective_variant == 0 ? PerceivedEventKind::assistance : PerceivedEventKind::conflict,
        1, 2,
        perspective_variant == 0 ? "The guard stepped between the visitor and the crowd."
                                 : "The guard moved toward the visitor before anyone called for help.",
        0.7F, 12.0};
    EventObservation account_b{
        observation_b_id, event_id,
        perspective_variant == 0 ? PerceivedEventKind::conflict : PerceivedEventKind::assistance,
        2, 1,
        perspective_variant == 0 ? "The visitor reached toward the guard before the crowd reacted."
                                 : "The visitor backed away when the guard stepped forward.",
        0.8F, 12.0};

    std::vector<EventObservation> observations{account_a, account_b};
    std::shuffle(observations.begin(), observations.end(), random);
    for (const EventObservation& observation : observations) {
        const AgentId observer = observation.observation_id == observation_a_id ? 1 : 2;
        if (!require(cognition.observe_event(observer, observation), run, "witness observation was rejected")) {
            return false;
        }
    }

    const Testimony direct{
        direct_id, event_id, 1, 1, 0,
        account_a.perceived_details, account_a.confidence, 13.0};
    if (!require(cognition.hear_testimony(2, direct), run, "direct testimony was rejected")) {
        return false;
    }
    const Testimony retelling{
        retelling_id, event_id, 2, 1, direct_id,
        perspective_variant == 0 ? "Nara says the guard intervened to protect the visitor."
                                 : "Nara says the guard confronted the visitor before the crowd moved.",
        0.55F, 16.0};
    if (!require(cognition.hear_testimony(3, retelling), run, "retelling was rejected")) {
        return false;
    }

    SocialEpisode positive{
        3000U + run * 10U, preferred_partner, SocialEpisodeKind::promise_kept,
        4000U + run, 0, "This person kept a previous promise.", 18.0};
    SocialEpisode negative{
        3001U + run * 10U, other_partner, SocialEpisodeKind::promise_broken,
        5000U + run, 0, "This person broke a previous promise.", 19.0};
    if (variant(random) == 0) {
        cognition.remember_social_episode(3, positive);
        cognition.remember_social_episode(3, negative);
    } else {
        cognition.remember_social_episode(3, negative);
        cognition.remember_social_episode(3, positive);
    }

    std::vector<AgentObservation> visible{
        {preferred_partner, distance(random)},
        {other_partner, distance(random)},
    };
    std::shuffle(visible.begin(), visible.end(), random);
    if (!require(cognition.observe(3, PerceptionSnapshot{{}, visible}), run, "perception snapshot was rejected")) {
        return false;
    }

    cognition.step(0.0F);
    const std::vector<ActionIntent> intents = cognition.take_intents();
    if (!require(intents.size() == 1, run, "the witness did not propose exactly one social action") ||
        !require(intents.front().action == ActionKind::share_testimony, run, "the action did not reflect testimony memory") ||
        !require(intents.front().target_id == preferred_partner, run, "relationship history did not guide target selection") ||
        !require(intents.front().testimony_id == retelling_id, run, "the intent lost the testimony reference")) {
        return false;
    }

    const ActionResult result{3, intents.front().sequence, true, NeedKind::social, 0.1F};
    if (!require(cognition.report_result(result), run, "the host result did not resolve the social intent")) {
        return false;
    }
    const Testimony returned{
        returned_id, event_id, 3, 1, retelling_id,
        perspective_variant == 0 ? "I heard Nara say the guard intervened to protect someone."
                                 : "I heard Nara say the guard moved toward the visitor first.",
        0.4F, 20.0};
    if (!require(cognition.hear_testimony(preferred_partner, returned), run, "the host-confirmed retelling was rejected")) {
        return false;
    }

    const auto memory_a = cognition.event_memory_for(1);
    const auto memory_b = cognition.event_memory_for(2);
    const auto memory_uninvolved = cognition.event_memory_for(4);
    const auto testimony_uninvolved = cognition.testimony_memory_for(4);
    if (!require(memory_a && memory_b && memory_a->observations.size() == 1 && memory_b->observations.size() == 1,
                 run, "witness perspectives were not retained separately") ||
        !require(memory_a->observations.begin()->second.perceived_kind != memory_b->observations.begin()->second.perceived_kind,
                 run, "the two perspectives converged unexpectedly") ||
        !require(memory_uninvolved && memory_uninvolved->observations.empty() && testimony_uninvolved && testimony_uninvolved->received.empty(),
                 run, "an uninvolved NPC learned information without a contact path")) {
        return false;
    }

    transcript << "Run " << run << " (seed " << seed << ")\n"
              << "  Event " << event_id << ": agent 1 remembers: " << account_a.perceived_details << "\n"
              << "  Event " << event_id << ": agent 2 remembers: " << account_b.perceived_details << "\n"
              << "  Agent 2 hears from agent 1: " << direct.statement << "\n"
              << "  Agent 3 hears from agent 2: " << retelling.statement << "\n"
              << "  Agent 3 proposes sharing testimony " << intents.front().testimony_id
              << " with agent " << intents.front().target_id << ".\n"
              << "  GotH confirms the interaction; the target stores agent 3's retelling.\n"
              << "  Agent 4 has no event observation or testimony.\n"
              << "  Summary: seed=" << seed
              << " perspective_variant=" << perspective_variant
              << " preferred_partner=" << preferred_partner
              << " chose=" << intents.front().target_id
              << " testimony=" << intents.front().testimony_id
              << " uninvolved_remains_uninformed=yes\n\n";
    return true;
}

} // namespace

int main(int argc, char** argv) {
    std::uint32_t runs = 16;
    std::uint32_t base_seed = 1;
    std::string output_path;
    for (int i = 1; i < argc; ++i) {
        const std::string_view argument(argv[i]);
        if ((argument == "--runs" || argument == "--seed") && i + 1 < argc) {
            std::uint32_t value = 0;
            if (!parse_number(argv[++i], value) || value == 0) {
                std::cerr << "Expected a positive integer after " << argument << ".\n";
                return EXIT_FAILURE;
            }
            if (argument == "--runs") {
                runs = value;
            } else {
                base_seed = value;
            }
        } else if (argument == "--output" && i + 1 < argc) {
            output_path = argv[++i];
        } else {
            std::cerr << "Usage: ac_scenario_runner [--runs N] [--seed N] [--output FILE]\n";
            return EXIT_FAILURE;
        }
    }
    if (runs > 10000) {
        std::cerr << "Limit runs to 10000 per invocation.\n";
        return EXIT_FAILURE;
    }

    std::ofstream transcript_file;
    std::ostream* transcript = &std::cout;
    if (!output_path.empty()) {
        transcript_file.open(output_path, std::ios::out | std::ios::trunc);
        if (!transcript_file) {
            std::cerr << "Could not open scenario transcript: " << output_path << '\n';
            return EXIT_FAILURE;
        }
        transcript = &transcript_file;
    }
    *transcript << "Artificial Culture two-witness scenario batch; base_seed=" << base_seed
                << " runs=" << runs << "\n\n";
    for (std::uint32_t run = 0; run < runs; ++run) {
        if (!run_scenario(run + 1, base_seed + run, *transcript)) {
            return EXIT_FAILURE;
        }
    }
    return EXIT_SUCCESS;
}
