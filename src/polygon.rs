use raylib::prelude::*;
use std::fs::File;
use std::io::Write;

pub fn export_bmp(img: &Image, filename: &str) {
    let width = img.width() as u32;
    let height = img.height() as u32;
    let row_size = ((width * 3 + 3) / 4) * 4;
    let pixel_data_size = row_size * height;
    let file_size = 14 + 40 + pixel_data_size;

    let mut buffer = Vec::with_capacity(file_size as usize);

    buffer.extend_from_slice(b"BM");
    buffer.extend_from_slice(&file_size.to_le_bytes());
    buffer.extend_from_slice(&0u32.to_le_bytes());
    buffer.extend_from_slice(&(14u32 + 40u32).to_le_bytes());

    buffer.extend_from_slice(&40u32.to_le_bytes());
    buffer.extend_from_slice(&(width as i32).to_le_bytes());
    buffer.extend_from_slice(&(height as i32).to_le_bytes());
    buffer.extend_from_slice(&1u16.to_le_bytes());
    buffer.extend_from_slice(&24u16.to_le_bytes());
    buffer.extend_from_slice(&0u32.to_le_bytes());
    buffer.extend_from_slice(&pixel_data_size.to_le_bytes());
    buffer.extend_from_slice(&2835i32.to_le_bytes());
    buffer.extend_from_slice(&2835i32.to_le_bytes());
    buffer.extend_from_slice(&0u32.to_le_bytes());
    buffer.extend_from_slice(&0u32.to_le_bytes());

    let rgba = img.get_image_data_u8(true);
    let padding = (row_size - width * 3) as usize;

    for y in 0..height as usize {
        for x in 0..width as usize {
            let idx = (y * width as usize + x) * 4;
            buffer.push(rgba[idx + 2]);
            buffer.push(rgba[idx + 1]);
            buffer.push(rgba[idx]);
        }
        buffer.extend(std::iter::repeat(0u8).take(padding));
    }

    let mut file = File::create(filename).expect("could not create bmp file");
    file.write_all(&buffer).expect("could not write bmp file");
}

pub fn point(img: &mut Image, x: i32, y: i32, color: Color) {
    if x >= 0 && x < img.width() && y >= 0 && y < img.height() {
        img.draw_pixel(x, y, color);
    }
}

pub fn line(img: &mut Image, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
    let mut x = x0;
    let mut y = y0;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    loop {
        point(img, x, y, color);
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}

pub fn draw_polygon(img: &mut Image, points: &[(i32, i32)], color: Color) {
    let n = points.len();
    for i in 0..n {
        let (x0, y0) = points[i];
        let (x1, y1) = points[(i + 1) % n];
        line(img, x0, y0, x1, y1, color);
    }
}

pub fn fill_polygon(img: &mut Image, contours: &[&[(i32, i32)]], color: Color) {
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    for contour in contours {
        for &(_, y) in contour.iter() {
            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }
        }
    }

    for y in min_y..=max_y {
        let mut crossings: Vec<f32> = Vec::new();

        for contour in contours {
            let n = contour.len();
            for i in 0..n {
                let (x0, y0) = contour[i];
                let (x1, y1) = contour[(i + 1) % n];

                if (y0 <= y && y1 > y) || (y1 <= y && y0 > y) {
                    let x = x0 as f32
                        + (y - y0) as f32 / (y1 - y0) as f32 * (x1 - x0) as f32;
                    crossings.push(x);
                }
            }
        }

        crossings.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut i = 0;
        while i + 1 < crossings.len() {
            let x_start = crossings[i].ceil() as i32;
            let x_end = crossings[i + 1].floor() as i32;
            for x in x_start..=x_end {
                point(img, x, y, color);
            }
            i += 2;
        }
    }
}