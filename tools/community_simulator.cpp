#include "artificial_culture/cognition.hpp"

#include <algorithm>
#include <array>
#include <charconv>
#include <cstdint>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <map>
#include <random>
#include <set>
#include <string>
#include <string_view>
#include <vector>

namespace {

using namespace artificial_culture;

constexpr AgentId isolated_agent = 13;
constexpr WorldEventId shared_event = 9001;

struct Options {
    std::uint32_t runs = 16;
    std::uint32_t ticks = 24;
    std::uint32_t seed = 1;
    bool bridge = true;
    bool verbose = false;
    std::string output_path;
};

struct RunStats {
    std::uint32_t executed_interactions = 0;
    std::uint32_t rejected_actions = 0;
    std::uint32_t delivered_testimonies = 0;
    std::uint32_t cross_group_deliveries = 0;
    std::array<std::uint32_t, 2> origin_recipient_count{};
    std::array<bool, 2> origin_crossed_groups{};
};

bool parse_positive(std::string_view text, std::uint32_t& value) {
    const auto parsed = std::from_chars(text.data(), text.data() + text.size(), value);
    return parsed.ec == std::errc{} && parsed.ptr == text.data() + text.size() && value > 0;
}

bool parse_options(int argc, char** argv, Options& options) {
    for (int i = 1; i < argc; ++i) {
        const std::string_view arg(argv[i]);
        if (arg == "--runs" || arg == "--ticks" || arg == "--seed") {
            if (i + 1 >= argc) return false;
            std::uint32_t value = 0;
            if (!parse_positive(argv[++i], value)) return false;
            if (arg == "--runs") options.runs = value;
            else if (arg == "--ticks") options.ticks = value;
            else options.seed = value;
        } else if (arg == "--topology" && i + 1 < argc) {
            const std::string_view topology(argv[++i]);
            if (topology == "bridge") options.bridge = true;
            else if (topology == "separate") options.bridge = false;
            else return false;
        } else if (arg == "--output" && i + 1 < argc) {
            options.output_path = argv[++i];
        } else if (arg == "--verbose") {
            options.verbose = true;
        } else {
            return false;
        }
    }
    return options.runs <= 1000 && options.ticks <= 1000;
}

std::array<std::vector<AgentId>, 14> make_network(bool bridge) {
    std::array<std::vector<AgentId>, 14> neighbors;
    auto connect = [&neighbors](AgentId first, AgentId second) {
        neighbors.at(static_cast<std::size_t>(first)).push_back(second);
        neighbors.at(static_cast<std::size_t>(second)).push_back(first);
    };

    for (AgentId first = 1; first <= 6; ++first) {
        const AgentId second = first == 6 ? 1 : first + 1;
        connect(first, second);
    }
    for (AgentId first = 7; first <= 12; ++first) {
        const AgentId second = first == 12 ? 7 : first + 1;
        connect(first, second);
    }
    if (bridge) connect(6, 7);
    return neighbors;
}

std::uint32_t group_of(AgentId id) {
    return id <= 6 ? 0U : id <= 12 ? 1U : 2U;
}

bool has_edge(const std::array<std::vector<AgentId>, 14>& neighbors, AgentId first, AgentId second) {
    const auto& adjacent = neighbors.at(static_cast<std::size_t>(first));
    return std::find(adjacent.begin(), adjacent.end(), second) != adjacent.end();
}

bool run_community(const Options& options, std::uint32_t run_index, std::ostream& output,
                   RunStats& stats) {
    const std::uint32_t seed = options.seed + run_index;
    std::mt19937 random(seed);
    std::uniform_real_distribution<float> distance(1.0F, 5.0F);
    std::uniform_real_distribution<float> chance(0.0F, 1.0F);
    const auto neighbors = make_network(options.bridge);

    CognitionTuning tuning;
    tuning.random_seed = seed;
    tuning.social_decay_per_second = 0.01F;
    tuning.rumor_injection_threshold = 0.0F;
    tuning.hunger_decay_per_second = 0.0F;
    tuning.thirst_decay_per_second = 0.0F;
    tuning.rest_decay_per_second = 0.0F;
    tuning.safety_decay_per_second = 0.0F;

    flecs::world world;
    CognitionModule cognition(world, tuning);
    for (AgentId id = 1; id <= isolated_agent; ++id) {
        cognition.add_agent(id, NeedState{0.9F, 0.9F, 0.9F, 0.9F, 0.15F});
    }

    const EventObservation account_one{
        1001, shared_event, PerceivedEventKind::assistance, 1, 4,
        "The keeper opened the gate to let the family leave.", 0.75F, 1.0};
    const EventObservation account_two{
        1002, shared_event, PerceivedEventKind::conflict, 8, 4,
        "The keeper blocked the family until the crowd moved back.", 0.70F, 1.0};
    if (!cognition.observe_event(1, account_one) || !cognition.observe_event(8, account_two)) {
        output << "run=" << run_index << " failure=initial-observation\n";
        return false;
    }

    // Seed one account inside each group through a real edge before the simulated rounds.
    const Testimony first_account{2001, shared_event, 1, 1, 0, account_one.perceived_details, 0.75F, 2.0};
    const Testimony second_account{2002, shared_event, 8, 8, 0, account_two.perceived_details, 0.70F, 2.0};
    if (!has_edge(neighbors, 1, 2) || !has_edge(neighbors, 8, 9) ||
        !cognition.hear_testimony(2, first_account) || !cognition.hear_testimony(9, second_account) ||
        !cognition.report_social_contact(1, 2) || !cognition.report_social_contact(8, 9)) {
        output << "run=" << run_index << " failure=initial-testimony\n";
        return false;
    }

    std::map<TestimonyId, Testimony> host_ledger{
        {first_account.id, first_account},
        {second_account.id, second_account},
    };
    TestimonyId next_testimony_id = 3000;
    SocialEpisodeId next_episode_id = 10000;
    std::uint32_t actual_actor_turns = 0;

    for (std::uint32_t tick = 0; tick < options.ticks; ++tick) {
        for (AgentId id = 1; id <= isolated_agent; ++id) {
            std::vector<AgentObservation> visible;
            for (AgentId neighbor : neighbors.at(static_cast<std::size_t>(id))) {
                visible.push_back({neighbor, distance(random)});
            }
            std::shuffle(visible.begin(), visible.end(), random);
            if (!cognition.observe(id, PerceptionSnapshot{{}, std::move(visible)})) {
                output << "run=" << run_index << " failure=invalid-perception npc=" << id << '\n';
                return false;
            }
        }

        cognition.step(1.0F);
        const std::vector<ActionIntent> intents = cognition.take_intents();
        actual_actor_turns += static_cast<std::uint32_t>(intents.size());
        for (const ActionIntent& intent : intents) {
            if (intent.action != ActionKind::socialize && intent.action != ActionKind::share_testimony) {
                output << "run=" << run_index << " failure=unexpected-action npc=" << intent.agent_id << '\n';
                return false;
            }
            if (!has_edge(neighbors, intent.agent_id, intent.target_id)) {
                output << "run=" << run_index << " failure=contact-outside-network npc="
                       << intent.agent_id << " target=" << intent.target_id << '\n';
                return false;
            }

            const bool executed = chance(random) >= 0.10F;
            ActionResult result;
            result.agent_id = intent.agent_id;
            result.sequence = intent.sequence;
            result.accepted = executed;
            result.affected_need = NeedKind::social;
            result.satisfaction_delta = executed ? 0.02F : 0.0F;
            Testimony delivered;
            bool has_delivery = false;

            if (executed) {
                result.social_episode_id = next_episode_id++;
                result.social_outcome_time_seconds = static_cast<double>(tick + 3U);
                if (intent.action == ActionKind::share_testimony) {
                    const auto actor_memory = cognition.testimony_memory_for(intent.agent_id);
                    if (!actor_memory || !actor_memory->received.contains(intent.testimony_id)) {
                        output << "run=" << run_index << " failure=missing-source-testimony npc="
                               << intent.agent_id << '\n';
                        return false;
                    }
                    const Testimony& source = actor_memory->received.at(intent.testimony_id);
                    const auto source_record = host_ledger.find(source.id);
                    if (source_record == host_ledger.end() || source_record->second.event_id != intent.event_id) {
                        output << "run=" << run_index << " failure=broken-host-lineage npc="
                               << intent.agent_id << '\n';
                        return false;
                    }

                    result.social_outcome = chance(random) < 0.7F
                        ? SocialOutcomeKind::testimony_accepted
                        : SocialOutcomeKind::testimony_disputed;
                    delivered = Testimony{
                        next_testimony_id++, source.event_id, intent.agent_id,
                        source.attributed_origin_id, source.id, source.statement,
                        std::clamp(source.confidence * (chance(random) < 0.8F ? 1.0F : 0.8F), 0.0F, 1.0F),
                        static_cast<double>(tick + 3U),
                    };
                    has_delivery = true;
                } else {
                    result.social_outcome = chance(random) < 0.65F
                        ? SocialOutcomeKind::conversation_reciprocated
                        : SocialOutcomeKind::conversation_rejected;
                }
            }

            if (!cognition.report_result(result)) {
                output << "run=" << run_index << " failure=result-rejected npc=" << intent.agent_id << '\n';
                return false;
            }

            if (!executed) {
                ++stats.rejected_actions;
                if (cognition.relationship_memory_for(intent.agent_id)->episodes.contains(result.social_episode_id)) {
                    output << "run=" << run_index << " failure=rejected-action-remembered npc=" << intent.agent_id << '\n';
                    return false;
                }
                continue;
            }

            ++stats.executed_interactions;
            cognition.report_social_contact(intent.agent_id, intent.target_id);
            if (has_delivery) {
                if (!cognition.hear_testimony(intent.target_id, delivered)) {
                    output << "run=" << run_index << " failure=delivery-rejected recipient=" << intent.target_id << '\n';
                    return false;
                }
                host_ledger.emplace(delivered.id, delivered);
                ++stats.delivered_testimonies;
                if (group_of(intent.agent_id) != group_of(intent.target_id)) {
                    ++stats.cross_group_deliveries;
                }
            }

            const auto relationship = cognition.relationship_memory_for(intent.agent_id);
            if (!relationship || !relationship->episodes.contains(result.social_episode_id)) {
                output << "run=" << run_index << " failure=executed-interaction-not-remembered npc="
                       << intent.agent_id << '\n';
                return false;
            }
            if (options.verbose && has_delivery) {
                output << "  tick=" << tick << " NPC " << intent.agent_id << " -> NPC " << intent.target_id
                       << " shared testimony=" << delivered.id << " parent=" << delivered.parent_testimony_id
                       << " origin=" << delivered.attributed_origin_id << '\n';
            }
        }
    }

    const auto isolated_memory = cognition.testimony_memory_for(isolated_agent);
    const auto isolated_events = cognition.event_memory_for(isolated_agent);
    if (!isolated_memory || !isolated_memory->received.empty() || !isolated_events ||
        !isolated_events->observations.empty()) {
        output << "run=" << run_index << " failure=isolated-npc-learned-without-path\n";
        return false;
    }
    if (actual_actor_turns == 0 || stats.delivered_testimonies == 0) {
        output << "run=" << run_index << " failure=no-community-interaction\n";
        return false;
    }

    std::array<std::set<AgentId>, 2> origin_recipients;
    for (AgentId id = 1; id <= 12; ++id) {
        const auto memory = cognition.testimony_memory_for(id);
        if (!memory) return false;
        for (const auto& [testimony_id, testimony] : memory->received) {
            (void)testimony_id;
            const auto parent = testimony.parent_testimony_id == 0
                ? host_ledger.end()
                : host_ledger.find(testimony.parent_testimony_id);
            if (testimony.event_id != shared_event ||
                (testimony.parent_testimony_id != 0 &&
                 (parent == host_ledger.end() || parent->second.event_id != testimony.event_id))) {
                output << "run=" << run_index << " failure=invalid-provenance npc=" << id << '\n';
                return false;
            }
            if (testimony.attributed_origin_id == 1) origin_recipients[0].insert(id);
            if (testimony.attributed_origin_id == 8) origin_recipients[1].insert(id);
        }
    }

    if (!options.bridge) {
        for (AgentId id : origin_recipients[0]) {
            if (group_of(id) != 0) {
                output << "run=" << run_index << " failure=claim-crossed-disconnected-network origin=1 npc=" << id << '\n';
                return false;
            }
        }
        for (AgentId id : origin_recipients[1]) {
            if (group_of(id) != 1) {
                output << "run=" << run_index << " failure=claim-crossed-disconnected-network origin=8 npc=" << id << '\n';
                return false;
            }
        }
        if (stats.cross_group_deliveries != 0) {
            output << "run=" << run_index << " failure=disconnected-network-had-cross-contact\n";
            return false;
        }
    }

    stats.origin_recipient_count = {
        static_cast<std::uint32_t>(origin_recipients[0].size()),
        static_cast<std::uint32_t>(origin_recipients[1].size()),
    };
    for (AgentId id : origin_recipients[0]) stats.origin_crossed_groups[0] |= group_of(id) != 0;
    for (AgentId id : origin_recipients[1]) stats.origin_crossed_groups[1] |= group_of(id) != 1;
    output << "run=" << run_index << " seed=" << seed
           << " topology=" << (options.bridge ? "bridge" : "separate")
           << " actor_turns=" << actual_actor_turns
           << " executed=" << stats.executed_interactions
           << " host_rejections=" << stats.rejected_actions
           << " testimony_deliveries=" << stats.delivered_testimonies
           << " cross_group_deliveries=" << stats.cross_group_deliveries
           << " origin1_recipients=" << origin_recipients[0].size()
           << " origin8_recipients=" << origin_recipients[1].size()
           << " origin1_crossed=" << (stats.origin_crossed_groups[0] ? "yes" : "no")
           << " origin8_crossed=" << (stats.origin_crossed_groups[1] ? "yes" : "no")
           << " isolated=uninformed\n";
    return true;
}

} // namespace

int main(int argc, char** argv) {
    Options options;
    if (!parse_options(argc, argv, options)) {
        std::cerr << "Usage: ac_community_simulator [--runs N] [--ticks N] [--seed N] "
                     "[--topology bridge|separate] [--output FILE] [--verbose]\n";
        return EXIT_FAILURE;
    }

    std::ofstream file;
    std::ostream* output = &std::cout;
    if (!options.output_path.empty()) {
        file.open(options.output_path, std::ios::out | std::ios::trunc);
        if (!file) {
            std::cerr << "Could not open community transcript: " << options.output_path << '\n';
            return EXIT_FAILURE;
        }
        output = &file;
    }

    RunStats aggregate;
    *output << "Artificial Culture standalone community simulation; runs=" << options.runs
            << " ticks=" << options.ticks << " base_seed=" << options.seed
            << " topology=" << (options.bridge ? "bridge" : "separate") << '\n';
    for (std::uint32_t run = 0; run < options.runs; ++run) {
        RunStats stats;
        if (!run_community(options, run, *output, stats)) return EXIT_FAILURE;
        aggregate.executed_interactions += stats.executed_interactions;
        aggregate.rejected_actions += stats.rejected_actions;
        aggregate.delivered_testimonies += stats.delivered_testimonies;
        aggregate.cross_group_deliveries += stats.cross_group_deliveries;
        for (std::size_t origin = 0; origin < 2; ++origin) {
            aggregate.origin_recipient_count[origin] += stats.origin_recipient_count[origin];
            aggregate.origin_crossed_groups[origin] = aggregate.origin_crossed_groups[origin] ||
                stats.origin_crossed_groups[origin];
        }
    }
    if (options.bridge && aggregate.cross_group_deliveries == 0) {
        *output << "failure=bridge-topology-produced-no-cross-group-testimony\n";
        return EXIT_FAILURE;
    }
    *output << "aggregate executed=" << aggregate.executed_interactions
            << " host_rejections=" << aggregate.rejected_actions
            << " testimony_deliveries=" << aggregate.delivered_testimonies
            << " cross_group_deliveries=" << aggregate.cross_group_deliveries
            << " origin1_recipient_instances=" << aggregate.origin_recipient_count[0]
            << " origin8_recipient_instances=" << aggregate.origin_recipient_count[1]
            << " origin1_crossed_at_least_once=" << (aggregate.origin_crossed_groups[0] ? "yes" : "no")
            << " origin8_crossed_at_least_once=" << (aggregate.origin_crossed_groups[1] ? "yes" : "no") << '\n';
    return EXIT_SUCCESS;
}
