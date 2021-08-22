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
) -> Vec<Vec<u8>> {
    let mut all_cols: Vec<Vec<u8>> = Vec::with_capacity(width);
    for img_y in 0..height {
        let mut row = Vec::with_capacity(height);
        for img_x in 0..width {
            let cx = x_min + (x_max - x_min) * (img_x as f64 / width as f64);
            let cy = y_min + (y_max - y_min) * (img_y as f64 / height as f64);
            let escaped_at_iter = mandelbrot_at_point(cx, cy, no_of_iters);
            let pixel_color: u8 =
                ((((escaped_at_iter - 0) * (255 - 0)) / (no_of_iters - 0)) + 0) as u8;
            row.push(pixel_color);
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

fn render_mandelbrot(mandelbrot_set: &[Vec<u8>], bounds: (usize, usize)) -> image::ImageResult<()> {
    // for row in mandelbrot_set {
    //     let mut line = String::with_capacity(row.len());
    //     for col in row {
    //         let value = match *col {
    //             0..=2 => ' ',
    //             3..=5 => '.',
    //             6..=10 => '•',
    //             11..=20 => '◦', //*
    //             21..=50 => '+',
    //             51..=100 => 'x',
    //             101..=150 => '$',
    //             151..=200 => '#',
    //             _ => '%',
    //         };
    //         line.push(value);
    //     }
    //     println!("{}", line);
    // }
    // println!();
    let mandelbrot = mandelbrot_set.clone().concat();
    image::save_buffer(
        "img.png",
        &mandelbrot,
        bounds.0 as u32,
        bounds.1 as u32,
        image::ColorType::L8,
    )?;
    let mut img = image::open("m.png")?;
    img.invert();
    let img1 = img.crop(900, 700, (bounds.0 / 3) as u32, (bounds.1 / 3) as u32);
    img1.save("croped.png")?;
    img.save("img_inverted.png")?;
    println!("image saved to img.png, img_inverted.png, cropped.png");
    Ok(())
}

fn main() {
    let (width, height) = (4000, 3000);
    let mandelbrot = calculate_mandelbrot(400, -1.20, 0.20, -1.0, 0.35, width, height);
    match render_mandelbrot(&mandelbrot, (width, height)) {
        Ok(_) => println!("images saved successfully"),
        Err(err) => println!("an error occured {:?}", err),
    }
}
