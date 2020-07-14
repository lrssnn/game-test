#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PitchType {Fastball, Curveball, None}

#[derive(Debug, Copy, Clone)]
pub struct Pitch {
    pub pitchType: PitchType,
    pub aim_x: f32,
    pub aim_y: f32,
    pub loc_x: f32,
    pub loc_y: f32,
    pub speed: f32,
}

pub fn build_pitch(pitchType: PitchType, aim_x: f32, aim_y: f32, loc_x: f32, loc_y: f32, speed: f32) -> Pitch {
    Pitch {
        pitchType,
        aim_x,
        aim_y,
        loc_x,
        loc_y,
        speed,
    }
}