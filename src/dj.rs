use super::fft::fft;
use num::Complex;
use raylib::core::color::Color;
use raylib::prelude::{RaylibDraw, RaylibDrawHandle, Vector2};
use std::os::raw::c_void;

const N: usize = 16384;
const COLOR_PALE_RED: Color = Color::new(245, 85, 73, 255);

static mut INPS: [f32; N] = [0.0; N];
static mut OUTS: [Complex<f32>; N] = [Complex::new(0.0, 0.0); N];

#[derive(Clone, Copy)]
struct Frame {
    left: f32,
    right: f32,
}

pub unsafe extern "C" fn callback(buffer_data: *mut c_void, frames: u32) {
    let fs = buffer_data as *mut Frame;
    let frames = frames as usize;
    INPS.rotate_left(frames);
    for i in 0..frames {
        let avg_frame = ((*fs.add(i)).left + (*fs.add(i)).right) / 2.0;
        INPS[N - frames + i] = avg_frame;
    }
}

pub fn amp(z: Complex<f32>) -> f32 {
    let real_part = z.re.abs();
    let imag_part = z.im.abs();
    if real_part >= imag_part {
        real_part
    } else {
        imag_part
    }
}

// TODO: refacto this block
pub fn draw_music(d: &mut RaylibDrawHandle) {
    d.clear_background(Color::new(12, 12, 13, 255));

    let h = d.get_screen_height() as f32;
    let w = d.get_screen_width() as f32;

    unsafe {
        fft(&INPS, &mut OUTS);
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
            let t = a / (max_ampl / 2.0);
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
