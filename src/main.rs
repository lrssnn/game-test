use console_engine::pixel;
use console_engine::Color;
use console_engine::KeyCode;

fn main() {
    // Initialise screen 20x20 characters, 10 fps
    let mut engine = console_engine::ConsoleEngine::init(20,20,10);

    let mut frame = 0;
    let mut offset = 0;

    loop {
        // Wait for next frame + capture input
        engine.wait_frame();

        engine.clear_screen();

        // Draw a diagonal line which scans across the screen
        engine.line(0 + offset,0,19 + offset,19,pixel::pxl('#'));

        // Quit on 'q'
        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }

        // Update frame
        frame += 1;
        offset = frame % 20;

        // Draw
        engine.draw();
    }
}
