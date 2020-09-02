use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;

use skia_safe::{EncodedImageFormat, Paint, Surface};
mod tree;

fn main() {
    let width = 1920;
    let height = 1080;
    let mut paint = Paint::default();
    paint.set_anti_alias(true);

    let mut surface =
        Surface::new_raster_n32_premul((width, height)).expect("No SKIA surface available.");

    let canvas = surface.canvas();

    tree::draw_tree(canvas);

    // Save the result.

    match create_dir_all("rendering") {
        Err(_e) => {
            eprintln!("ERROR: Coudn't create the `rendering` directory");
            std::process::exit(1)
        }
        Ok(()) => (),
    }

    let file_name = "rendering/tree.png";
    let mut file = match File::create(file_name) {
        Err(_e) => {
            eprintln!("ERROR: failed to create the file {}", file_name);
            std::process::exit(1)
        }
        Ok(file) => file,
    };
    let image = surface.image_snapshot();
    match image.encode_to_data(EncodedImageFormat::PNG) {
        Some(data) => {
            match file.write_all(data.as_bytes()) {
                Err(_e) => eprintln!("ERROR: failed to write in file `{}`", file_name),
                Ok(()) => (),
            };
        }
        None => {
            eprintln!("ERROR: failed to encode image as PNG.");
        }
    }
}
