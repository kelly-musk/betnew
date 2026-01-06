use std::sync::Arc;
use crate::core::models::{Prediction, Event};

#[derive(Debug)]
pub struct ScoringService {
    config: ScoringConfig,
}

#[derive(Debug, Clone)]
pub struct ScoringConfig {
    pub base_points: i32,
    pub confidence_multiplier: f64,
    pub streak_bonus: f64,
    pub early_bird_bonus: i32,
}

impl ScoringService {
    pub fn new(config: ScoringConfig) -> Self {
        Self { config }
    }
    
    pub fn calculate_points(
        &self,
        prediction: &Prediction,
        event: &Event,
        user_streak: i32,
    ) -> i32 {
        let mut points = self.config.base_points;
        
        // Confidence bonus (0-50 points)
        let confidence_bonus = (prediction.confidence as f64 * 0.5) as i32;
        points += confidence_bonus;
        
        // Early bird bonus (if predicted early)
        if self.is_early_prediction(prediction, event) {
            points += self.config.early_bird_bonus;
        }
        
        // Streak multiplier
        if user_streak > 0 {
            let multiplier = 1.0 + (user_streak as f64 * self.config.streak_bonus);
            points = (points as f64 * multiplier) as i32;
        }
        
        points
    }
    
    fn is_early_prediction(&self, prediction: &Prediction, event: &Event) -> bool {
        // Check if prediction was made within first 24 hours of event creation
        let prediction_time = prediction.created_at;
        let event_creation = event.created_at;
        let early_window = chrono::Duration::hours(24);
        
        prediction_time - event_creation < early_window
    }
}