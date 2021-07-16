use std::usize;

use num::complex::Complex;

fn calculate_mandelbrot(
    no_of_iters: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize,
    height: usize,
) -> Vec<Vec<usize>> {
    let mut all_cols: Vec<Vec<usize>> = Vec::with_capacity(width);
    for img_y in 0..height {
        let mut row = Vec::with_capacity(height);
        for img_x in 0..width {
            let cx = x_min + (x_max - x_min) * (img_x as f64 / width as f64);
            let cy = y_min + (y_max - y_min) * (img_y as f64 / height as f64);
            let escaped_at_iter = mandelbrot_at_point(cx, cy, no_of_iters);
            row.push(escaped_at_iter);
        }
        all_cols.push(row);
    }
    all_cols
}

fn mandelbrot_at_point(x: f64, y: f64, max_iter: usize) -> usize {
    let mut z = Complex::new(0.0, 0.0);
    let c = Complex::new(x, y);
    for i in 0..max_iter {
        if z.norm() > 2.0 {
            return i;
        }
        z = z * z + c;
    }
    max_iter
}

fn render_mandelbrot(mandelbrot_set: &[Vec<usize>]) {
    for row in mandelbrot_set {
        let mut line = String::with_capacity(row.len());
        for col in row {
            let value = match *col {
                0..=2 => ' ',
                3..=5 => '.',
                6..=10 => '•',
                11..=30 => '◦', //*
                31..=100 => '+',
                101..=200 => 'x',
                201..=400 => '$',
                401..=700 => '#',
                _ => '%',
            };
            line.push(value);
        }
        println!("{}", line);
    }
    println!();
}

fn main() {
    let mandelbrot = calculate_mandelbrot(1000, -2.0, 1.0, -1.0, 1.0, 90, 40);
    render_mandelbrot(&mandelbrot);
}
