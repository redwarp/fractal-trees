use std::borrow::Cow;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;

use skia_safe::{Canvas, EncodedImageFormat, Paint, Surface};
mod geometry;
mod hitomezashi;
mod maze;
mod mountain;
mod mugen;
mod night;
mod rabbit;
mod tree;
mod utils;

const WIDTH: i32 = 1920;
const HEIGHT: i32 = 1080;

struct Painting<'a> {
    draw_fn: fn(&mut Canvas) -> (),
    output: &'a str,
}

fn main() -> Result<(), String> {
    let paintings = vec![
        Painting::new(tree::draw, "tree"),
        Painting::new(mountain::draw, "mountain"),
        Painting::new(maze::draw, "maze"),
        Painting::new(mugen::draw, "mugen"),
        Painting::new(night::draw, "night"),
        Painting::new(rabbit::draw, "rabbit"),
        Painting::new(hitomezashi::draw, "hitomezashi"),
    ];

    for painting in &paintings {
        draw(painting.draw_fn, painting.output, (WIDTH, HEIGHT))?;
    }

    Ok(())
}

fn draw(draw_fn: fn(&mut Canvas) -> (), output: &str, size: (i32, i32)) -> Result<(), String> {
    println!("===\nDrawing {}", output);

    let (width, height) = size;
    let mut paint = Paint::default();
    paint.set_anti_alias(true);

    let mut surface =
        Surface::new_raster_n32_premul((width, height)).expect("No SKIA surface available.");

    let canvas = surface.canvas();

    draw_fn(canvas);

    // Save the result.
    create_dir_all("images")
        .map_err(|_e| "ERROR: Coudn\'t create the `rendering` directory".to_string())?;

    let file_name = format!("images/{}_{}x{}.png", output, width, height);
    let mut file = match File::create(file_name) {
        Err(_e) => {
            return Err(format!(
                "ERROR: failed to create the file `{}_{}x{}.png`",
                output, width, height
            ))
        }
        Ok(file) => file,
    };
    let image = surface.image_snapshot();
    match image.encode_to_data(EncodedImageFormat::PNG) {
        Some(data) => {
            file.write_all(&optimize_image(data.as_bytes())?)
                .map_err(|_e| format!("ERROR: failed to write in the file `{}.png`", output))?;
        }
        None => {
            return Err("ERROR: failed to encode image as PNG.".to_string());
        }
    };

    println!("===\n");

    Ok(())
}

impl Painting<'_> {
    fn new(draw_fn: fn(&mut Canvas) -> (), output: &str) -> Painting {
        Painting { draw_fn, output }
    }
}

#[cfg(not(debug_assertions))]
fn optimize_image(png_data: &[u8]) -> Result<Cow<[u8]>, &str> {
    let mut opts = oxipng::Options::from_preset(5);
    opts.strip = oxipng::Headers::Safe;

    oxipng::optimize_from_memory(png_data, &opts)
        .map_err(|_e| "Couldn't optimize png")
        .map(Cow::Owned)
}

#[cfg(debug_assertions)]
fn optimize_image(png_data: &[u8]) -> Result<Cow<[u8]>, &str> {
    // No-op
    Ok(Cow::Borrowed(png_data))
}
