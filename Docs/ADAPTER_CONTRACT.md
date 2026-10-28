# GotH host adapter contract

Contract version: `artificial_culture::host_adapter_contract_version == 2`.

Artificial Culture is a cognition module inside the host's Flecs world. Gala of the Hidden (GotH) owns world facts and action execution; AC owns the cognitive entities it creates and the beliefs, needs, memory, and pending intents attached to them.

## Host responsibilities

- Keep the `flecs::world` alive for the lifetime of `CognitionModule`.
- Give each active NPC a stable, nonzero `AgentId`. Keep it stable while that NPC exists.
- Send only facts the NPC is allowed to perceive through `observe`.
- Send agent-specific event accounts through `observe_event`; do not broadcast a global omniscient interpretation.
- Report social contact only when GotH confirms that the contact happened.
- Call `step` with elapsed simulation seconds.
- Drain each returned intent, validate it against authoritative world state, and execute or reject it.
- Report one `ActionResult` for each dispatched intent, with the same agent ID and sequence number. Report the actual host outcome; do not infer success from the proposal.
- Call `remove_agent` when the NPC leaves the simulation. A late result for a removed agent is rejected.

## Tick flow

1. GotH updates its authoritative world and prepares a perception snapshot for each NPC.
2. GotH calls `observe(agent_id, snapshot)`. `false` means the NPC is unknown or the snapshot contains invalid identity/range values.
3. GotH calls `observe_event(agent_id, account)` separately for each NPC who perceived an event. Accounts for the same `event_id` can differ. An observation ID is unique within that NPC; replaying it returns `false` rather than duplicating memory. `event_memory_for` exposes that NPC's remembered accounts for inspection.
4. GotH calls `hear_testimony(listener_id, testimony)` for a statement actually heard in an interaction. `speaker_id` is the conversation partner confirmed by the host; `attributed_origin_id` and `parent_testimony_id` preserve what that speaker says about where the claim came from. These attribution fields are reported provenance, not verified truth. Each listener stores its own received versions; a new retelling should get a new testimony ID. `testimony_memory_for` exposes the listener's records for inspection.
5. GotH calls `remember_social_episode(observer_id, episode)` for a remembered interaction from that NPC's perspective. `partner_id` is the other participant and the host supplies event/testimony links when known. The record captures the observer's remembered account; it does not assign a numeric trust score. `relationship_memory_for` exposes episodes for inspection.
6. GotH reports any confirmed NPC-to-NPC contacts with `report_social_contact(first, second)`.
7. GotH calls `step(delta_seconds)` once for the chosen cognition interval.
8. GotH calls `take_intents()`. Each intent has an agent ID and monotonically increasing per-agent sequence. A pending intent is returned once until resolved. A social intent also carries the intended partner; a testimony-sharing intent carries the testimony and event IDs.
9. GotH validates and executes each intent. AC does not move entities, reserve resources, or decide the final result. For `share_testimony`, GotH creates the interaction and supplies the heard statement to the recipient through `hear_testimony`.
10. GotH reports the confirmed result with `report_result`. For an executed social interaction, it may include a unique social episode ID, simulation time, and observed outcome (`conversation_reciprocated`, `conversation_rejected`, `testimony_accepted`, or `testimony_disputed`). AC records that outcome as the acting NPC's perspective and links it to the partner/event/testimony. If the host rejected the action before interaction, it must omit the social outcome. A mismatched, duplicate, late, or internally inconsistent result returns `false` and does not update cognitive state.
11. When an NPC departs, GotH calls `remove_agent`. This deletes the AC-owned Flecs entity and discards queued contacts involving that agent. It does not delete host-owned entities.

## Ownership and limits in version 2

The module creates named Flecs entities `npc_<AgentId>` in the supplied world. Those entity handles and components are AC-owned; host systems should not delete them directly. Other world entities belong to GotH. `remove_agent` is the explicit teardown API. Cultural beliefs and rumor telemetry are not authoritative world events.

Version 2 is a synchronous in-process contract. Event observations carry a host event ID, agent-specific perceived details, confidence and simulation time; AC does not receive or own the authoritative event record. Testimony records identify the actual speaker supplied by the host and separately retain the claimed origin and parent version. AC does not generate dialogue or verify attributions. Social outcome feedback records an actor's memory only when the host confirms an interaction occurred. The contract does not define networking, save/load, asynchronous actions, schema negotiation beyond the version constant, or thread safety. A host adapter must not call the same module concurrently. These concerns need explicit versions if the integration requires them.

## Local contract check

`ac_adapter_contract_tests` checks dispatch-once behavior, matching and duplicate result handling, entity removal, late-result rejection, and preservation of a host-owned entity. `ac_subjective_event_memory_tests` checks separate accounts for a shared event and absence of automatic knowledge. `ac_testimony_provenance_tests` checks per-listener conflicting statements, reported lineage, and separation of immediate speaker from attributed origin. `ac_relationship_memory_tests` checks perspective-specific episodes and partner links. `ac_social_memory_action_tests` checks that testimony changes a proposed social action and is attached to the host intent. `ac_confirmed_social_feedback_tests` checks that an executed dispute changes later partner selection while a host-rejected action creates no memory. `ac_two_witnesses_scenario` runs one end-to-end narrative replay across observation, retelling, action proposal, host confirmation, and a new account. These checks cover software contracts and designed scenarios; they do not evaluate whether NPC behavior feels human or culturally coherent.
