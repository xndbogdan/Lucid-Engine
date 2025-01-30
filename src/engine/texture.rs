use image::GenericImageView;
use std::path::Path;

#[derive(Clone)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pixels: Vec<u32>,
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

        // Convert to RGBA8
        let img = img.to_rgba8();
        let (width, height) = img.dimensions();
        let mut pixels = Vec::with_capacity((width * height) as usize);

        // Convert to u32 RGBA format for faster rendering
        for pixel in img.pixels() {
            let r = pixel[0] as u32;
            let g = pixel[1] as u32;
            let b = pixel[2] as u32;
            let a = pixel[3] as u32;
            pixels.push((r << 24) | (g << 16) | (b << 8) | a);
        }

        Ok(Self {
            width,
            height,
            pixels,
        })
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> u32 {
        if x >= self.width || y >= self.height {
            return 0; // Return transparent black for out of bounds
        }
        self.pixels[(y * self.width + x) as usize]
    }

    pub fn get_pixel_scaled(&self, x: f32, y: f32) -> u32 {
        // Scale x and y to texture coordinates
        let tx = ((x * self.width as f32) as u32) % self.width;
        let ty = ((y * self.height as f32) as u32) % self.height;
        self.get_pixel(tx, ty)
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: u32) {
        if x < self.width && y < self.height {
            self.pixels[(y * self.width + x) as usize] = color;
        }
    }
}

pub struct TextureCache {
    textures: Vec<Texture>,
}

impl TextureCache {
    pub fn new() -> Self {
        Self {
            textures: Vec::new(),
        }
    }

    pub fn load_texture<P: AsRef<Path>>(&mut self, path: P) -> Result<usize, String> {
        let texture = Texture::load(path)?;
        let index = self.textures.len();
        self.textures.push(texture);
        Ok(index)
    }

    pub fn get_texture(&self, index: usize) -> Option<&Texture> {
        self.textures.get(index)
    }

    pub fn clear(&mut self) {
        self.textures.clear();
    }
}

impl Default for TextureCache {
    fn default() -> Self {
        Self::new()
    }
}
