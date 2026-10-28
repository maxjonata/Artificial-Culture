# Cultural scenario validation

## Purpose

Artificial Culture is intended to create a world that each NPC experiences from a limited, socially situated point of view. Validation therefore follows agents through events and asks what each came to believe, why, from whom, and how that belief changed their later choices. Unit tests for equations and data bounds are useful engineering checks, but they do not establish cultural realism.

Scenarios are qualitative acceptance tests. They should record event timelines, each agent's observations, relationships, remembered testimony, interpretation, belief changes, and resulting choices. A scenario should be replayable from a seed, while evaluation should also vary seeds and small changes in timing or contact order. The goal is coherent variation across runs, not one scripted “correct” outcome.

## Scenario suite

### 1. The closed-room account

An event occurs in a room witnessed by a few NPCs. One witness later tells a close friend; another tells a rival; a third never speaks about it. A distant NPC is not present and has no contact with the witnesses.

**Acceptance:** knowledge follows plausible observation and contact routes. The uninformed NPC does not learn the event by magic. The accounts retain who said what, and the listener's relationship to the speaker affects how they receive it. Different listeners can end with different confidence without needing different world facts.

### 2. Two witnesses, one event

Two NPCs see the same ambiguous event from different positions and with different prior expectations. They describe it differently, then each account travels through overlapping social circles.

**Acceptance:** the simulation distinguishes the shared event from each witness's interpretation. Both interpretations can persist in the community; transmission does not overwrite them into one globally shared string. Later NPCs can know the source chain and hold conflicting accounts.

### 3. A story changes as it travels

A consequential claim passes through several people. One reteller omits the original source, another adds an interpretation, and a third challenges it with firsthand evidence.

**Acceptance:** provenance loss, retelling, and correction are represented as changes in testimony or interpretation. NPCs update according to their own experiences and relationships; the simulation does not treat every contact as a fresh identical copy.

### 4. Reputation outlives a single conversation

An NPC has a history of accurate warnings with one group and a history of self-serving claims with another. The NPC tells both groups the same uncertain story.

**Acceptance:** trust is relational and grounded in remembered interactions. The two audiences can respond differently, and later outcomes can strengthen or weaken that trust. A fixed personality trait alone cannot stand in for a relationship.

### 5. A norm becomes visible through behavior

A practice is followed by most members of a group, questioned by a few, and rejected by an outsider. NPCs observe approval, sanctions, and consequences over time.

**Acceptance:** a norm can emerge or weaken through social learning and enforcement. The norm influences later choices, while dissent and subgroup differences remain possible. A global “culture” flag applied identically to every NPC does not pass.

### 6. The same event means different things to different groups

Two groups with different histories witness a public event. Their members remember different details as important, tell different stories, and make different choices afterward.

**Acceptance:** group membership and collective memory shape attention and meaning without forcing uniform belief inside either group. The event remains anchored in the authoritative world, while its significance is subjective.

## Evaluation practice

- Keep the external event timeline fixed while varying NPC perspective, relationships, and contact paths; then compare the resulting social histories.
- Run multiple seeds and report distributions and representative transcripts. A single lucky random outcome is not evidence of realism.
- Ask reviewers familiar with the setting to judge whether each trajectory is intelligible from that NPC's knowledge and history, not whether every NPC reaches a preferred conclusion.
- Include counterfactual replays: remove a conversation, change a relationship history, or give an NPC firsthand evidence. The resulting belief and action should change for an understandable reason.
- Preserve surprising but coherent outcomes. Do not encode one “right” cultural interpretation as a universal expected answer.

## Current coverage

The C++ vertical slice can represent resource observations, action proposals, confirmed outcomes, text-keyed rumor beliefs, and contact-driven transmission. It cannot currently represent source attribution, individual interpretations of shared events, pairwise trust or relationships, groups and norms, evidence-based revision, or culture affecting decision-making. Thus scenarios 1–6 are design acceptance criteria, not passing C++ tests yet. The next implementation work should establish event/provenance and relationship-aware belief state before writing executable tests that claim to evaluate cultural subjectivity.
