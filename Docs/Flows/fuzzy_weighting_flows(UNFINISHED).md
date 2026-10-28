# Micro-detailed Flows with Fuzzy Weighting and Process Competition

> **Research status: unfinished legacy hypothesis draft; do not use the numeric ranges below as implementation parameters or empirical findings.** The listed weights and mechanisms are not individually sourced or calibrated. For evidence-linked multi-input relations, study boundaries, and matched counterfactuals, use [`../BEHAVIORAL_HYPERGRAPH.md`](../BEHAVIORAL_HYPERGRAPH.md). Fuzzy memberships are model representations, not probabilities or effect sizes.

## 1. Sensory-Cognitive Flow with Dynamic Weighting

### 1.1 Sensory Input and Processing
- **1.1.1 Multimodal Sensory Transduction**
  - Visual stimuli → Retinal processing → Visual pathways (weight: 0.2-0.6, context-dependent)
  - Auditory stimuli → Cochlear processing → Auditory pathways (weight: 0.1-0.5, context-dependent)
  - Interoceptive stimuli → Visceral signals → Homeostatic pathways (weight: 0.1-0.7, need-dependent)
  - **Integration**: Weighted sum with competition for salience (non-linear)

- **1.1.2 Adaptive Attentional Filtering**
  - Bottom-up salience (weight: 0.3-0.8, stimulus-dependent)
  - Top-down relevance (weight: 0.2-0.7, goal-dependent)
  - Contextual novelty (weight: 0.1-0.6, habituation-dependent)
  - **Mechanism**: Neural competition with lateral inhibition and recurrent feedback

- **1.1.3 Probabilistic Perceptual Categorization**
  - Pattern recognition (confidence: 60-95%, experience-dependent)
  - Activation of prior schemas (strength: 0.4-0.9, familiarity-dependent)
  - Predictive filling (rate: 0.2-0.6, ambiguity-dependent)
  - **Result**: Probability distribution over possible interpretations

### 1.2 Parallel Emotional-Motivational Evaluation
- **1.2.1 Primary Emotional Evaluation (Fast Pathway)**
  - Threat detection (amygdala) → Defensive response (weight: 0.3-0.9, threat-dependent)
  - Reward detection (striatum) → Appetitive response (weight: 0.2-0.8, value-dependent)
  - Social detection (superior temporal sulcus) → Affiliative response (weight: 0.1-0.7, context-dependent)
  - **Dynamics**: Resource competition with partial winner-takes-all

- **1.2.2 Homeostatic Motivational Evaluation**
  - Energy state → Food motivation (weight: 0.1-0.9, deprivation-dependent)
  - Hydration state → Thirst motivation (weight: 0.1-0.9, deprivation-dependent)
  - Social state → Affiliative motivation (weight: 0.2-0.8, isolation-dependent)
  - **Integration**: Motivational vector with multiple dimensions competing for priority

- **1.2.3 Contextual Value Modulation**
  - Physical context → Priority adjustment (modifier: 0.7-1.3, environment-dependent)
  - Social context → Normative adjustment (modifier: 0.6-1.4, social presence-dependent)
  - Temporal context → Urgency adjustment (modifier: 0.8-1.5, deadline-dependent)
  - **Mechanism**: Non-linear multipliers applied to the motivational vector

### 1.3 Memory Retrieval and Integration
- **1.3.1 Associative Network Activation**
  - Perceptual similarity → Activation of episodic memories (strength: 0.3-0.8, specificity-dependent)
  - Semantic relevance → Activation of declarative knowledge (strength: 0.2-0.7, expertise-dependent)
  - Emotional congruence → Activation of affective memories (strength: 0.3-0.9, intensity-dependent)
  - **Propagation**: Activation diffusion with exponential decay and resonance reinforcement

- **1.3.2 Bayesian Predictive Integration**
  - Priors based on experience → Initial expectations (weight: 0.3-0.7, confidence-dependent)
  - Current sensory evidence → Belief updating (weight: 0.2-0.8, precision-dependent)
  - Contextual consistency → Narrative coherence (weight: 0.1-0.6, ambiguity-dependent)
  - **Calculation**: Approximate Bayesian inference with stochastic sampling

- **1.3.3 Probabilistic Prospective Simulation**
  - Projection of immediate consequences (precision: 0.5-0.8, familiarity-dependent)
  - Projection of social consequences (precision: 0.4-0.7, theory of mind-dependent)
  - Projection of long-term consequences (precision: 0.3-0.6, time horizon-dependent)
  - **Mechanism**: Mental simulation with multiple parallel scenarios and plausibility weighting

## 2. Decision Processes with Competition and Cooperation

### 2.1 Multidimensional Option Evaluation
- **2.1.1 Subjective Expected Value Calculation**
  - Reward magnitude × Subjective probability (weight: 0.3-0.7, risk-dependent)
  - Energetic/temporal cost (weight: 0.2-0.6, resource-dependent)
  - Social/normative value (weight: 0.1-0.8, social context-dependent)
  - **Integration**: Non-linear weighted sum with context-based adaptive weights

- **2.1.2 Relative Comparison Between Alternatives**
  - Value contrast (sensitivity: 0.6-0.9, magnitude-dependent)
  - Framing effect (bias: 0.1-0.4, presentation-dependent)
  - Anchoring effect (bias: 0.1-0.3, reference-dependent)
  - **Mechanism**: Divisive normalization with lateral inhibition between options

- **2.1.3 Hyperbolic Temporal Modulation**
  - Temporal discounting for future rewards (rate: 0.1-0.8, impulsivity-dependent)
  - Weighting of future risks (rate: 0.2-0.7, anxiety-dependent)
  - Projection of future self-states (precision: 0.3-0.6, self-continuity-dependent)
  - **Function**: Hyperbolic discounting with individualized and contextual parameters

### 2.2 Competition Between Parallel Decision Systems
- **2.2.1 Habitual System (Model-Free)**
  - Learned stimulus-response associations (strength: 0.3-0.9, repetition-dependent)
  - Contextual automaticity (weight: 0.2-0.8, familiarity-dependent)
  - Computational efficiency (advantage: 0.1-0.5, complexity-dependent)
  - **Dynamics**: Fast activation with low cognitive cost

- **2.2.2 Deliberative System (Model-Based)**
  - Simulation of consequences (precision: 0.4-0.8, knowledge-dependent)
  - Evaluation of multiple goals (weight: 0.3-0.7, conflict-dependent)
  - Adaptive flexibility (advantage: 0.2-0.6, novelty-dependent)
  - **Dynamics**: Slow activation with high cognitive cost

- **2.2.3 Emotional-Intuitive System (Heuristic)**
  - Somatic markers (intensity: 0.3-0.8, prior experience-dependent)
  - Moral/social intuitions (strength: 0.2-0.7, socialization-dependent)
  - Fast pattern processing (efficacy: 0.3-0.7, expertise-dependent)
  - **Dynamics**: Intermediate activation with moderate cognitive cost

- **2.2.4 Arbitration Between Systems**
  - Availability of cognitive resources (weight: 0.2-0.7, cognitive load-dependent)
  - Confidence in each system (weight: 0.3-0.8, prior success-dependent)
  - Time pressure (modulator: 0.5-1.5, urgency-dependent)
  - **Mechanism**: Dynamic competition with mutual inhibition and contextual dominance

### 2.3 Conflict Resolution and Decision Making
- **2.3.1 Dynamic Evidence Accumulation**
  - Accumulation rate (drift rate: 0.1-0.5, evidence clarity-dependent)
  - Decision threshold (threshold: 0.5-0.9, caution-dependent)
  - Process noise (variance: 0.1-0.3, neural stability-dependent)
  - **Model**: Diffusion with adaptive boundaries and multiple alternatives

- **2.3.2 Value Conflict Resolution**
  - Hierarchical value prioritization (rigidity: 0.3-0.8, personality-dependent)
  - Internal negotiation between goals (flexibility: 0.2-0.7, creativity-dependent)
  - Suppression of suboptimal alternatives (efficiency: 0.4-0.8, executive control-dependent)
  - **Process**: Attentional competition with feedback and dynamic weight adjustment

- **2.3.3 Commitment to Action**
  - Inhibition of unselected alternatives (strength: 0.5-0.9, decisiveness-dependent)
  - Motor/cognitive preparation (efficiency: 0.4-0.8, practice-dependent)
  - Point of no return (threshold: 0.6-0.9, impulsivity-dependent)
  - **Dynamics**: Catastrophic bifurcation with hysteresis and progressive stabilization

## 3. Execution and Monitoring with Adaptive Feedback

### 3.1 Action Implementation with Continuous Adjustment
- **3.1.1 Intention-Action Translation**
  - Selection of motor/behavioral scheme (precision: 0.6-0.9, familiarity-dependent)
  - Contextual parameterization (adaptability: 0.4-0.8, flexibility-dependent)
  - Temporal sequencing (coordination: 0.5-0.9, complexity-dependent)
  - **Mechanism**: Activation of motor/behavioral programs with contextual modulation

- **3.1.2 Progress Monitoring**
  - Comparison of current vs. desired state (precision: 0.5-0.8, attention-dependent)
  - Detection of significant deviations (sensitivity: 0.4-0.7, criterion-dependent)
  - Prediction of future trajectory (accuracy: 0.3-0.7, experience-dependent)
  - **Process**: Continuous comparison with forward model and prediction error calculation

- **3.1.3 Real-time Adaptive Adjustment**
  - Microbehavioral corrections (granularity: 0.1-0.4, skill-dependent)
  - Strategic changes (magnitude: 0.3-0.7, flexibility-dependent)
  - Abandonment/substitution of plan (threshold: 0.6-0.9, persistence-dependent)
  - **Control**: Negative feedback system with adaptive gains and multiple hierarchical levels

### 3.2 Outcome Evaluation and Learning
- **3.2.1 Multidimensional Success Evaluation**
  - Achievement of primary goal (weight: 0.4-0.8, importance-dependent)
  - Resource efficiency (weight: 0.2-0.6, scarcity-dependent)
  - Secondary consequences (weight: 0.1-0.5, scope-dependent)
  - **Calculation**: Weighted average with dynamic weights based on values and context

- **3.2.2 Feedback Processing**
  - Explicit external feedback (weight: 0.3-0.7, source credibility-dependent)
  - Internal feedback based on expectations (weight: 0.2-0.6, confidence-dependent)
  - Indirect social feedback (weight: 0.1-0.5, social relevance-dependent)
  - **Integration**: Bayesian synthesis with weighting by reliability and relevance

- **3.2.3 Internal Model Updating**
  - Adjustment of expectations (rate: 0.1-0.5, surprise-dependent)
  - Strategy refinement (rate: 0.1-0.4, efficacy-dependent)
  - Revision of values/priorities (rate: 0.05-0.3, emotional impact-dependent)
  - **Mechanism**: Reinforcement learning with adaptive learning rates and multiple parallel systems

## 4. Social Dynamics with Reciprocal Influences

### 4.1 Social Perception and Interpretation
- **4.1.1 Detection of Social Signals**
  - Facial/vocal expressions (sensitivity: 0.5-0.9, social attention-dependent)
  - Body language (sensitivity: 0.4-0.8, nonverbal awareness-dependent)
  - Verbal content (comprehension: 0.6-0.9, linguistic skill-dependent)
  - **Processing**: Multimodal integration with weighting by reliability and salience

- **4.1.2 Inference of Mental States**
  - Attribution of emotions (precision: 0.4-0.8, empathy-dependent)
  - Inference of beliefs/intentions (precision: 0.3-0.7, theory of mind-dependent)
  - Prediction of behavior (precision: 0.3-0.6, familiarity-dependent)
  - **Model**: Mental simulation with partial projection and adjustment based on perceived differences

- **4.1.3 Relationship and Context Evaluation**
  - Previous interaction history (weight: 0.3-0.7, familiarity-dependent)
  - Applicable contextual norms (weight: 0.2-0.6, normative salience-dependent)
  - Power/status dynamics (weight: 0.2-0.5, hierarchy-dependent)
  - **Integration**: Dynamic relational map with multiple dimensions and continuous updating

### 4.2 Regulation of Social Behavior
- **4.2.1 Selection of Interaction Strategy**
  - Personal goals (weight: 0.3-0.8, motivation-dependent)
  - Normative expectations (weight: 0.2-0.7, social pressure-dependent)
  - Maintenance of social image (weight: 0.1-0.6, self-awareness-dependent)
  - **Mechanism**: Multi-objective optimization with normative constraints and adaptive weights

- **4.2.2 Calibration of Social Expression**
  - Emotional regulation (efficacy: 0.4-0.8, self-control-dependent)
  - Impression management (sophistication: 0.3-0.7, social intelligence-dependent)
  - Authenticity vs. conformity (balance: 0.2-0.8, personality-dependent)
  - **Process**: Continuous modulation with social feedback and dynamic adjustment

- **4.2.3 Interaction Coordination**
  - Behavioral synchronization (degree: 0.3-0.7, rapport-dependent)
  - Turn-taking in communication (fluency: 0.4-0.8, conversational skill-dependent)
  - Mutual adjustment of expectations (efficacy: 0.3-0.7, reciprocal empathy-dependent)
  - **Dynamics**: Coupled system with bidirectional influences and emergence of patterns

### 4.3 Social Influence and Contagion
- **4.3.1 Susceptibility to Influence**
  - Normative conformity (degree: 0.2-0.8, need for belonging-dependent)
  - Informational persuasion (degree: 0.3-0.7, uncertainty-dependent)
  - Emotional contagion (degree: 0.2-0.6, empathy-dependent)
  - **Modulators**: Source status, group size, personal certainty, identity relevance

- **4.3.2 Propagation of Behaviors and Beliefs**
  - Direct imitation (probability: 0.2-0.6, visibility-dependent)
  - Adoption of perceived norms (probability: 0.1-0.5, group identification-dependent)
  - Internalization of values (probability: 0.05-0.3, repeated exposure-dependent)
  - **Model**: Network diffusion with adaptive thresholds and heterogeneous influences

- **4.3.3 Consensus Formation and Polarization**
  - Convergence of opinions (rate: 0.1-0.4, openness-dependent)
  - Group polarization (probability: 0.2-0.6, homophily-dependent)
  - Emergence of norms (stability: 0.3-0.7, institutionalization-dependent)
  - **Dynamics**: Complex system with multiple equilibrium points and phase transitions

## 5. Emergence of Identity and Personal Narrative

### 5.1 Construction and Maintenance of the Self
- **5.1.1 Self-concept Integration**
  - Perceived personal attributes (consistency: 0.4-0.8, self-awareness-dependent)
  - Incorporated social feedback (weight: 0.3-0.7, external validation-dependent)
  - Social comparison (influence: 0.2-0.6, social orientation-dependent)
  - **Process**: Continuous synthesis with weighting by centrality and contextual salience

- **5.1.2 Narrative Continuity**
  - Autobiographical memory (coherence: 0.3-0.7, integration-dependent)
  - Reconstruction of experiences (bias: 0.2-0.5, self-esteem-dependent)
  - Projection of future self (vividness: 0.2-0.6, prospective capacity-dependent)
  - **Mechanism**: Dynamic narrative with continuous reconstruction and maintenance of central themes

- **5.1.3 Self-esteem Regulation**
  - Competence evaluation (weight: 0.3-0.7, domain-dependent)
  - Social acceptance evaluation (weight: 0.2-0.6, belonging-dependent)
  - Congruence with values (weight: 0.2-0.5, authenticity-dependent)
  - **Dynamics**: Homeostatic system with compensatory mechanisms and adaptive reference points

### 5.2 Development and Adaptation
- **5.2.1 Long-term Learning**
  - Skill acquisition (rate: 0.1-0.3, practice-dependent)
  - Refinement of mental models (rate: 0.05-0.2, exposure-dependent)
  - Adjustment of life strategies (rate: 0.02-0.1, accumulated feedback-dependent)
  - **Process**: Incremental learning with periodic consolidation and reorganization

- **5.2.2 Adaptation to Contextual Changes**
  - Cognitive flexibility (degree: 0.3-0.7, openness-dependent)
  - Emotional resilience (degree: 0.2-0.6, regulation-dependent)
  - Adjustment of expectations (rate: 0.1-0.4, realism-dependent)
  - **Mechanism**: Dynamic rebalancing with periods of destabilization and reorganization

- **5.2.3 Integration of Significant Experiences**
  - Processing of traumatic events (efficacy: 0.2-0.6, support-dependent)
  - Incorporation of life transitions (completeness: 0.3-0.7, meaning-dependent)
  - Resolution of internal conflicts (degree: 0.2-0.5, self-awareness-dependent)
  - **Dynamics**: Cycles of disintegration and reintegration with search for coherence and meaning

### 5.3 Emergence of Meaning and Purpose
- **5.3.1 Construction of Meaning**
  - Interpretation of experiences (depth: 0.3-0.7, reflexivity-dependent)
  - Connection with cultural narratives (degree: 0.2-0.6, cultural exposure-dependent)
  - Creation of coherence (efficacy: 0.2-0.5, need for structure-dependent)
  - **Process**: Hermeneutic synthesis with multiple levels of integration and contextualization
(Content truncated due to size limit. Use line ranges to read in chunks)
<!-- TODO -->
