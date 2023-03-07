use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(500, 500)
        .title("Pixels")
        .build();
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        d.draw_fps(0, 0);
    }
}