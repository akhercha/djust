mod fft;

use fft::fft;
use num::Complex;
use raylib::consts::KeyboardKey::KEY_SPACE;
use raylib::core::audio::{Music, RaylibAudio};
use raylib::core::color::Color;
use raylib::prelude::*;
use std::os::raw::c_void;

use std::ptr::copy;

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

const N: usize = 16384;

static mut INPS: [f32; N] = [0.0; N];
static mut OUTS: [Complex<f32>; N] = [Complex::new(0.0, 0.0); N];

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
    let fs = buffer_data as *mut Frame;
    let frames = frames as usize;
    INPS.rotate_left(frames);
    for i in 0..frames {
        INPS[N - frames + i] = (*fs.add(i)).left;
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

    let mut counter_fft: usize = 0;
    while !rl.window_should_close() {
        {
            let mut d = rl.begin_drawing(&thread);

            d.clear_background(Color::new(12, 12, 13, 255));

            let h = d.get_screen_height() as f32;
            let w = d.get_screen_width() as f32;

            unsafe {
                if counter_fft % 3 == 0 {
                    fft(&INPS, &mut OUTS);
                }
            }

            let mut max_ampl: f32 = 0.0;
            unsafe {
                for out in OUTS {
                    let a = amp(out);
                    if max_ampl < a {
                        max_ampl = a;
                    }
                }
            }

            let step: f32 = 1.06;
            let mut f: f32 = 20.0;
            let mut m: usize = 0;
            while (f as usize) < N {
                f *= step;
                m += 1;
            }
            let cell_w: f32 = w / m as f32;
            m = 0;
            f = 20.0;
            while (f as usize) < N {
                let f1: f32 = f * step;
                let mut a: f32 = 0.0;
                unsafe {
                    let mut q: usize = f as usize;
                    while (q < N) && (q < f1 as usize) {
                        a += amp(OUTS[q]);
                        q += 1;
                    }
                    a /= (f1 as usize - f as usize + 1) as f32;
                    let t = a / (max_ampl / 3.0);
                    let v_pos = Vector2 {
                        x: cell_w * m as f32,
                        y: h - (h * t),
                    };
                    let v_size = Vector2 {
                        x: cell_w,
                        y: (h * t),
                    };
                    d.draw_rectangle_v(v_pos, v_size, COLOR_PALE_RED);
                }
                f *= step;
                m += 1;
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
        counter_fft += 1;
    }
}
