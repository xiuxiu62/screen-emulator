mod pixel;
mod screen;

use iced::{Application, Settings};
use screen::{Screen, ScreenOptions};

// fn main() {
//     let pixels = vec![
//         Pixel::new(0x00, 0x00, 0x00, 0x00),
//         Pixel::new(0xFF, 0xFF, 0xFF, 0x00),
//         Pixel::new(0xFF, 0x00, 0x00, 0x00),
//         Pixel::new(0x00, 0xFF, 0x00, 0x00),
//         Pixel::new(0x00, 0x00, 0xFF, 0x00),
//         Pixel::new(0x56, 0xA7, 0xC8, 0x00),
//         Pixel::default(),
//     ];

//     pixels.into_iter().for_each(|pixel| println!("{}", pixel));
// }

fn main() -> iced::Result {
    let flags = ScreenOptions::new(Some("test"), Some((32, 24)), None);
    let settings = Settings {
        antialiasing: true,
        flags,
        ..Settings::default()
    };

    Screen::run(settings)
}
