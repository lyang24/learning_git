use num::Complex;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex{ re: 0.0, im: 0.0};
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

fn parse_pair<T: FromStr>(s: &str, sep: char) -> Option<(T,T)> {
    match s.find(sep) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index..])) {
                (OK(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}
