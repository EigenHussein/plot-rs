
use freetype::{self, face::LoadFlag, Face};

fn main(){
    let freetype_library = freetype::library::Library::init().unwrap();
    let face = freetype_library.new_face("C:/Users/Haloc/Downloads/cambria.ttc", 0).unwrap();
    face.set_char_size(0, 100 << 6 , 0, 100).unwrap();
    face.load_char('H' as _, LoadFlag::DEFAULT).unwrap();
    
    let myglyph = face.glyph();
    println!("{:?}",myglyph.metrics())
}