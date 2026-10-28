# Flecs migration: implementation status

Updated 27/10/2026. This records shipped software milestones; refer to [`ROADMAP_FLECS.md`](./ROADMAP_FLECS.md) for acceptance criteria and future phases.

## Completed on `master`

- **Stage 0 — build baseline:** Visual Studio 2026 x64 CMake preset, Release build, demo launch. Commits: `2dff85b`, `3fe53d5`.
- **Stage 1 — host adapter foundation:** contract v1, explicit NPC removal, one-time intent dispatch, rejection of duplicate/late results, and preservation of host-owned entities. `ac_adapter_contract_tests` passes. Commit: `92a7e3a`.
- **Stage 2 — subjective event memory foundation:** host can provide unique per-agent observations referring to the same world event, while different perceived details remain separate; NPCs without an observation learn nothing automatically. `ac_subjective_event_memory_tests` passes. Commit: `bb52f95`.
- **Stage 3 — testimony provenance foundation:** each listener can retain a host-confirmed speaker's statement, the source the speaker attributes it to, and the prior testimony version it references. Conflicting accounts remain separate. `ac_testimony_provenance_tests` passes. This stores attribution as reported; it does not verify truth or generate dialogue.
- **Stage 4 — relationship memory foundation:** agents can retain separate interaction episodes linked to a partner and, when known, an event and testimony. The memory records perspective without an arbitrary numeric trust update. `ac_relationship_memory_tests` passes.
- **Stage 5 — social intent foundation:** testimony in memory can change a social proposal from `socialize` to `share_testimony`; remembered positive/negative episodes also influence recipient selection. This uses a simple, uncalibrated balance rule. The host still owns dialogue and outcome. `ac_social_memory_action_tests` and `ac_relationship_influenced_choice_tests` cover the scenario contrast.
- **Stage 7 — scenario runner foundation:** `ac_two_witnesses_scenario` covers one narrative replay; `ac_scenario_runner` now repeats that scenario family with seeded variations and writes readable traces. A 12-run batch is in CTest, and two runs with the same seed produced identical output. The other five scenario families still need implementation.

These tests verify integration and information-boundary contracts. They do not establish that behavior feels human or culturally coherent.

## Still needed before those phases can be considered complete

- Stage 1: validate the contract against an actual GotH adapter; define persistence, entity reload, async actions, threading, and event clock if integration requires them. There is no live GotH integration yet.
- Stage 2: scenario replay and perspective changes over time; event memory is not yet connected to transmission or action choice.
- Stage 3: connect testimony to generated retelling, evidence, correction, and belief revision.
- Stage 4: evidence-based trust, familiarity and expectations derived from episodes, plus persistence/forgetting behavior.
- Stage 5: let relationship history alter recipient/response; connect delivered testimony to new receiver memory and confirmed outcomes.
- Stage 6: local groups and norms with dissent and sanction.
- Stage 7: multi-seed cultural scenario runner and readable per-agent traces.
- Stage 8: controlled player perception study.
- Stage 9: limited GotH pilot, persistence, diagnostics, and release process.

## Working tree warning

Before these implementation commits, 42 Rust files were already staged for deletion and `Docs/Research/` was untracked. These changes are intentionally outside the commits listed above. Do not reset, clean, or commit them without reviewing the user's intended migration/archive state. The archive branch `archive/rust-bevy` should remain preserved.
