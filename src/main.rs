mod dj;
mod events;
mod fft;

use dj::draw_music;
use events::handle_events;
use raylib::core::audio::{Music, RaylibAudio};
use raylib::core::color::Color;
use raylib::prelude::{RaylibDraw, RaylibDrawHandle};

const WINDOW_NAME: &str = "DJust";
const WINDOW_WIDTH: i32 = 860;
const WINDOW_HEIGHT: i32 = 600;

const FRAMES_PER_SECOND: u32 = 60;

fn draw_screen(d: &mut RaylibDrawHandle, ra: &mut RaylibAudio, music: &mut Option<Music>) {
    if let Some(music) = music.as_mut() {
        ra.update_music_stream(music);
        draw_music(d);
    } else {
        d.draw_text("Drop a music!", 300, 280, 40, Color::WHITE);
    }
}

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
