fn main() {
    let (mut rl, thread) = raylib::init()
        .size(500, 500)
        .title("Pixels")
        .build();
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        
        for y in 0..cell_map.len(){
            for x in 0..cell_map[0].len(){
                cell_map[x][y].draw(&mut d);

            }
        }

        d.draw_fps(0, 0);
    }
}