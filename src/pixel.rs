use std::fmt::Display;

//          (R,  G,  B,  A)
#[derive(Clone, Copy, Debug, Default)]
pub struct Pixel(u8, u8, u8, u8);
impl Pixel {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(r, g, b, a)
    }

    pub fn inner(&self) -> (u8, u8, u8, u8) {
        (self.0, self.1, self.2, self.3)
    }

    pub fn name(&self) -> Option<&str> {
        match self.inner() {
            (0x00, 0x00, 0x00, 0x00) => Some("White"),
            (0xFF, 0xFF, 0xFF, 0x00) => Some("Black"),
            (0xFF, 0x00, 0x00, 0x00) => Some("Red"),
            (0x00, 0xFF, 0x00, 0x00) => Some("Green"),
            (0x00, 0x00, 0xFF, 0x00) => Some("Blue"),
            _ => None,
        }
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.name().unwrap_or("Unknown");
        let (r, g, b, a) = self.inner();
        write!(f, "{} -> <{} {} {} {}>", name, r, g, b, a)
    }
}

impl<'a> IntoIterator for &'a Pixel {
    type Item = u8;
    type IntoIter = PixelIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PixelIntoIterator {
            color: self,
            index: 0,
        }
    }
}

pub struct PixelIntoIterator<'a> {
    color: &'a Pixel,
    index: usize,
}

impl<'a> Iterator for PixelIntoIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => self.color.0,
            1 => self.color.1,
            2 => self.color.2,
            3 => self.color.3,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}
