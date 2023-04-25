use plotters::prelude::*;
use std::f32::consts::PI;


// define a function that creates a sine wave and returns time and the sine wave
fn sine(duration: f32, freq: f32) -> (Vec<f32>, Vec<f32>) {
    let mut wave = Vec::new();
    let mut time = Vec::new();
    let n = (duration * freq) as usize;
    for i in 0..n {
        let t = i as f32 / freq;
        let y = (2.0 * PI * freq * t).sin();
        wave.push(y);
        time.push(t);
    }
    (time, wave)
}

// define a function that plots the sine wave using plotters

fn plot_sine(time: Vec<f32>, wave: Vec<f32>) {
    let root = BitMapBackend::new("sine.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption("Sine Wave", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..time[time.len() - 1], -1.0..1.0)
        .unwrap();
    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .x_desc("Time")
        .y_desc("Amplitude")
        .draw()
        .unwrap();
    chart
        .draw_series(LineSeries::new(time.iter().zip(wave.iter()), &RED))
        .unwrap();
}

fn main() {
    let (time, wave) = sine(1.0, 440.0);
    plot_sine(time, wave);
}


