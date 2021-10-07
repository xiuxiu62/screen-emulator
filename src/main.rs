#![feature(generic_associated_types)]

mod pixel;
mod screen;

use iced::{window, Application, Settings};
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

fn main() {
    // let screen = Screen::new(options);
    let settings = Settings {
        antialiasing: true,
        flags: ScreenOptions::new(Some("test"), Some((32, 24)), None),
        ..Settings::default()
    };

    Screen::run(settings);
}
