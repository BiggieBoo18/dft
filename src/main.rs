use rodio::source::{Source, Buffered};
use rodio::buffer::SamplesBuffer;
use plotlib::page::Page;
use plotlib::view::ContinuousView;
use plotlib::style::Line;
use failure::Error;

#[allow(dead_code)]
fn get_type<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

fn draw_wave(data: &Vec<(f64, f64)>, outpath: &str) -> Result<(), Error> {
    let mut style = plotlib::line::Style::new();
    let li = plotlib::line::Line::new(&data)
        .style(style.colour("#000000"));
    let v = ContinuousView::new().add(&li);
    Page::single(&v).save(outpath)?;
    Ok(())
}

fn create_sine_wave(a: f64, f: f64, fs: f64, length: f64) -> Buffered<SamplesBuffer<i16>> {
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

fn dft<I>(data: &mut I, start: usize, N: usize)
    where I: Iterator<Item=i16>
{
    for k in 0..N-1 {
        for n in 0..N-1 {
            let xn = match data.nth(start+n) {
                None -> {println!("None!")},// return None},
                Some(xn) -> xn
            };
            println!("{:?}", xn);
            // let rk = ((2.0*std::f64::consts::PI*k*n)/N).cos()
        }
    }
}

fn main() {
    // create sine wave
    let mut source = create_sine_wave(1.0, 440.0, 44100.0, 1.0);
    // println!("{}", source.sample_rate()); // 44100
    // println!("{}", source.channels()); // 1
    let fs = source.sample_rate();
    dft(&mut source, 0, 256);

    // draw graph
    // let data: Vec<(f64, f64)> = source.clone().take(255).enumerate().map(|(i, x)| (i as f64, x as f64)).collect();
    // draw_wave(&data, "wave.svg").expect("Failed draw graph");

    // println!("{:?}", source.max());
}
