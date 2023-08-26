use raylib::prelude::*;

use raylib::core::audio::{Music, RaylibAudio};

const SCREEN_WIDTH: i32 = 860;
const SCREEN_HEIGHT: i32 = 600;
const MUSIC_NIGHTCORE: &str = "songs/light-it-up-nightcore.mp3";

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("DJust")
        .build();
    let mut ra: RaylibAudio = RaylibAudio::init_audio_device();
    let mut music = Music::load_music_stream(&thread, MUSIC_NIGHTCORE).unwrap();
    ra.play_music_stream(&mut music);
    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RED);
        ra.update_music_stream(&mut music);
    }
}
