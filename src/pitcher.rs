use std::collections::HashMap;
use rand::prelude::*;

#[path = "pitch.rs"] pub mod pitch;
use pitch::*;


pub struct Pitcher {
    pitch_tendencies: HashMap<PitchType, f32>,
    strength: f32,
    accuracy: f32,
}

pub fn build_pitcher(pitch_tendencies: HashMap<PitchType, f32>, strength: f32, accuracy: f32) -> Pitcher {
    Pitcher {
        pitch_tendencies,
        strength,
        accuracy,
    }
}

impl Pitcher {
    pub fn generate_pitch(&self) -> Pitch {
        // Decide on a pitch
        let mut pitchChoice: PitchType = PitchType::None;
        // Generate a float from 0 to 1
        let mut r: f32 = random();
        for (p, prob) in &self.pitch_tendencies {
            if r < *prob {
                pitchChoice = *p;
                break;
            } else {
                r -= prob;
            }
        }

        build_pitch(pitchChoice, 0.5, 0.5, 100.0)
    }
}