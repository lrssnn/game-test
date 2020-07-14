use std::collections::HashMap;
use rand::prelude::*;
use rand_distr::{Normal, Distribution};

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
        let mut pitch_choice: PitchType = PitchType::None;
        // Generate a float from 0 to 1
        let mut r: f32 = random();
        for (p, prob) in &self.pitch_tendencies {
            if r < *prob {
                pitch_choice = *p;
                break;
            } else {
                r -= prob;
            }
        }

        // Aim Spot
        // Who knows how this should happen
        let aim_x: f32 = random();
        let aim_y: f32 = random();

        // Aim missing. Normal Dist with mean 0, Std.dev should depend on pitcher quality/stamina etc.
        let normal = Normal::new(0.0, 0.1).unwrap();
        let miss_x: f32 = normal.sample(&mut rand::thread_rng());
        let miss_y: f32 = normal.sample(&mut rand::thread_rng());

        let x = aim_x + miss_x;
        let y = aim_y + miss_y;

        build_pitch(pitch_choice, aim_x, aim_y, x, y, 100.0)
    }
}