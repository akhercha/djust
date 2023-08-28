use num::Complex;
use std::f32::consts::PI;

/// Fast Fourier Transform
/// https://en.wikipedia.org/wiki/Fast_Fourier_transform
fn fft(inputs: &[f32]) -> Vec<Complex<f32>> {
    let n = inputs.len();
    if n <= 1 {
        return vec![Complex::new(inputs[0], 0.0)];
    }
    let even_inputs: Vec<_> = inputs.iter().step_by(2).cloned().collect();
    let odd_inputs: Vec<_> = inputs.iter().skip(1).step_by(2).cloned().collect();

    let even = fft(&even_inputs);
    let odd = fft(&odd_inputs);

    let mut outputs = vec![Complex::new(0.0, 0.0); n];
    for k in 0..(n / 2) {
        let t = (k as f32) / (n as f32);
        let v = Complex::from_polar(1.0, -2.0 * PI * t) * odd[k];
        outputs[k] = even[k] + v;
        outputs[k + n / 2] = even[k] - v;
    }

    outputs
}
