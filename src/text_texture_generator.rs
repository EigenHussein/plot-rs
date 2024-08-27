use freetype::{face::LoadFlag, ffi::FT_LOAD_RENDER, Face};
use image::{GrayImage, Luma};
use plot_rs::texts::CharMetrics;

fn get_pixel_range(font: &Face, pixel_size: u32, text: &'static str) -> (i32, i32, i32, i32) {
    let mut x_min = 1000000;
    let mut x_max = -1000000;
    let mut y_min = 1000000;
    let mut y_max = -1000000;

    let mut advance = 0;

    font.set_pixel_sizes(0, pixel_size).unwrap();

    for ch in text.chars() {
        font.load_char(ch as _, LoadFlag::RENDER).unwrap();
        let glyph = font.glyph();
        let processed_metrics = CharMetrics::from_freetype_metrics(&glyph.metrics());
        println!("{:?}", processed_metrics);
        // character's information
        let x0 = advance + processed_metrics.horiBearingX;
        let x1 = x0 + processed_metrics.width;
        let y1 = processed_metrics.horiBearingY;
        let y0 = y1 - processed_metrics.height;

        x_min = x_min.min(x0);
        x_max = x_max.max(x1);
        y_min = y_min.min(y0);
        y_max = y_max.max(y1);
        advance += processed_metrics.horiAdvance;
    }

    (x_min, x_max, y_min, y_max)
}

fn generate_text_texture(
    font: &Face,
    pixel_size: u32,
    text: &'static str,
) -> (GrayImage, u32, u32) {
    let freetype_pixel_range = get_pixel_range(font, pixel_size, text);

    let texture_width = (freetype_pixel_range.1 - freetype_pixel_range.0) as _;
    let texture_height = (freetype_pixel_range.3 - freetype_pixel_range.2) as _;

    let mut output_texture = GrayImage::new(texture_width, texture_height);

    let x_offset = -freetype_pixel_range.0;
    let y_offset = -freetype_pixel_range.2;

    let mut advance = 0;
    for ch in text.chars() {
        font.load_char(ch as _, LoadFlag::RENDER).unwrap();
        let glyph = font.glyph();
        let processed_metrics = CharMetrics::from_freetype_metrics(&glyph.metrics());
        println!("{:?}", processed_metrics);
        // character's information
        // let x0 = advance + processed_metrics.horiBearingX;
        // let x1 = x0 + processed_metrics.width;
        // let y1 = processed_metrics.horiBearingY;
        // let y0 = y1 - processed_metrics.height;

        let x0 = advance + processed_metrics.horiBearingX;
        let y0 = freetype_pixel_range.3 - processed_metrics.horiBearingY;

        // x_min = x_min.min(x0);
        // x_max = x_max.max(x1);
        // y_min = y_min.min(y0);
        // y_max = y_max.max(y1);
        // let bitmap = glyph.bitmap().buffer().as_ptr();
        for y in 0..processed_metrics.height {
            for x in 0..processed_metrics.width {
                let x_val = (x0 + x_offset + x) as u32;
                let y_val = (y0 + y) as u32;
                
                let current_pixel = 
                output_texture[(x_val, y_val)] = Luma {
                    0: [
                        output_texture[(x_val, y_val)].0[0]+ glyph.bitmap().buffer()[(y * processed_metrics.width + x) as usize], /* unsafe {
                                                                                                 *bitmap.add((y_val * processed_metrics.width as u32 + x_val) as usize)
                                                                                             } */
                    ],
                };
            }
        }
        advance += processed_metrics.horiAdvance;
    }

    (output_texture, texture_width, texture_height)
}

fn main() {
    let freetype_lib = freetype::Library::init().unwrap();
    let face = freetype_lib.new_face("cambriaz.ttf", 0).unwrap();

    // println!("{:?}", get_pixel_range(&face, 10, "fa"));

    let tex = generate_text_texture(&face, 100, "f(x)=10x^2");
    tex.0.save("Testing.jpg");
}
