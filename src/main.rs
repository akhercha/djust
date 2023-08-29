mod dj;
mod draw;
mod events;
mod fft;

use draw::draw_screen;
use events::handle_events;
use raylib::core::audio::RaylibAudio;
use raylib::prelude::RaylibDrawHandle;

const WINDOW_NAME: &str = "DJust";
const WINDOW_WIDTH: i32 = 860;
const WINDOW_HEIGHT: i32 = 600;

const FRAMES_PER_SECOND: u32 = 60;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .resizable()
        .title(WINDOW_NAME)
        .build();
    let mut ra: RaylibAudio = RaylibAudio::init_audio_device();
    let mut music = None;
    rl.set_target_fps(FRAMES_PER_SECOND);
    while !rl.window_should_close() {
        handle_events(&mut rl, &mut ra, &thread, &mut music);
        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
        draw_screen(&mut d, &mut ra, &mut music);
    }
}
