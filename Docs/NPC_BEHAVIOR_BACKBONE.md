# NPC behavioral backbone: shared baseline plus individual variation

**Status:** architecture and evidence register for the next executable slice. The empirical findings below justify relations and experimental contrasts. They do not yet constitute a calibrated universal set of NPC weights. The current code does not implement this backbone.

## Direct answer: was this already documented?

Only in part. [`BEHAVIORAL_HYPERGRAPH.md`](BEHAVIORAL_HYPERGRAPH.md) documented the stress/threat/safety chain and explicitly said its model links were hypotheses. It did **not** specify a shared organism/species baseline plus individual-specific weights, parameter composition, or the ECS component split. This document adds those pieces.

## Parameter layers

Keep five concepts separate. They change at different rates and come from different evidence.

| Layer | Meaning | Example | Update rule |
|---|---|---|---|
| Organism/species baseline | Shared physiological/action defaults for a population the model is intended to represent. | Human need channels, baseline threat response, approximate rest/hunger regulation. | Static profile version. For the current human NPC population there is one human baseline; this is not a cross-species comparison. |
| Individual disposition | Stable differences among members of that population. | Higher/lower trait aggression, reward sensitivity, effort aversion, or social approach propensity, only where the construct has evidence. | Initialized per NPC and held stable unless a deliberate development system changes it. |
| Dynamic state | Temporary state that rises/falls with events and time. | Acute stress, fatigue, hunger, thirst, safety satisfaction. | Updated by explicit state-transition systems from elapsed time and host-confirmed events. |
| Appraisal | Agent-specific interpretation of the current perceived situation. | Threat distance, expected harm, uncertainty, social rejection risk. | Recomputed from private perception, memory, and current state; never copied from authoritative hidden world facts. |
| Culture and relationship history | Learned expectations and partner-specific/socially transmitted information. | Norm expectation, who said what, remembered assistance or rejection. | Changed only by the corresponding experience/learning systems. |

“Species baseline” must not become a place to hide typical stereotypes. If all current NPCs are humans, species-specific neurochemical or physiological differences are out of scope until supported by the design and population-specific evidence. The baseline is a common computational reference; empirical human variation belongs in the individual layer.

## Recommended composition

Use a hierarchical score in which the shared baseline is modified by sparse individual deviations and by explicitly supported situation/state relations. For action `a` and NPC `i`:

```text
score(i, a) = baseline(human, a)
            + individual_offset(i, a)
            + Σ[evidence edge e affecting a]
                (shared_effect(e) + individual_deviation(i, e))
                × conjunction(inputs_e)
            + current_learning(i, a)
```

`conjunction(inputs_e)` represents a **documented multi-input relation**, for example stress × threat distance × individual aggression. It is not an all-to-all multiplication of every fuzzy value. Use the same defined membership/normalization for NPCs and matched scenarios. The exact fuzzy conjunction operator, membership curves, and action-selection temperature are not empirically established yet; keep them configurable and exploratory until scenario calibration.

This additive-on-score formulation is the recommended first representation because it keeps the population baseline interpretable and makes an individual's difference inspectable. Do not multiply raw weights blindly: products can collapse toward zero as dimensions are added, and it becomes hard to tell whether the cause was state, context, or disposition. Do not duplicate a global rule into every NPC's private profile. Store only an individual's deviations where there is evidence or a deliberate test hypothesis.

For an estimated population, the statistical analogue is partial pooling:

```text
β(i, e) = β(human, e) + δ(i, e)
```

Estimate `β(human,e)` and individual variability `δ` from repeated observations with participant-level variation. Do not hand-set a distribution for `δ` and call it empirical. Until repeated behavior data exist, initialize deviations to neutral and introduce controlled contrast profiles for tests. This yields varied NPCs without pretending that a literature coefficient supplies their exact personality.

## First parameter/evidence register

“Empirical value” below means the value reported for that experiment's model or outcome. **None** is an approved direct engine weight. The raw values should remain in their original units and must never be copied into a fuzzy [0,1] field without a documented transformation and a validation set.

| Relation / hyperedge | Empirical result available | What can be carried into the backbone | Engine-weight status |
|---|---|---|---|
| Acute stress × threat distance × trait aggression → approach/avoidance and response speed | 80 adults randomized to stress/control before an approach-avoidance conflict task. Stress increased inhibition for distant threats and sped responses when threats were close; higher trait aggression was associated with more approach, especially under stress. [Vogel & Schwabe (2019)](https://pubmed.ncbi.nlm.nih.gov/30685681/) | Include the **interaction topology and directions**: stress does not have one fixed effect; threat distance and trait aggression moderate it. Treat trait aggression as its own construct. | No coefficient extracted from the abstract. Direction only; do not invent a numeric weight. |
| Acute stress × threat probability × effort cost → effort for safety | In the reported model-predicted subjective value, stress condition had `B=0.68`, 95% CI `[0.05, 1.31]`, `p=.028` for low threat / high effort trials. Other threat/effort combinations did not show evidence of a stress-group difference (`p=.21–.34`). The study used threat-of-shock versus effort, then a separate monetary reward task. [Pavlíčková et al. (2024)](https://doi.org/10.1038/s44271-024-00103-7) | Preserve the **conditionality**: stress can reduce effort-cost aversion for safety in one tested cell, with no general stress multiplier for all effort/reward choices. Keep threat, effort, and goal type as separate inputs. | `B=0.68` is task/model-specific and is **not** the NPC safety coefficient. It can guide sign and scenario contrasts, not direct calibration. |
| Learned social-evaluative threat / social aggression × partner history → social approach/avoidance | 38 adults learned different evaluation histories for virtual peers. Escalating social-evaluative threat and threat of aggression increased avoidance, but individual patterns varied: about 61% increased avoidance, 26% continued approach, and 13% were inconsistent in one task analysis. [Schlund et al. (2021)](https://doi.org/10.1002/jeab.654) | Model threat appraisal as partner-specific and history-dependent. Preserve heterogeneity; do not make all NPCs flee at the same threshold. | Reported partial η² values are repeated-measures task effects, not per-NPC fuzzy slopes. No direct engine weight. |
| Acute hunger × social decision domain → prosocial choice | A multi-study primary paper found null results for social value orientation and volunteering after strong hunger differences (`d=0.04` and `φ=0.02` in one large study, `N=363`), alongside task-specific results elsewhere in the paper. [“Acute hunger does not always undermine prosociality” (2019)](https://doi.org/10.1038/s41467-019-12579-7) | Hunger should strongly motivate food-seeking when food access is represented, but must not automatically suppress helping, sharing, or social contact. Add food scarcity/food relevance as a moderator. | For these two prosocial outcomes, observed estimates are near zero; do not turn them into a universal zero either. The paper supports rejecting a blanket “hungry = selfish” multiplier. |
| Neuromodulator manipulation × uncertainty type → learning/response | A double-blind pharmacological task included 124 analyzed healthy adults and computationally distinguished irreducible, estimation, and volatility uncertainty. [Marshall et al. (2016)](https://doi.org/10.1371/journal.pbio.1002575) | Keep uncertainty types distinct rather than one `uncertainty` scalar. This supports a candidate learning/appraisal decomposition, not a direct neurotransmitter simulation. | Model parameters were fit to a specific reaction-time task; they are not interchangeable with fuzzy NPC weights or neurochemical concentrations. Extract the exact fitted estimates only if using this task as a calibration target. |
| Dopaminergic manipulation × reward prediction error → instrumental learning | L-DOPA and haloperidol changed reward-prediction-error signals during a human instrumental-learning task, and the fitted model estimated a task-specific reinforcement magnitude from striatal BOLD response. [Pessiglione et al. (2006)](https://doi.org/10.1038/nature05051) | The backbone should keep outcome expectation, observed result, and signed prediction error separate. Learning should update action expectations from confirmed outcomes rather than assign “dopamine = pleasure.” | This supports the relation and update mechanism, not a universal numeric learning rate. The paper's fitted reinforcement magnitude is task/imaging-specific. |
| Prolonged serotonergic manipulation × outcome valence → reward/punishment learning | In 66 healthy volunteers, week-long citalopram changed computationally estimated learning asymmetry (more learning from punishment, less from reward); single-dose testing showed no significant modulation. Differences between acute and prolonged effects were not significant. [Michely et al. (2022)](https://doi.org/10.1038/s42003-022-03690-5) | Keep positive and negative outcomes as separate learning channels and state timescale explicit. A rejection or failure may update expectation differently from a reward, but only after the environment confirms the outcome. | Computational learning rates and a pharmacological group contrast are not a game coefficient. Do not represent acute and chronic changes as interchangeable states. |
| Serotonin precursor depletion × unfair offer × self-directed harm → retaliation | In ultimatum-game experiments, tryptophan depletion increased rejection of unfair offers; a later experiment found specificity for unfair treatment directed at the participant rather than third-party norm enforcement. [Crockett et al. (2008)](https://doi.org/10.1126/science.1155577), [Crockett et al. (2013)](https://pubmed.ncbi.nlm.nih.gov/23426678/) | If the game models retaliation, distinguish personal harm from observing harm to someone else, and distinguish punishment from general norm enforcement. | The result is task-, manipulation-, and valence-specific. It does not authorize a universal aggression or morality parameter. |

### Parameter status vocabulary

- `direction_only`: measured result motivates the direction or interaction, but no transferable numerical coefficient has been extracted.
- `task_estimate`: a numerical coefficient/statistic was extracted in the paper's own outcome/model units; not an engine parameter.
- `exploratory_engine_value`: chosen to make a test executable, clearly tagged, swept in sensitivity analysis, and never described as human-calibrated.
- `calibrated_for_scope`: estimated against a stated human dataset/task and held-out comparison; valid only for the tested population and context.

At present, evidence in this document is `direction_only` or `task_estimate`. There are no `calibrated_for_scope` engine weights.

## ECS component layout

Keep data split by lifetime/ownership so Flecs systems can query only what they need. Avoid a single `NPCEverything` component and avoid storing the same shared coefficients on every entity.

| Flecs data | Lifetime / owner | Contents |
|---|---|---|
| `HumanBaselineProfile` on an immutable prefab/config entity | Shared model configuration | Versioned baseline values, allowed evidence-edge IDs, model profile identifier. For the current scope, one human profile. |
| `IndividualDisposition` | Per-NPC, slow-changing AC state | Sparse stable offsets / phenotype dimensions with construct IDs and neutral defaults. Do not copy the whole population table here. |
| `NeedState` | Per-NPC, dynamic AC state | Hunger, thirst, rest, safety, social satisfaction; the existing component. |
| `AffectState` | Per-NPC, dynamic AC state | Add acute stress separately from stable personality; future fatigue can be separate from the rest/sleep need if the model requires both. |
| `PerceptionSnapshot` | Per tick, host supplied | Visible threats, resources, people, distance, uncertainty/evidence quality, and route-cost estimates the host allows the NPC to perceive. |
| `AppraisalState` | Per-NPC, derived AC state | Threat appraisal, expected safety value, social-risk appraisal, and appraisal confidence. Recomputed from permitted perception + memory + current state. |
| `RelationshipMemory`, `KnowledgeBase`, event/testimony memories | Per-NPC, learned AC state | Existing partner/event-specific memories and testimony. Keep perspective private. |
| `ActionIntent` | Per-tick output to host | Action/goal plus target or preferred destination and a traceable reason/edge identifier; no position or movement authority. |
| `ActionResult` / confirmed feedback | Host supplied | Whether the host actually executed, blocked, or rejected the intent and the observed outcome. |

The first systems should have single responsibility: `UpdateNeedsAndAffect`, `AppraisePerception`, `ScoreCandidateGoals`, `EmitActionIntent`, then `ApplyConfirmedOutcome`. The scoring system reads the baseline prefab/profile, individual disposition, current state, appraisal, and memories; it writes only an intent. GotH continues to own pathfinding, movement, collision, world facts, and the authoritative result.

## Minimal initial backbone to implement

1. Keep the shared human profile and the individual offset separate. Default all individual offsets to neutral.
2. Add a dynamic acute-stress component and host-provided threat observations plus route-cost estimates.
3. Implement one small evidence-edge family: stress × threat proximity × individual approach disposition, and stress × threat value × effort cost for safety-seeking.
4. Keep physical danger and social-evaluative threat as distinct appraisal channels; social threat is partner/history specific.
5. Emit `find_safety` plus a host-owned destination/route preference. Record the chosen option, each contributing factor, and whether the host executed it.
6. Exercise matched counterfactuals before integrating with the game. Sweep exploratory membership/combination parameters; report their influence and failures. Do not tune to a single attractive transcript.
7. Later estimate individual deviations from repeated, varied human behavioral data and compare blind human-perception judgments. A human-likeness result validates perception only in the tested conditions, not the biological interpretation.

## Data required before numeric engine calibration

For each included primary study, preserve its exact task, population, randomized manipulation/comparator, repeated measures, outcome scale, raw/model coefficient and interval if reported, missingness/exclusions, and links to related reports from the same sample. Then decide whether outcomes are sufficiently comparable to estimate a shared coefficient. If not, retain task-specific effects as separate edges. The local systematic review remains partial; [`Docs/Research/systematic/SYNTHESIS.md`](Research/systematic/SYNTHESIS.md) explicitly says no mechanism is approved and effect extraction/critical appraisal are unfinished.

Zotero's local API was configured but was not running during this inventory attempt (connection to `127.0.0.1:23119` was refused). The candidate inventory and synthesis in the repository were therefore checked directly; Zotero item keys have not been verified in this pass.

