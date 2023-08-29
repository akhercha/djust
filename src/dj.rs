use super::fft::fft;
use num::Complex;
use raylib::core::color::Color;
use raylib::prelude::{RaylibDraw, RaylibDrawHandle, Vector2};
use std::os::raw::c_void;

const N: usize = 16384;
const COLOR_PALE_RED: Color = Color::new(245, 85, 73, 255);

const TWO_PI: f32 = 2.0 * std::f32::consts::PI;

static mut INPS: [f32; N] = [0.0; N];
// Smoothed input
static mut INPS_TWO: [f32; N] = [0.0; N];
static mut OUTS: [Complex<f32>; N] = [Complex::new(0.0, 0.0); N];

#[derive(Clone, Copy)]
struct Frame {
    left: f32,
    _right: f32,
}

fn amp(z: Complex<f32>) -> f32 {
    (z.re * z.re + z.im * z.im).sqrt()
}

pub unsafe extern "C" fn callback(buffer_data: *mut c_void, frames: u32) {
    let fs = buffer_data as *mut Frame;
    let frames = frames as usize;
    INPS.rotate_left(frames);
    for i in 0..frames {
        INPS[N - frames + i] = (*fs.add(i)).left;
    }
}

// TODO: refacto this block
pub fn draw_music(d: &mut RaylibDrawHandle) {
    d.clear_background(Color::new(12, 12, 13, 255));

    let h = d.get_screen_height() as f32;
    let w = d.get_screen_width() as f32;

    unsafe {
        for (i, inp) in INPS.iter().enumerate().take(N / 2) {
            let t = i as f32 / N as f32;
            let hann = 0.5 - 0.5 * (TWO_PI * t).cos();
            INPS_TWO[i] = inp * hann;
        }
        fft(&INPS_TWO, &mut OUTS);
    }

    let mut max_ampl: f32 = 0.0;
    unsafe {
        for out in OUTS.iter().take(N / 2) {
            let a = amp(*out);
            if max_ampl < a {
                max_ampl = a;
            }
        }
    }

    let step: f32 = 1.06;
    let low_f: f32 = 20.0;
    let mut f: f32 = low_f;
    let mut m: usize = 0;
    while (f as usize) < N / 2 {
        f = (f * step).ceil();
        m += 1;
    }
    // Get the width of a cell
    let cell_w: f32 = w / m as f32;
    m = 0;
    f = low_f;
    // For each frequencies...
    while (f as usize) < N / 2 {
        let f1: f32 = (f * step).ceil();
        let mut a: f32 = 0.0;
        unsafe {
            // Compute the average amplitude of the frequency band
            let mut q: usize = f as usize;
            while (q < N / 2) && (q < f1 as usize) {
                let b: f32 = amp(OUTS[q]);
                if b > a {
                    a = b;
                }
                q += 1;
            }
            // Normalize the amplitude
            let t = a / (max_ampl);
            let v_pos = Vector2 {
                x: cell_w * m as f32,
                y: h - (h / 2.0 * t),
            };
            let v_size = Vector2 {
                x: cell_w,
                y: (h / 2.0 * t),
            };
            // Draw the rectangle
            d.draw_rectangle_v(v_pos, v_size, COLOR_PALE_RED);
        }
        f = (f * step).ceil();
        m += 1;
    }
}
