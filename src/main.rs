mod fft;

use fft::fft;
use num::Complex;
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
    _right: f32,
}

const N: usize = 512;

static mut INPS: [f32; N] = [0.0; N];
static mut OUTS: [Complex<f32>; N] = [Complex::new(0.0, 0.0); N];
static mut MAX_AMPL: f32 = 0.0;

fn amp(z: Complex<f32>) -> f32 {
    let real_part = z.re.abs();
    let imag_part = z.im.abs();
    if real_part >= imag_part {
        real_part
    } else {
        imag_part
    }
}

unsafe extern "C" fn callback(buffer_data: *mut c_void, frames: u32) {
    let buffer_data = buffer_data as *mut Frame;
    let frames: usize = frames as usize;
    if frames < N {
        return;
    }
    for (i, inp) in INPS.iter_mut().enumerate() {
        if i >= N {
            break;
        }
        let frame = &mut *buffer_data.add(i);
        *inp = frame.left;
    }
    let outputs = fft(&INPS);
    MAX_AMPL = 0.0;
    for (i, out) in outputs.iter().enumerate() {
        let a = amp(*out);
        if MAX_AMPL < a {
            MAX_AMPL = a;
        }
        OUTS[i] = *out;
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
            let cell_w: f32 = w / N as f32;
            unsafe {
                for (i, out) in OUTS.iter().enumerate() {
                    let t = amp(*out) / MAX_AMPL;
                    let v_pos = Vector2 {
                        x: cell_w * i as f32,
                        y: h - (h * t),
                    };
                    let v_size = Vector2 {
                        x: cell_w,
                        y: (h * t),
                    };
                    d.draw_rectangle_v(v_pos, v_size, COLOR_PALE_RED);
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
