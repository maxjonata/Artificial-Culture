# GotH host adapter contract

Contract version: `artificial_culture::host_adapter_contract_version == 1`.

Artificial Culture is a cognition module inside the host's Flecs world. Gala of the Hidden (GotH) owns world facts and action execution; AC owns the cognitive entities it creates and the beliefs, needs, memory, and pending intents attached to them.

## Host responsibilities

- Keep the `flecs::world` alive for the lifetime of `CognitionModule`.
- Give each active NPC a stable, nonzero `AgentId`. Keep it stable while that NPC exists.
- Send only facts the NPC is allowed to perceive through `observe`.
- Report social contact only when GotH confirms that the contact happened.
- Call `step` with elapsed simulation seconds.
- Drain each returned intent, validate it against authoritative world state, and execute or reject it.
- Report one `ActionResult` for each dispatched intent, with the same agent ID and sequence number. Report the actual host outcome; do not infer success from the proposal.
- Call `remove_agent` when the NPC leaves the simulation. A late result for a removed agent is rejected.

## Tick flow

1. GotH updates its authoritative world and prepares a perception snapshot for each NPC.
2. GotH calls `observe(agent_id, snapshot)`. `false` means the NPC is unknown or the snapshot contains invalid identity/range values.
3. GotH reports any confirmed NPC-to-NPC contacts with `report_social_contact(first, second)`.
4. GotH calls `step(delta_seconds)` once for the chosen cognition interval.
5. GotH calls `take_intents()`. Each intent has an agent ID and monotonically increasing per-agent sequence. A pending intent is returned once until resolved.
6. GotH validates and executes each intent. AC does not move entities, reserve resources, or decide the final result.
7. GotH reports the confirmed result with `report_result`. A mismatched, duplicate, or late result returns `false` and does not update cognitive state.
8. When an NPC departs, GotH calls `remove_agent`. This deletes the AC-owned Flecs entity and discards queued contacts involving that agent. It does not delete host-owned entities.

## Ownership and limits in version 1

The module creates named Flecs entities `npc_<AgentId>` in the supplied world. Those entity handles and components are AC-owned; host systems should not delete them directly. Other world entities belong to GotH. `remove_agent` is the explicit teardown API. Cultural beliefs and rumor telemetry are not authoritative world events.

Version 1 is a synchronous in-process contract. It does not define networking, save/load, asynchronous actions, event timestamps, schema negotiation beyond the version constant, or thread safety. A host adapter must not call the same module concurrently. These concerns need explicit versions if the integration requires them.

## Local contract check

`ac_adapter_contract_tests` checks dispatch-once behavior, matching and duplicate result handling, entity removal, late-result rejection, and preservation of a host-owned entity. These checks cover the software boundary only; they do not evaluate whether NPC behavior feels human or culturally coherent.
