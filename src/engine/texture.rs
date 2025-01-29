use image::GenericImageView;
use std::path::Path;

pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u32>,
}

impl Texture {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![0; (width * height) as usize],
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let img = image::open(path).map_err(|e| format!("Failed to load texture: {}", e))?;

        let (width, height) = img.dimensions();
        let mut texture = Self::new(width, height);

        // Convert image to RGBA
        let rgba = img.to_rgba8();

        // Convert to u32 pixels (RGBA)
        for (i, pixel) in rgba.pixels().enumerate() {
            let [r, g, b, a] = pixel.0;
            texture.pixels[i] = u32::from_be_bytes([r, g, b, a]);
        }

        Ok(texture)
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> u32 {
        if x < self.width && y < self.height {
            self.pixels[(y * self.width + x) as usize]
        } else {
            0
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: u32) {
        if x < self.width && y < self.height {
            self.pixels[(y * self.width + x) as usize] = color;
        }
    }
}
