use crate::components::components_npc::Personality;

/// Helper function implementing Social Influence Theory for rumor transmission
/// Based on Katz & Lazarsfeld's Two-Step Flow Theory and Social Network Analysis
/// Returns transmission probability based on personality traits and social factors
pub fn calculate_rumor_transmission_probability(
    sender_personality: &Personality,
    receiver_personality: &Personality,
    social_distance: f32,
    base_transmission_rate: f32,
) -> f32 {
    // Social influence decreases with distance (based on Homophily Theory)
    let distance_factor = 1.0 / (1.0 + social_distance * 0.01);

    // Openness affects both transmission and reception
    // High openness = more likely to share and believe new information
    let sender_openness_factor = 0.5 + sender_personality.openness * 0.5;
    let receiver_openness_factor = 0.5 + receiver_personality.openness * 0.5;

    // Combined transmission probability
    base_transmission_rate * distance_factor * sender_openness_factor * receiver_openness_factor
}

/// Helper function to determine rumor spread success based on stochastic social dynamics
/// Based on Threshold Models of Collective Behavior (Granovetter, 1978)
/// Uses random sampling to simulate social uncertainty and individual differences
pub fn should_rumor_spread(transmission_probability: f32) -> bool {
    let random_value: f32 = rand::random();
    random_value < transmission_probability.clamp(0.0, 1.0)
}

/// Helper function to calculate rumor decay over time
/// Based on Information Diffusion Theory - rumors lose credibility over time
pub fn calculate_rumor_decay(current_belief: f32, decay_rate: f32, delta_time: f32) -> f32 {
    (current_belief - decay_rate * delta_time).clamp(0.0, 1.0)
}

/// Helper function to determine if an NPC should inject a new rumor
/// Based on Social Psychology - certain personality types are more likely to start rumors
pub fn should_inject_rumor(personality: &Personality, injection_threshold: f32) -> bool {
    // High openness individuals are more likely to share novel information
    // High extraversion correlates with gossip and social information sharing
    // Low agreeableness may lead to spreading negative rumors
    let injection_probability = (personality.openness * 0.4)
        + (personality.extraversion * 0.3)
        + ((1.0 - personality.agreeableness) * 0.3);

    let random_value: f32 = rand::random();
    random_value < (injection_probability * injection_threshold).clamp(0.0, 1.0)
}
