use raylib::prelude::*;
use std::os::raw::{c_uint, c_void};
use std::ptr::copy_nonoverlapping;

use raylib::consts::KeyboardKey::KEY_SPACE;
use raylib::core::audio::{Music, RaylibAudio};
use raylib::ffi;

use raylib::core::color::Color;

const MUSIC_NIGHTCORE: &str = "songs/light-it-up-nightcore.mp3";
const MUSIC_BACK_TO_BASICS: &str = "songs/headie-one-back-to-basics.mp3";

const BUFFER_SIZE: usize = 1024;
static mut BUFFER: [u32; BUFFER_SIZE] = [0; BUFFER_SIZE];
static mut GLOBAL_FRAMES_COUNT: usize = 0;

unsafe extern "C" fn callback(buffer_data: *mut c_void, frames: c_uint) {
    let mut frames_usize = frames as usize;
    if frames_usize > BUFFER.len() {
        frames_usize = BUFFER.len();
    }
    copy_nonoverlapping(buffer_data as *const u32, BUFFER.as_mut_ptr(), frames_usize);
    GLOBAL_FRAMES_COUNT = frames_usize;
}

fn main() {
    let (mut rl, thread) = raylib::init().size(860, 600).title("DJust").build();

    let mut ra: RaylibAudio = RaylibAudio::init_audio_device();
    let mut music = Music::load_music_stream(&thread, MUSIC_BACK_TO_BASICS).unwrap();
    ra.play_music_stream(&mut music);
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        // draws
        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::new(12, 12, 13, 255));

            unsafe {
                // Pass the callback directly without taking a reference.
                ffi::AttachAudioStreamProcessor(music.stream, Some(callback));
            }

            let h = d.get_screen_height() as f32;
            let w = d.get_screen_width() as f32;
            unsafe {
                let cell_w: f32 = w / BUFFER.len() as f32;

                for (i, v) in BUFFER.iter().enumerate() {
                    let sample: f32 = *v as f32;
                    if sample > 0.0 {
                        let t = sample / i32::MAX as f32;
                        let v_pos = Vector2 {
                            x: cell_w * i as f32,
                            y: h / 2.0 - h / 2.0 * t,
                        };
                        let v_size = Vector2 {
                            x: cell_w,
                            y: h / 2.0 * t,
                        };
                        d.draw_rectangle_v(v_pos, v_size, Color::new(245, 85, 73, 255));
                    } else {
                        let t: f32 = sample / i32::MIN as f32;
                        let v_pos = Vector2 {
                            x: cell_w * i as f32,
                            y: h / 2.0,
                        };
                        let v_size = Vector2 {
                            x: cell_w,
                            y: h / 2.0 * t,
                        };
                        d.draw_rectangle_v(v_pos, v_size, Color::new(245, 85, 73, 255));
                    }
                }
            }
        }

        // event loop
        {
            if rl.is_key_pressed(KEY_SPACE) {
                if ra.is_music_stream_playing(&music) {
                    ra.pause_music_stream(&mut music);
                } else {
                    ra.resume_music_stream(&mut music);
                }
            }
        }

        // music stream
        {
            ra.update_music_stream(&mut music);
        }
    }
}
