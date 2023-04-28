// use ndarray::prelude::*;
use plotters::prelude::*;
use std::f32::consts::PI;


// define a function that creates a sine wave and returns time and the sine wave
fn sine(duration: f32, freq: f32) -> (Vec<f32>, Vec<f32>) {
    let mut wave: Vec<f32> = Vec::new();
    let mut time: Vec<f32> = Vec::new();
    let n: usize = (duration * freq) as usize;
    (0..n).for_each(|i: usize| {
        let t: f32 = i as f32 / freq;
        let y: f32 = (2.0 * PI * freq * t).sin();
        wave.push(y);
        time.push(t);
    });
    (time, wave)
}


// fn plot() {
//     let root_area = BitMapBackend::new("/Users/gabestechschulte/Downloads/sine.png", (600, 400))
//       .into_drawing_area();
    
//     root_area.fill(&WHITE).unwrap();
  
//     let mut ctx = ChartBuilder::on(&root_area)
//       .set_label_area_size(LabelAreaPosition::Left, 40)
//       .set_label_area_size(LabelAreaPosition::Bottom, 40)
//       .caption("Line Plot Demo", ("sans-serif", 40))
//       .build_cartesian_2d(-10..10, 0..100)
//       .unwrap();
  
//     ctx.configure_mesh().draw().unwrap();
  
//     ctx.draw_series(
//       LineSeries::new((-10..=10).map(|x| (x, x* x)), &GREEN)
//     ).unwrap();
//   }

// define a function that plots the sine wave using plotters
fn plot_sine(time: Vec<f32>, wave: Vec<f32>) {

    let min_y: f32 = wave.iter().fold(0.0, |a, &b| a.min(b));
    let max_y: f32 = wave.iter().fold(0.0, |a, &b| a.max(b));

    let root = BitMapBackend::new("/Users/gabestechschulte/Downloads/sine.png", (640, 480))
        .into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Sine Wave", ("sans-serif", 50))
        .build_cartesian_2d(0.0..time[time.len() - 1], min_y..max_y)
        .unwrap();

    chart
        .draw_series(LineSeries::new(time.iter().zip(wave.iter()), &RED))
        .unwrap();
}

fn main() {
    let (time, wave) = sine(1.0, 440.0);
    // print the shape of the time
    println!("Time: {:?}", time.len());
    let min: f32 = wave.iter().fold(0.0, |a, &b| a.min(b));
    print!("Min: {:?}", min);
    // print the max value from the vector
    // println!("Max: {:?}", wave.iter().max());
    // println!("Min: {:?}", wave.iter().min());
    // print the sine wave
    // println!("{:?}", wave);
    //plot_sine(time, wave);
    // plot();
}


