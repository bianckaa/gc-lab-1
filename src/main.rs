mod polygon;

use raylib::prelude::*;
use crate::polygon::{draw_polygon, export_bmp, fill_polygon};

fn main() {
    let image_width = 800;
    let image_height = 600;

    let mut image = Image::gen_image_color(image_width, image_height, Color::BLACK);

    let line_color = Color::WHITE;
    let fill1 = Color::new(0xE6, 0xC8, 0x28, 255);
    let fill2 = Color::new(0x3C, 0x8C, 0xDC, 255);
    let fill3 = Color::new(0xC8, 0x46, 0x46, 255);
    let fill4 = Color::new(0x46, 0xB4, 0x64, 255);

    let polygon1: [(i32, i32); 10] = [
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383),
    ];

    let polygon2: [(i32, i32); 4] = [
        (321, 335), (288, 286), (339, 251), (374, 302),
    ];

    let polygon3: [(i32, i32); 3] = [
        (377, 249), (411, 197), (436, 249),
    ];

    let polygon4: [(i32, i32); 18] = [
        (413, 177), (448, 159), (502, 88),  (553, 53),  (535, 36),
        (676, 37),  (660, 52),  (750, 145), (761, 179), (672, 192),
        (659, 214), (615, 214), (632, 230), (580, 230), (597, 215),
        (552, 214), (517, 144), (466, 180),
    ];

    let polygon5: [(i32, i32); 4] = [
        (682, 175), (708, 120), (735, 148), (739, 170),
    ];

    fill_polygon(&mut image, &[&polygon1], fill1);
    draw_polygon(&mut image, &polygon1, line_color);

    fill_polygon(&mut image, &[&polygon2], fill2);
    draw_polygon(&mut image, &polygon2, line_color);

    fill_polygon(&mut image, &[&polygon3], fill3);
    draw_polygon(&mut image, &polygon3, line_color);

    fill_polygon(&mut image, &[&polygon4, &polygon5], fill4);
    draw_polygon(&mut image, &polygon4, line_color);
    draw_polygon(&mut image, &polygon5, line_color);

    image.export_image("out.png");
    export_bmp(&image, "out.bmp");

    println!("Image saved successfully as 'out.png' and 'out.bmp'!");
}
