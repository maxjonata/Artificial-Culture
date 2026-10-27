# Artificial Culture: architecture and migration

## Purpose

Artificial Culture (AC) models the needs, decisions, learning, memory, and social behavior of non-player characters. In Gala of the Hidden (GotH), AC is intended to provide cognition and social simulation while GotH owns the authoritative game world.

This document describes the active C++20/Flecs direction. The legacy documents elsewhere in `Docs/` describe earlier Rust/Bevy designs and are reference material until individually reviewed and migrated.

## Runtime authority boundary

GotH is authoritative for entity existence, position, navigation, collision, resource inventory and availability, interactions, and the final outcome of actions. AC receives a bounded perception snapshot, updates internal cognitive state, and emits an action intent. It must not assume an intent succeeded. GotH reports acceptance and the actual cognitive effect; AC then updates needs and learning.

This boundary prevents two systems from independently simulating movement or inventory. It also allows AC to run as a module in the same Flecs world or to be moved behind a process boundary later without changing who owns game state. The deployment mode has not yet been selected.

## Active implementation

The current vertical slice is in `include/artificial_culture/cognition.hpp` and `src/cognition.cpp`:

| Type | Role |
| --- | --- |
| `NpcIdentity` | Stable AC agent ID and monotonic action sequence |
| `NeedState` | Normalized hunger, thirst, rest, safety, and social satisfaction |
| `Personality` | Big Five traits used by legacy rumor injection and transmission rules |
| `KnowledgeBase` | Per-agent rumor beliefs and their current strength |
| `PerceptionSnapshot` | Resources and agents that GotH says are visible |
| `ResourceMemory` | Resource IDs and last observed facts retained from prior snapshots |
| `PendingAction` | One outstanding proposal awaiting its result |
| `ActionIntent` | Outbox record for GotH to validate and execute |
| `ActionResult` | Authoritative outcome sent back by GotH |
| `ActionLearning` | Accepted/rejected action counts for later adaptation |
| `RumorEvent` | Injection and transmission telemetry returned to the host |

The decision loop applies the legacy per-second need-decay values and selects the highest-weighted action for which a suitable remembered target exists. `observe` records each resource fact supplied by GotH and does not discover hidden entities. A later snapshot can omit a previously seen resource without erasing the memory. The initial priority weights are thirst 1.0, hunger 0.85, safety 0.75, rest 0.6, and social 0.3. An agent can have only one outstanding action proposal; results must match the dispatched sequence. Need changes are applied only for accepted results.

`CognitionModule` accepts a `flecs::world&`, so the types can be instantiated within GotH's ECS. `ac_demo` is a small local adapter demonstration, not a game-server integration.

Rumor rules preserve the legacy defaults: one injection attempt after 3 seconds, weighted by openness, extraversion, and agreeableness; contact-based transmission with the legacy 0.5 base rate and social-distance factor; 0.8 belief strength on transfer; and 0.01 belief decay per second, removing beliefs below 0.1. GotH reports each confirmed contact with `report_social_contact`; AC owns the resulting belief state and emits attempt/spread records through `take_rumor_events`. The seeded random generator makes runs repeatable when the same seed and event order are supplied.

## Build and dependency

The root CMake project requires CMake 3.24 and C++20. It fetches and pins Flecs v4.1.6 through CMake FetchContent. The project builds `ArtificialCulture::Core` and the `ac_demo` executable. The first configure requires network access to the Flecs repository.

## Migration plan

1. **Cognition foundation (in progress):** needs, perception input, resource memory, decision scoring, intent outbox, and confirmed result handling.
2. **GotH integration:** adapt GotH's Flecs types to AC observations, intents, and confirmed social contacts; connect server validation and action-result feedback.
3. **Personality and memory (partial):** Big Five traits, resource memory, and rumor knowledge are ported; persistence and adaptive learning remain.
4. **Social simulation (partial):** contact-based rumor propagation is ported; trust, reputation, relationship state, and additional interaction rules remain.
5. **Parity and cleanup:** compare migrated behavior against the archived implementation, update the formal GDD and these docs, then remove obsolete migration scaffolding only after review.

The archived Rust tests are behavior references; they are not tests of the new C++ implementation. The Windows build was compiled successfully with CMake 4.4.4, MSVC 19.51, Windows SDK 10.0.26100, and Flecs 4.1.6. The `ac_demo` executable was also run successfully. No C++ test suite has been ported yet.

## License consideration

The repository license is CC BY-NC-SA 4.0. Since GotH may be commercial, resolve whether the port and its source material can be used in that release before distributing derived work. See the root `LICENSE` and consult the rights holder as needed.
