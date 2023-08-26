use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(860, 600).title("Hello, World").build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::RED);
        d.draw_text("Hello, world!", 430, 300, 20, Color::BLACK);
    }
}
