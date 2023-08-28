use raylib::consts::KeyboardKey::KEY_SPACE;
use raylib::core::audio::{Music, RaylibAudio};
use raylib::core::color::Color;
use raylib::prelude::*;
use std::os::raw::c_void;

// Path to your music
// const MUSIC: &str = "songs/dreams.ogg";
// const MUSIC: &str = "songs/crankdat-higher.ogg";
const MUSIC: &str = "songs/headie-one-back-to-basics.ogg";
const COLOR_PALE_RED: Color = Color::new(245, 85, 73, 255);

#[derive(Clone, Copy)]
struct Frame {
    left: f32,
    right: f32,
}

const BUFFER_CAPACITY: usize = 4800;
static mut GLOBAL_FRAMES: [Frame; BUFFER_CAPACITY] = [Frame {
    left: 0.0,
    right: 0.0,
}; BUFFER_CAPACITY];
static mut GLOBAL_FRAMES_COUNT: usize = 0;

unsafe extern "C" fn callback(buffer_data: *mut c_void, frames: u32) {
    let buffer_data = buffer_data as *mut Frame;
    let frames: usize = frames as usize;
    if frames <= BUFFER_CAPACITY - GLOBAL_FRAMES_COUNT {
        std::ptr::copy(
            buffer_data,
            GLOBAL_FRAMES.as_mut_ptr().add(GLOBAL_FRAMES_COUNT),
            frames,
        );
        GLOBAL_FRAMES_COUNT += frames;
    } else if frames <= BUFFER_CAPACITY {
        std::ptr::copy(
            GLOBAL_FRAMES.as_mut_ptr().add(frames),
            GLOBAL_FRAMES.as_mut_ptr(),
            BUFFER_CAPACITY - frames,
        );
        std::ptr::copy(
            buffer_data,
            GLOBAL_FRAMES.as_mut_ptr().add(BUFFER_CAPACITY - frames),
            frames,
        );
    } else {
        std::ptr::copy(buffer_data, GLOBAL_FRAMES.as_mut_ptr(), BUFFER_CAPACITY);
        GLOBAL_FRAMES_COUNT = BUFFER_CAPACITY;
    }
}

fn get_drawable_vecs(sample: f32, cell_w: f32, h: f32, i: f32) -> (Vector2, Vector2) {
    if sample > 0.0 {
        (
            Vector2 {
                x: cell_w * i,
                y: h / 2.0 - (h / 2.0 * sample),
            },
            Vector2 {
                x: cell_w,
                y: (h / 2.0 * sample),
            },
        )
    } else {
        (
            Vector2 {
                x: cell_w * i,
                y: h / 2.0,
            },
            Vector2 {
                x: cell_w,
                y: (h / 2.0 * sample.abs()),
            },
        )
    }
}

fn main() {
    let (mut rl, thread) = raylib::init().size(860, 600).title("DJust").build();

    let mut ra: RaylibAudio = RaylibAudio::init_audio_device();
    let mut music = Music::load_music_stream(&thread, MUSIC).unwrap();
    ra.play_music_stream(&mut music);
    unsafe {
        ffi::AttachAudioStreamProcessor(music.stream, Some(callback));
    }

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::new(12, 12, 13, 255));
            let h = d.get_screen_height() as f32;
            let w = d.get_screen_width() as f32;
            unsafe {
                let cell_w: f32 = w / GLOBAL_FRAMES.len() as f32;
                for (i, frame) in GLOBAL_FRAMES.iter().enumerate() {
                    let (v_pos_left, v_size_left) =
                        get_drawable_vecs(frame.left, cell_w, h, i as f32);
                    let (v_pos_right, v_size_right) =
                        get_drawable_vecs(frame.right, cell_w, h, i as f32);
                    d.draw_rectangle_v(v_pos_left, v_size_left, COLOR_PALE_RED);
                    d.draw_rectangle_v(v_pos_right, v_size_right, COLOR_PALE_RED);
                }
            }
        }
        {
            if rl.is_key_pressed(KEY_SPACE) {
                if ra.is_music_stream_playing(&music) {
                    ra.pause_music_stream(&mut music);
                } else {
                    ra.resume_music_stream(&mut music);
                }
            }
        }
        {
            ra.update_music_stream(&mut music);
        }
    }
}
