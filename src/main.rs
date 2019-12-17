use failure::Error;
use num_complex::Complex;
use plotlib::page::Page;
use plotlib::view::ContinuousView;
use plotlib::style::Line;
use rodio::source::{Source, Buffered};
use rodio::buffer::SamplesBuffer;

#[allow(dead_code)]
fn get_type<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

fn draw_graph(data: &Vec<(f64, f64)>, outpath: &str) -> Result<(), Error> {
    let mut style = plotlib::line::Style::new();
    let li = plotlib::line::Line::new(&data)
        .style(style.colour("#000000"));
    let v = ContinuousView::new().add(&li);
    Page::single(&v).save(outpath)?;
    Ok(())
}

fn create_sine_wave(a: f64, f: f64, fs: f64, length: f64) -> Buffered<SamplesBuffer<i16>> {
    // println!("size={}", fs*length);
    let mut data = Vec::with_capacity((fs*length) as usize);
    for n in 0..(fs*length) as usize {
        let mut w = a*(2.0*std::f64::consts::PI*f*(n as f64/fs)).sin();
        if w > 1.0 {
            w = 1.0;
        } else if w < -1.0 {
            w = -1.0;
        }
        w *= 32767.0; // -32768.0 <= w <= 32767.0
        // println!("{}", w as i16);
        data.push(w as i16);
    }
    SamplesBuffer::new(1, fs as u32, data).buffered()
}

fn dft<I>(data: &mut I, start: usize, N: usize) -> Option<Vec<Complex<f64>>>
    where I: Iterator<Item=i16>
{
    let vec_data: Vec<i16> = data.skip(start).collect();
    let mut ret: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0);N];
    for k in 0..N {
        for n in 0..N {
            let xn = vec_data[n] as f64/32768.0;
            // println!("k={}, n={}, xn={:?}", k, n, xn);
            let rk = xn*((2.0*std::f64::consts::PI*k as f64*n as f64)/N as f64).cos();
            let ik = xn*-((2.0*std::f64::consts::PI*k as f64*n as f64)/N as f64).sin();
            ret[start+k] += Complex::new(rk, ik);
            // println!("rk={}", rk);
            // println!("ik={}", ik);
            // println!("ret={}", ret[start+k]);
        }
    }
    Some(ret)
}

use std::io::Write;
fn main() {
    // create sine wave
    let source = create_sine_wave(1.0, 256.0, 44100.0, 1.0);
    // println!("{}", source.sample_rate()); // 44100
    // println!("{}", source.channels()); // 1
    let N = 4096;
    let dft_data = match dft(&mut source.clone(), 0, N) {
        Some(data) => data,
        None => {
            writeln!(std::io::stderr(), "DFT Error!").unwrap();
            std::process::exit(1);
        }
    };
    let fs = source.sample_rate();
    let freq_vec: Vec<f64> = (0..N).map(|k| k as f64*fs as f64/N as f64).collect();
    // draw graph
    let data: Vec<(f64, f64)> = source.clone().take(255).enumerate().map(|(i, x)| (i as f64, x as f64)).collect();
    draw_graph(&data, "wave.svg").expect("Failed draw graph");
    let amplitude: Vec<(f64, f64)> = dft_data.clone().iter().map(|d| d.norm_sqr()).zip(freq_vec.clone()).map(|(a, b)| (b, a)).collect();
    draw_graph(&amplitude, "amp.svg").expect("Failed draw graph");
    let phase: Vec<(f64, f64)> = dft_data.clone().iter().map(|d| d.im.round().atan2(d.re.round())).zip(freq_vec.clone()).map(|(a, b)| (b, a)).collect();
    draw_graph(&phase, "phase.svg").expect("Failed draw graph");

    // println!("{:?}", source.max());
}
