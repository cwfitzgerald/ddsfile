mod pixel_format;
pub use self::pixel_format::{FourCC, PixelFormat, PixelFormatFlags};

mod d3d;
pub use self::d3d::D3DFormat;

mod dxgi;
pub use self::dxgi::DxgiFormat;

pub trait DataFormat {
    /// This gets the number of bytes required to store one row of data
    fn get_pitch(&self, width: u32) -> Option<u32>;

    /// This gets the height of each row of data. Normally it is 1, but for block
    /// compressed textures, each row is 4 pixels high.
    fn get_pitch_height(&self) -> u32 {
        if self.get_block_size().is_some() {
            4
        } else {
            1
        }
    }

    /// This gets the number of bits required to store a single pixel.  It is
    /// only defined for uncompressed formats
    fn get_bits_per_pixel(&self) -> Option<u8>;

    /// This gets a block compression format's block size, and is only defined
    /// for compressed formats
    fn get_block_size(&self) -> Option<u32>;

    /// Get the fourcc code for this format, if known
    fn get_fourcc(&self) -> Option<FourCC>;

    /// Returns true if the DX10 extention is required to use this format.
    fn requires_extension(&self) -> bool;

    /// This gets the minimum mipmap size in bytes. Even if they go all the way
    /// down to 1x1, there is a minimum number of bytes based on bits per pixel
    /// or blocksize.
    fn get_minimum_mipmap_size_in_bytes(&self) -> Option<u32> {
        if let Some(bpp) = self.get_bits_per_pixel() {
            Some((bpp as u32 + 7) / 8)
        } else {
            self.get_block_size()
        }
    }
}
