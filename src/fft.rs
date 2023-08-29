use num::Complex;
use std::f32::consts::PI;

fn fft_recursive(inputs: &[f32], outputs: &mut [Complex<f32>], step: usize) {
    let n: usize = outputs.len();
    if n == 1 {
        outputs[0] = Complex::new(inputs[0], 0.0);
        return;
    }

    let half_n = n / 2;
    fft_recursive(inputs, &mut outputs[..half_n], step * 2);
    fft_recursive(&inputs[step..], &mut outputs[half_n..], step * 2);

    for k in 0..half_n {
        let t = (k as f32) / (n as f32);
        let v = Complex::from_polar(1.0, -2.0 * PI * t) * outputs[k + half_n];
        outputs[k + half_n] = outputs[k] - v;
        outputs[k] += v;
    }
}

/// Fast Fourier Transform
/// https://en.wikipedia.org/wiki/Fast_Fourier_transform
pub fn fft(inputs: &[f32], output: &mut [Complex<f32>]) {
    fft_recursive(inputs, output, 1);
}
