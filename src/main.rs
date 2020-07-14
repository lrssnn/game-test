use console_engine::pixel;
use console_engine::Color;
use console_engine::KeyCode;
use console_engine::screen::Screen;

use std::collections::HashMap;

use rand::prelude::*;

mod pitcher;

use pitcher::*;
use pitcher::pitch::PitchType;

// Constants - I fear I have gone overboard with short names, but their usage in below makes it very challenging
// STRIKE_WIDTH is so long that the line functions become enormous.
// Buffer size - space between screen edge and strike zone edge - Applied on either side
const BUFF_X: i32 = 8;
const BUFF_Y: i32 = 5;

// Strike Zone Size - float to allow aiming maths to be fuzzy
const STRK_WDTH: f32 = 16.0;
const STRK_HGHT: f32 = 10.0;

// Screen Size - Simply the strike zone with buffer either side
const SCR_X: i32 = STRK_WDTH as i32 + BUFF_X * 2;
const SCR_Y: i32 = STRK_HGHT as i32 + BUFF_Y * 2;

// Strike zone end boundaries - Right/Bottom edges
const STRK_ENDX:i32 = BUFF_X  + STRK_WDTH as i32;
const STRK_ENDY:i32 = BUFF_Y + STRK_HGHT as i32;

fn main() {


    // Initialise the screen - Add 1 to sizes to avoid -1 to all uses of SCR_X/Y below
    let mut engine = console_engine::ConsoleEngine::init(SCR_X as u32 + 1, SCR_Y as u32 + 1, 1);

    loop {
        // Wait for next frame + capture input
        engine.wait_frame();
        engine.clear_screen();

        // Draw the subscreen
        engine.print_screen(0, 0, &draw_strike_zone_screen());

        // Quit on 'q'
        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }

        // Draw
        engine.draw();
    }
}

fn draw_strike_zone_screen() -> Screen {
        let mut strike_screen = Screen::new(SCR_X as u32 + 1, SCR_Y as u32 + 1);

        //Draw The Strike Panel Boundary
        strike_screen.line(0,     0,     SCR_X, 0,     pixel::pxl('='));
        strike_screen.line(0,     SCR_Y, SCR_X, SCR_Y, pixel::pxl('='));
        strike_screen.line(0,     0,     0,     SCR_Y, pixel::pxl('║'));
        strike_screen.line(SCR_X, 0,     SCR_X, SCR_Y, pixel::pxl('║'));
        //Corners
        strike_screen.set_pxl(0,     0,     pixel::pxl('+'));
        strike_screen.set_pxl(0,     SCR_Y, pixel::pxl('+'));
        strike_screen.set_pxl(SCR_X, 0,     pixel::pxl('+'));
        strike_screen.set_pxl(SCR_X, SCR_Y, pixel::pxl('+'));

        //Draw The Strike Zone
        strike_screen.line(BUFF_X,    BUFF_Y,    STRK_ENDX, BUFF_Y,     pixel::pxl('-'));
        strike_screen.line(BUFF_X,    STRK_ENDY, STRK_ENDX, STRK_ENDY,  pixel::pxl('-'));
        strike_screen.line(BUFF_X,    BUFF_Y,    BUFF_X,    STRK_ENDY,  pixel::pxl('|'));
        strike_screen.line(STRK_ENDX, BUFF_Y,    STRK_ENDX, STRK_ENDY,  pixel::pxl('|'));
        //Corners
        strike_screen.set_pxl(BUFF_X,    BUFF_Y,    pixel::pxl('+'));
        strike_screen.set_pxl(BUFF_X,    STRK_ENDY, pixel::pxl('+'));
        strike_screen.set_pxl(STRK_ENDX, BUFF_Y,    pixel::pxl('+'));
        strike_screen.set_pxl(STRK_ENDX, STRK_ENDY, pixel::pxl('+'));

        // Create A pitcher
        let mut pitchbook: HashMap<pitcher::pitch::PitchType, f32> = HashMap::new();
        pitchbook.insert(PitchType::Fastball, 0.7);
        pitchbook.insert(PitchType::Curveball, 0.3);
        let pitcher = build_pitcher(pitchbook, 100.0, 100.0);
        let pitch = pitcher.generate_pitch();

        // Convert pitch locations (0-1) to screenspace 
        let pitch_x: i32 = (pitch.loc_x * SCR_X as f32) as i32;
        let pitch_y: i32 = (pitch.loc_y * SCR_Y as f32) as i32;

        let target_x: i32 = (pitch.aim_x * SCR_X as f32) as i32;
        let target_y: i32 = (pitch.aim_y * SCR_Y as f32) as i32;

        strike_screen.print(0,0, format!("{},{} -> {},{}", pitch.aim_x, pitch.aim_y, target_x, target_y).as_str());
        strike_screen.print(0,1, format!("{},{} -> {},{}", pitch.loc_x, pitch.loc_y, pitch_x, pitch_y).as_str());

        strike_screen.set_pxl(pitch_x, pitch_y, pixel::pxl('o'));
        strike_screen.set_pxl(target_x, target_y, pixel::pxl('x'));

        strike_screen
}
