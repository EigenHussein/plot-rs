#[cfg(target_os = "windows")]
static FONT_PATH: &'static str = "C:\\Users\\Haloc\\Dev\\Rust\\plot-rs\\cambriaz.ttf";
#[cfg(target_os = "linux")]
static FONT_PATH: &'static str = "/home/hussein/Dev/Rust/plotters/cambriaz.ttf";

use std::{
    alloc::{alloc, Layout},
    time::{Duration, Instant},
};

use freetype::{self, face::LoadFlag, Face, GlyphMetrics, GlyphSlot};

struct FreeTypeCharacter {
    character: char,
    bitmap: Vec<u8>,
    glyph: GlyphMetrics,
}
impl FreeTypeCharacter {
    pub fn new_character(face: &Face, size: u32, character: char) -> Self {
        face.set_pixel_sizes(0, size);
        face.load_char(character as _, LoadFlag::RENDER);
        let glyph = face.glyph();
        let metrics = glyph.metrics();
        Self {
            character: character,
            bitmap: glyph.bitmap().buffer().to_vec(),
            glyph: glyph.metrics(),
        }
    }
}

fn main() {
    let freetype_library = freetype::library::Library::init().unwrap();
    let face = freetype_library.new_face(FONT_PATH, 0).unwrap();

    let a = Instant::now();
    let chars: Vec<FreeTypeCharacter> = (0..256)
        .into_iter()
        .map(|i| FreeTypeCharacter::new_character(&face, 1000, char::from_u32(i).unwrap()))
        .collect();

    println!("{:?}", chars['A' as usize].glyph.height >> 6);

    let b = Instant::now();

    println!("{:?}", (b - a))

    // face.set_char_size(0, 100 << 6, 0, 100).unwrap();
    // face.load_char('H' as _, LoadFlag::DEFAULT).unwrap();

    // let myglyph = face.glyph();
    // println!("{:?}", myglyph.metrics())
}
