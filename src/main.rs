mod polygon;

use raylib::prelude::*;
use crate::polygon::{draw_polygon, fill_polygon};

fn main() {
    let image_width = 800;
    let image_height = 600;

    let mut image = Image::gen_image_color(image_width, image_height, Color::BLACK);

    let line_color = Color::WHITE;
    let fill4 = Color::new(0x46, 0xB4, 0x64, 255);

    let polygon4: [(i32, i32); 18] = [
        (413, 177), (448, 159), (502, 88),  (553, 53),  (535, 36),
        (676, 37),  (660, 52),  (750, 145), (761, 179), (672, 192),
        (659, 214), (615, 214), (632, 230), (580, 230), (597, 215),
        (552, 214), (517, 144), (466, 180),
    ];

    let polygon5: [(i32, i32); 4] = [
        (682, 175), (708, 120), (735, 148), (739, 170),
    ];

    fill_polygon(&mut image, &[&polygon4, &polygon5], fill4);

    draw_polygon(&mut image, &polygon4, line_color);
    draw_polygon(&mut image, &polygon5, line_color);

    let output_file_name = "out.png";
    image.export_image(output_file_name);

    println!("Image saved successfully as '{}'!", output_file_name);
}