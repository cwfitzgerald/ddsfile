mod pixel_format;
pub use self::pixel_format::{FourCC, PixelFormat, PixelFormatFlags};

mod d3d;
pub use self::d3d::D3DFormat;

mod dxgi;
pub use self::dxgi::DxgiFormat;

/// Common interface for querying format metadata from both [`D3DFormat`] and
/// [`DxgiFormat`].
///
/// Most users don't need to work with this trait directly — the methods on
/// [`Dds`](crate::Dds) delegate to it internally. It is useful when you need
/// to compute layout information for a format independently of a specific file
/// (e.g. to predict buffer sizes before creating a [`Dds`](crate::Dds)).
pub trait DataFormat {
    /// Returns the number of bytes per row of data at the given `width`.
    ///
    /// For uncompressed formats, this is bytes per scanline. For block-compressed
    /// formats, this is bytes per row of blocks (each block covering 4 pixels wide).
    fn get_pitch(&self, width: u32) -> Option<u32>;

    /// Returns the number of pixel rows per pitch row.
    ///
    /// Returns `4` for block-compressed formats (each block row covers 4 pixel
    /// rows), and `1` for everything else.
    fn get_pitch_height(&self) -> u32 {
        if self.get_block_size().is_some() {
            4
        } else {
            1
        }
    }

    /// Returns the bits per pixel for uncompressed formats, or `None` for
    /// block-compressed and planar formats. See [`Dds::get_bits_per_pixel`](crate::Dds::get_bits_per_pixel)
    /// for details on which formats return `None`.
    fn get_bits_per_pixel(&self) -> Option<u8>;

    /// Returns the block size in bytes for block-compressed formats, or `None` for
    /// uncompressed formats.
    ///
    /// BC1 and BC4 have a block size of 8 bytes; BC2, BC3, BC5, BC6H, and BC7
    /// have a block size of 16 bytes.
    fn get_block_size(&self) -> Option<u32>;

    /// Returns the FourCC code for this format, if one exists.
    fn get_fourcc(&self) -> Option<FourCC>;

    /// Returns `true` if this format requires the DX10 extension header
    /// ([`Header10`](crate::Header10)).
    fn requires_extension(&self) -> bool;

    /// Returns the minimum size in bytes of any single mipmap level.
    ///
    /// Even a 1×1 mip occupies at least this many bytes. For block-compressed
    /// formats this equals the block size; for uncompressed formats it equals
    /// the bytes per pixel (rounded up).
    fn get_minimum_mipmap_size_in_bytes(&self) -> Option<u32> {
        if let Some(bpp) = self.get_bits_per_pixel() {
            Some((bpp as u32).div_ceil(8))
        } else {
            self.get_block_size()
        }
    }
}
