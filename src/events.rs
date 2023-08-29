use raylib::consts::KeyboardKey::{KEY_R, KEY_SPACE};
use raylib::prelude::{ffi, Music, RaylibAudio, RaylibHandle, RaylibThread};

use super::dj::callback;

const MUSIC_EXTENSIONS: [&str; 2] = [".mp3", ".ogg"];

fn handle_key_events(rl: &mut RaylibHandle, ra: &mut RaylibAudio, music: &mut Option<Music>) {
    // Pause the music
    if rl.is_key_pressed(KEY_SPACE) {
        if let Some(m) = music.as_mut() {
            if ra.is_music_stream_playing(m) {
                ra.pause_music_stream(m);
            } else {
                ra.resume_music_stream(m);
            }
        }
    }
    // Restart the music
    if rl.is_key_pressed(KEY_R) {
        if let Some(m) = music.as_mut() {
            ra.stop_music_stream(m);
            ra.play_music_stream(m);
        }
    }
}

fn handle_file_dropped(
    rl: &mut RaylibHandle,
    ra: &mut RaylibAudio,
    thread: &RaylibThread,
    music: &mut Option<Music>,
) {
    let dropped_files = rl.load_dropped_files();
    if dropped_files.is_empty() {
        return;
    }

    let filename = &dropped_files[0];
    for ext in MUSIC_EXTENSIONS.iter() {
        if !filename.ends_with(ext) {
            continue;
        }
        if music.is_some() {
            ra.stop_music_stream(music.as_mut().unwrap());
        }
        *music = Some(Music::load_music_stream(thread, filename).unwrap());
        ra.play_music_stream(music.as_mut().unwrap());
        break;
    }
    unsafe {
        ffi::AttachAudioStreamProcessor(music.as_mut().unwrap().stream, Some(callback));
    }
}

pub fn handle_events(
    rl: &mut RaylibHandle,
    ra: &mut RaylibAudio,
    thread: &RaylibThread,
    music: &mut Option<Music>,
) {
    handle_key_events(rl, ra, music);
    if rl.is_file_dropped() {
        handle_file_dropped(rl, ra, thread, music);
    }
}
