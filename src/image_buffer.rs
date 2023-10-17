use chrono::Duration;
use image::{Rgb, RgbImage};
use imageproc::drawing::Canvas;
use rusttype::{point, Font, PositionedGlyph, Rect, Scale};
use std::cmp::max;

// Code mostly taken wholesale from
// https://github.com/image-rs/imageproc/blob/master/src/drawing/text.rs

fn layout_glyphs(
    scale: Scale,
    font: &Font,
    text: &str,
    mut f: impl FnMut(PositionedGlyph, Rect<i32>),
) -> (i32, i32) {
    let v_metrics = font.v_metrics(scale);

    let (mut w, mut h) = (0, 0);

    for g in font.layout(text, scale, point(0.0, v_metrics.ascent)) {
        if let Some(bb) = g.pixel_bounding_box() {
            w = max(w, bb.max.x);
            h = max(h, bb.max.y);
            f(g, bb);
        }
    }

    (w, h)
}

/// Draws colored text on an image in place.
///
/// `scale` is augmented font scaling on both the x and y axis (in pixels).
///
/// Note that this function *does not* support newlines, you must do this manually.
fn draw_text_mut<'a>(
    canvas: &'a mut RgbImage,
    color: Rgb<u8>,
    x: i32,
    y: i32,
    scale: Scale,
    font: &'a Font<'a>,
    text: &'a str,
) {
    let image_width = canvas.width() as i32;
    let image_height = canvas.height() as i32;

    layout_glyphs(scale, font, text, |g, bb| {
        g.draw(|gx, gy, gv| {
            let gx = gx as i32 + bb.min.x;
            let gy = gy as i32 + bb.min.y;

            let image_x = gx + x;
            let image_y = gy + y;

            if (0..image_width).contains(&image_x) && (0..image_height).contains(&image_y) {
                // code edited here from original, if there's any coverage just make it uniformly
                // the same color, else don't draw
                if gv > 0.1 {
                    canvas.draw_pixel(image_x as u32, image_y as u32, color);
                }
            }
        })
    });
}

fn draw_text_centered(img: &mut RgbImage, text: &str, x: f32, y: f32, font: &Font, scale: f32) {
    let black = image::Rgb([0u8, 0u8, 0u8]);
    let (text_width, text_height) = measure_text(&font, &text, scale);
    let scale = Scale::uniform(scale);
    let text_x = (x - text_width / 2.0).ceil() as i32;
    let text_y = (y - text_height / 2.0).ceil() as i32;

    draw_text_mut(img, black, text_x, text_y, scale, &font, &text);
}

fn measure_text(font: &Font, text: &str, font_size: f32) -> (f32, f32) {
    let font_size = Scale::uniform(font_size);
    let v_metrics = font.v_metrics(font_size);

    let xpad = 0f32;
    let ypad = 0f32;

    let glyphs: Vec<_> = font
        .layout(text, font_size, point(xpad, ypad + v_metrics.ascent))
        .collect();

    let height = (v_metrics.ascent - v_metrics.descent).ceil();
    let width = {
        let min_x = glyphs
            .first()
            .map(|g| g.pixel_bounding_box().unwrap().min.x)
            .unwrap();
        let max_x = glyphs
            .last()
            .map(|g| g.pixel_bounding_box().unwrap().max.x)
            .unwrap();
        (max_x - min_x) as f32
    };

    (width, height)
}

pub struct ImageBuffer {
    temp: f32,
    image: RgbImage,
    count: usize,
}

impl ImageBuffer {
    pub fn new() -> Self {
        let image = RgbImage::from_fn(800, 480, |_, _| -> Rgb<u8> { Rgb([255u8, 255u8, 255u8]) });
        ImageBuffer {
            temp: 0.0,
            image,
            count: 0,
        }
    }
    pub fn get_image(&self) -> f32 {
        // TODO: save image as png in memory
        0.0
    }
    pub fn update_image(&mut self, new_val: f32) {
        self.count += 1;
        let white = image::Rgb([255u8, 255u8, 255u8]);
        let black = image::Rgb([0u8, 0u8, 0u8]);
        let red = image::Rgb([255u8, 0u8, 0u8]);

        let font_data: &[u8] = include_bytes!("../fonts/Comfortaa-Medium.ttf");
        let font: Font<'static> = Font::try_from_bytes(font_data).expect("failed to open font");
        let temp_x = 10.0;
        let temp_y = 0.0;
        let temp_size = 150.0;
        let temp_text = format!("No: {} - {}°C", self.count, self.temp);
        draw_text_centered(
            &mut self.image,
            &temp_text,
            temp_x,
            temp_y,
            &font,
            temp_size,
        )
    }
}
