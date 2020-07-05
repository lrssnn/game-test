use console_engine::pixel;
use console_engine::Color;
use console_engine::KeyCode;

fn main() {
    // Initialise screen 20x20 characters, 10 fps
    const WIDTH: u32 = 100;
    const HEIGHT: u32 = 50;

    const WIDTHI: i32 = (WIDTH as i32) -1;
    const HEIGHTI: i32 = (HEIGHT as i32) -1;

    let mut engine = console_engine::ConsoleEngine::init(WIDTH,HEIGHT,30);

    let mut t: f32 = 0.0;
    let inc = 0.05;

    let mut xs = vec![];
    let mut ys = vec![];
    loop {
        // Wait for next frame + capture input
        engine.wait_frame();

        engine.clear_screen();

        //Border Lines
        engine.line(0, 0, WIDTHI, 0, pixel::pxl('-'));
        engine.line(0, HEIGHTI, WIDTHI, HEIGHTI, pixel::pxl('-'));
        engine.line(0, 0, 0, HEIGHTI, pixel::pxl('|'));
        engine.line(WIDTHI, 0, WIDTHI, HEIGHTI, pixel::pxl('|'));

        //Corners
        engine.set_pxl(0,0, pixel::pxl('+'));
        engine.set_pxl(0,HEIGHTI, pixel::pxl('+'));
        engine.set_pxl(WIDTHI,0, pixel::pxl('+'));
        engine.set_pxl(WIDTHI,HEIGHTI, pixel::pxl('+'));

        // Add a new point 
        /* Sine Wave
        let t_i = (t * 5.0).floor() as i32;
        let s_i = (t.sin() * 15.0).floor() as i32;
        let x = t_i;
        let y = (HEIGHTI / 2) + s_i;
        xs.push(x);
        ys.push(y);
        */

        //Spiral
        const CENTREX: i32 = WIDTHI / 2;
        const CENTREY: i32 = HEIGHTI / 2;

        let s_i = (t.sin() * t * 3.0).floor() as i32;
        let c_i = (t.cos() * t * 1.5).floor() as i32;
        let x = CENTREX + s_i;
        let y = CENTREY + c_i;
        xs.push(x);
        ys.push(y);

        for i in 0..xs.len() {
            engine.set_pxl(xs[i],ys[i],pixel::pxl('x'));
        }

        // PRint t
        engine.print(1,HEIGHTI-1, format!("t = {}", t).as_str());

        // Quit on 'q'
        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }

        // Update frame
        t += inc;

        // Draw
        engine.draw();
    }
}
