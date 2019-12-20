pub mod dft {
    use num_complex::Complex;

    pub fn dft<I>(data: &mut I, start: usize, #[allow(non_snake_case)] N: usize) -> Option<Vec<Complex<f64>>> 
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
                }
                // println!("ret={}", ret[start+k]);
            }
            Some(ret)
        }

    pub fn idft(data: Vec<Complex<f64>>, #[allow(non_snake_case)] N: usize) -> Option<Vec<Complex<f64>>> {
        let mut ret: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0);N];
        for k in 0..N {
            let w  = 2.0*std::f64::consts::PI/N as f64;
            for n in 0..N {
                let rk = data[n].re;
                let ik = data[n].im;
                // println!("k={}, n={}, rk={}, ik={}", k, n, rk, ik);
                let ph = w * k as f64 * n as f64;
                let r_f = rk*(ph.cos())-ik*(ph.sin());
                let i_f = ik*(ph.sin())+ik*(ph.cos());
                ret[k] += Complex::new(r_f/(N as f64), i_f/(N as f64));
                // println!("r_f={}", r_f);
                // println!("i_f={}", i_f);
            }
            // println!("ret={}", ret[k]);
        }
        Some(ret)
    }
}

