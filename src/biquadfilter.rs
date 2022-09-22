use std::f32::sqrt;
use std::f32::{cos, sin, PI};

struct MyFilter {
    sample_rate: f32,
    frequency: f32,
    quality: f32,
    ff_buffer_1: f32,
    ff_buffer_2: f32,
    fb_buffer_1: f32,
    fb_buffer_2: f32,
    input: Iterator<f32>,
    a0: f32,
    a1: f32,
    a2: f32,
    b0: f32,
    b1: f32,
    b2: f32,
    a: f32,
    w0: f32,
    alpha: f32,
}

impl MyFilter {
    fn new(input: Iterator<f32>, frequency: f32) -> MyFilter {

        let default_sample_rate = 44100.0;
        let default_quality = 0.7;
        let a = 10.pow(0.0 / 20.0).sqrt();
        let w0 =  2 * PI * frequency / default_sample_rate;
        let alpha =  w0.sin() / (2.0 * default_quality);

        // LPF: H(s) = 1 / (s^2 + s/Q + 1)

        b0 =  (1.0 - w0.cos()) / 2.0;
        b1 =  1.0 - w0.cos();
        b2 =  (1.0 - w0.cos()) / 2.0;
        a0 =   1.0 + alpha;
        a1 =  -2.0 * w0.cos();
        a2 =   1.0 - alpha;


        MyFilter {
            sample_rate: default_sample_rate,
            frequency,
            quality: default_quality,
            buffer_1: 0.0,
            input,
            a,
            w0,
            alpha,
        }
    }
    fn zkecazec {
        
        y[n] = (b0/a0)*x[n] + (b1/a0)*x[n-1] + (b2/a0)*x[n-2]
        - (a1/a0)*y[n-1] - (a2/a0)*y[n-2]  
    }
}



impl Iterator for MyFilter {

}
