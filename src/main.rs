mod dj;
mod events;
mod fft;

use dj::draw_music;
use events::{handle_file_dropped, handle_key_events};
use raylib::core::audio::{Music, RaylibAudio};
use raylib::core::color::Color;
use raylib::prelude::{RaylibDraw, RaylibDrawHandle};

fn draw_screen(d: &mut RaylibDrawHandle, ra: &mut RaylibAudio, music: &mut Option<Music>) {
    if let Some(music) = music.as_mut() {
        ra.update_music_stream(music);
        draw_music(d);
    } else {
        d.draw_text("Drop a music!", 300, 280, 40, Color::WHITE);
    }
}

fn main() {
    let (mut rl, thread) = raylib::init().size(860, 600).title("DJust").build();
    let mut ra: RaylibAudio = RaylibAudio::init_audio_device();
    let mut music = None;
    rl.set_target_fps(60);
    while !rl.window_should_close() {
        handle_key_events(&mut rl, &mut ra, &mut music);
        handle_file_dropped(&mut rl, &mut ra, &thread, &mut music);
        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
        draw_screen(&mut d, &mut ra, &mut music);
    }
}
