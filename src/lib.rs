//! A library for reading and writing Microsoft DirectDraw Surface (DDS) files.
//!
//! DDS is a container format for storing texture data, originally designed for DirectX
//! but widely used across graphics APIs (OpenGL, Vulkan, Metal) and asset pipelines.
//! This library handles the container envelope — parsing headers, computing layout
//! metadata, and providing access to the raw texture data. It does not decode or
//! encode pixel data.
//!
//! # Examples
//!
//! Reading a DDS file:
//!
//! ```no_run
//! use ddsfile::Dds;
//!
//! let file = std::fs::File::open("texture.dds").unwrap();
//! let dds = Dds::read(file).unwrap();
//!
//! println!("{}x{}", dds.get_width(), dds.get_height());
//! println!("Mipmaps: {}", dds.get_num_mipmap_levels());
//! println!("Format: {:?}", dds.get_dxgi_format());
//!
//! // Access the pixel data for the first (or only) layer
//! let data = dds.get_data(0).unwrap();
//! ```
//!
//! Creating and writing a DDS file:
//!
//! ```no_run
//! use ddsfile::{Dds, DxgiFormat, NewDxgiParams, D3D10ResourceDimension, AlphaMode};
//!
//! let dds = Dds::new_dxgi(NewDxgiParams {
//!     height: 256,
//!     width: 256,
//!     depth: None,
//!     format: DxgiFormat::BC7_UNorm_sRGB,
//!     mipmap_levels: Some(1),
//!     array_layers: None,
//!     caps2: None,
//!     is_cubemap: false,
//!     resource_dimension: D3D10ResourceDimension::Texture2D,
//!     alpha_mode: AlphaMode::Unknown,
//! }).unwrap();
//!
//! // Fill dds.data with your texture data, then write:
//! let mut file = std::fs::File::create("output.dds").unwrap();
//! dds.write(&mut file).unwrap();
//! ```
//!
//! # Format systems: D3D vs DXGI
//!
//! DDS files exist in two eras, and this library supports both:
//!
//! - **[`D3DFormat`]** — The original format system from Direct3D 9. Formats are
//!   identified by FourCC codes or RGB bitmasks stored in the [`PixelFormat`] struct
//!   within the [`Header`]. Supports ~44 formats including DXT1–DXT5 compression and
//!   common uncompressed layouts.
//!
//! - **[`DxgiFormat`]** — The modern format system introduced with DirectX 10. Supports
//!   80+ formats including sRGB variants, typeless formats, BC6H/BC7 compression, and
//!   video/planar formats. When a DDS file uses this system, the [`PixelFormat`] FourCC
//!   is set to `"DX10"` and an additional [`Header10`] follows the main [`Header`],
//!   carrying the [`DxgiFormat`] enum value directly.
//!
//! **If you're creating new DDS files, prefer [`Dds::new_dxgi`] unless you specifically
//! need compatibility with tools that only understand the legacy D3D format.**
//!
//! # Headers: Header vs Header10
//!
//! Every DDS file has a [`Header`] (124 bytes) containing dimensions, pitch/linear size,
//! mipmap count, pixel format, and capability flags. Files using [`DxgiFormat`] also have
//! a [`Header10`] (20 bytes) immediately after, which stores the DXGI format enum,
//! resource dimension, array size, and alpha mode. The presence of [`Header10`] is
//! signaled by `FourCC = "DX10"` in the header's pixel format.
//!
//! Most users should interact with the [`Dds`] struct directly rather than reading
//! headers manually — the getters handle both paths transparently.

#[cfg(test)]
mod tests;

mod error;
pub use error::*;

mod format;
pub use format::{D3DFormat, DataFormat, DxgiFormat, FourCC, PixelFormat, PixelFormatFlags};

mod header;
pub use header::{Caps, Caps2, Header, HeaderFlags};

mod header10;
pub use header10::{AlphaMode, D3D10ResourceDimension, Header10, MiscFlag};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::{Read, Write};

/// A parsed DDS (DirectDraw Surface) file.
///
/// Contains the mandatory [`Header`], an optional [`Header10`] (present for
/// [`DxgiFormat`] files), and the raw texture data. The header fields are public
/// for direct inspection, but prefer using the getter methods which handle both
/// D3D and DXGI paths transparently.
#[derive(Clone)]
pub struct Dds {
    // magic is implicit
    /// The mandatory 124-byte DDS header.
    pub header: Header,
    /// The optional DX10 extension header. Present when the file uses [`DxgiFormat`].
    pub header10: Option<Header10>,
    /// The raw texture data, including all array layers and mipmap levels.
    pub data: Vec<u8>,
}

/// Parameters for [`Dds::new_d3d`].
///
/// Use this when creating a DDS file with the legacy [`D3DFormat`] system.
/// Prefer [`NewDxgiParams`] for new files unless legacy compatibility is required.
#[derive(Debug, Clone)]
pub struct NewD3dParams {
    /// Texture height in pixels.
    pub height: u32,
    /// Texture width in pixels.
    pub width: u32,
    /// Depth for volume textures. `None` for 2D textures.
    pub depth: Option<u32>,
    /// The D3D pixel format.
    pub format: D3DFormat,
    /// Number of mipmap levels. `None` or `Some(1)` for no mipmaps.
    pub mipmap_levels: Option<u32>,
    /// Additional surface flags (e.g. [`Caps2::CUBEMAP`], [`Caps2::VOLUME`]).
    /// `None` for a simple 2D texture.
    pub caps2: Option<Caps2>,
}

/// Parameters for [`Dds::new_dxgi`].
///
/// Use this when creating a DDS file with the modern [`DxgiFormat`] system.
/// This is the preferred path for new files.
#[derive(Debug, Clone)]
pub struct NewDxgiParams {
    /// Texture height in pixels.
    pub height: u32,
    /// Texture width in pixels.
    pub width: u32,
    /// Depth for volume textures. `None` for 2D textures.
    pub depth: Option<u32>,
    /// The DXGI pixel format.
    pub format: DxgiFormat,
    /// Number of mipmap levels. `None` or `Some(1)` for no mipmaps.
    pub mipmap_levels: Option<u32>,
    /// Total number of array layers, **including cubemap faces**. For a single
    /// cubemap, pass `Some(6)`. For an array of 3 cubemaps, pass `Some(18)`.
    /// `None` for a non-array texture.
    ///
    /// This matches the value returned by [`Dds::get_num_array_layers`].
    pub array_layers: Option<u32>,
    /// Additional surface flags (e.g. [`Caps2::VOLUME`]). Cubemap flags are
    /// set automatically when `is_cubemap` is `true`. `None` for a simple 2D texture.
    pub caps2: Option<Caps2>,
    /// Whether this texture is a cubemap.
    pub is_cubemap: bool,
    /// The resource dimension (1D, 2D, or 3D texture).
    pub resource_dimension: D3D10ResourceDimension,
    /// How to interpret the alpha channel.
    pub alpha_mode: AlphaMode,
}

impl Dds {
    const MAGIC: u32 = 0x20534444; // b"DDS " in little endian

    /// Creates a new DDS file using the legacy [`D3DFormat`] system.
    ///
    /// The data buffer is allocated and zero-filled to the correct size for the
    /// given format, dimensions, and mipmap levels. Prefer [`Dds::new_dxgi`] for
    /// new files unless you need compatibility with tools that only understand D3D formats.
    pub fn new_d3d(params: NewD3dParams) -> Result<Dds, Error> {
        let mml = params.mipmap_levels.unwrap_or(1);
        let data_size = get_array_stride(
            params.width,
            params.height,
            params.depth,
            mml,
            &params.format,
        )
        .ok_or(Error::UnsupportedFormat)?;

        Ok(Dds {
            header: Header::new_d3d(
                params.height,
                params.width,
                params.depth,
                params.format,
                params.mipmap_levels,
                params.caps2,
            )?,
            header10: None,
            data: vec![0; data_size as usize],
        })
    }

    /// Creates a new DDS file using the modern [`DxgiFormat`] system.
    ///
    /// The data buffer is allocated and zero-filled to the correct size for the
    /// given format, dimensions, mipmap levels, and array layers. This produces
    /// a file with both a [`Header`] and a [`Header10`].
    pub fn new_dxgi(params: NewDxgiParams) -> Result<Dds, Error> {
        let arraysize = params.array_layers.unwrap_or(1);
        let mml = params.mipmap_levels.unwrap_or(1);
        let array_stride = get_array_stride(
            params.width,
            params.height,
            params.depth,
            mml,
            &params.format,
        )
        .ok_or(Error::UnsupportedFormat)?;

        let data_size = arraysize * array_stride;

        let arraysize = if params.is_cubemap {
            arraysize / 6
        } else {
            arraysize
        };
        let header10 = Header10::new(
            params.format,
            params.is_cubemap,
            params.resource_dimension,
            arraysize,
            params.alpha_mode,
        );

        Ok(Dds {
            header: Header::new_dxgi(
                params.height,
                params.width,
                params.depth,
                params.format,
                params.mipmap_levels,
                params.array_layers,
                params.caps2,
            )?,
            header10: Some(header10),
            data: vec![0; data_size as usize],
        })
    }

    /// Reads and parses a DDS file from any [`Read`] source.
    ///
    /// Automatically detects whether the file uses D3D or DXGI format by checking
    /// for the `"DX10"` FourCC code, and reads the [`Header10`] accordingly.
    /// All remaining bytes after the headers are stored as texture data.
    pub fn read<R: Read>(mut r: R) -> Result<Dds, Error> {
        let magic = r.read_u32::<LittleEndian>()?;
        if magic != Self::MAGIC {
            return Err(Error::BadMagicNumber);
        }

        let header = Header::read(&mut r)?;

        let header10 = if header.spf.fourcc == Some(FourCC(<FourCC>::DX10)) {
            Some(Header10::read(&mut r)?)
        } else {
            None
        };

        let mut data: Vec<u8> = Vec::new();
        r.read_to_end(&mut data)?;
        Ok(Dds {
            header,
            header10,
            data,
        })
    }

    /// Writes this DDS file to any [`Write`] destination.
    ///
    /// Writes the magic number, [`Header`], optional [`Header10`], and texture data.
    pub fn write<W: Write>(&self, w: &mut W) -> Result<(), Error> {
        w.write_u32::<LittleEndian>(Self::MAGIC)?;
        self.header.write(w)?;
        if let Some(ref header10) = self.header10 {
            header10.write(w)?;
        }
        w.write_all(&self.data)?;
        Ok(())
    }

    /// Returns the [`D3DFormat`] of this file, if it uses the legacy format system.
    ///
    /// Returns `None` for DX10-header files, even when the DXGI format has a D3D
    /// equivalent. This is a known limitation ([#6]).
    ///
    /// [#6]: https://github.com/cwfitzgerald/ddsfile/issues/6
    pub fn get_d3d_format(&self) -> Option<D3DFormat> {
        // FIXME(#6): some d3d formats are equivalent to some dxgi formats.
        //    but we dont have a try_from() between them yet.
        //    Right now we will yield None if the format is dxgi, but
        //    later on we should try to convert.

        D3DFormat::try_from_pixel_format(&self.header.spf)
    }

    /// Returns the [`DxgiFormat`] of this file, if it uses the modern format system.
    ///
    /// Returns `None` for legacy D3D-header files, even when the D3D format has a
    /// DXGI equivalent. This is a known limitation ([#6]).
    ///
    /// For legacy files that use FourCC codes which map directly to DXGI formats
    /// (e.g. `DXT1` → `BC1_UNorm_sRGB`), this method _will_ return the DXGI
    /// equivalent. It only fails for bitmask-identified D3D formats.
    ///
    /// [#6]: https://github.com/cwfitzgerald/ddsfile/issues/6
    pub fn get_dxgi_format(&self) -> Option<DxgiFormat> {
        // FIXME(#6): some d3d formats are equivalent to some dxgi formats.
        //    but we dont have a try_from() between them yet.
        //    Right now we will yield None if the format is d3d, but
        //    later on we should try to convert.
        if let Some(ref h10) = self.header10 {
            Some(h10.dxgi_format)
        } else {
            DxgiFormat::try_from_pixel_format(&self.header.spf)
        }
    }

    /// Returns the format as a type-erased [`DataFormat`] trait object.
    ///
    /// Tries DXGI first, then falls back to D3D. Returns `None` only if neither
    /// format system can identify the file's format.
    pub fn get_format(&self) -> Option<Box<dyn DataFormat>> {
        if let Some(dxgi) = self.get_dxgi_format() {
            Some(Box::new(dxgi))
        } else if let Some(d3d) = self.get_d3d_format() {
            Some(Box::new(d3d))
        } else {
            None
        }
    }

    /// Returns the texture width in pixels.
    pub fn get_width(&self) -> u32 {
        self.header.width
    }

    /// Returns the texture height in pixels.
    pub fn get_height(&self) -> u32 {
        self.header.height
    }

    /// Returns the depth of a volume texture, or `1` for non-volume textures.
    ///
    /// A return value of `1` means the texture is not a volume/3D texture.
    /// Graphics APIs treat depth=1 as a standard 2D texture.
    pub fn get_depth(&self) -> u32 {
        self.header.depth.unwrap_or(1)
    }

    /// Returns the number of bits per pixel for uncompressed formats.
    ///
    /// Returns `None` for block-compressed formats (BC1–BC7, DXT1–DXT5) and for
    /// planar YUV formats (`NV12`, `P010`, `P016`, `Format_420_Opaque`, `NV11`,
    /// `P208`, `V208`, `V408`). For block-compressed formats, use
    /// [`DataFormat::get_block_size`] instead.
    ///
    /// Non-planar YUV formats (`YUY2`, `AYUV`, `Y410`, etc.) and depth-stencil
    /// formats (`D32_Float`, `D24_UNorm_S8_UInt`, etc.) _do_ return a value.
    pub fn get_bits_per_pixel(&self) -> Option<u32> {
        // Try format first
        if let Some(format) = self.get_format() {
            if let Some(bpp) = format.get_bits_per_pixel() {
                return Some(bpp as u32);
            }
        }
        // Fall back to pixel_format rgb_bit_count field
        if let Some(bpp) = self.header.spf.rgb_bit_count {
            return Some(bpp);
        }
        None
    }

    /// Returns the number of bytes per row of data.
    ///
    /// For uncompressed formats, this is the number of bytes per scanline.
    /// For block-compressed formats, this is the number of bytes per row of
    /// _blocks_ (where each block covers a 4×4 pixel region). See
    /// [`get_pitch_height`](Self::get_pitch_height) for the number of pixel rows
    /// per pitch unit.
    ///
    /// Returns `None` for planar YUV formats (`NV12`, `P010`, `P016`,
    /// `Format_420_Opaque`, `NV11`, `P208`, `V208`, `V408`) where a single pitch
    /// value cannot describe the multi-plane layout, for sentinel formats
    /// (`Unknown`, `Force_UInt`), and for files with completely unrecognized
    /// pixel formats.
    pub fn get_pitch(&self) -> Option<u32> {
        // Try format first
        if let Some(format) = self.get_format() {
            if let Some(pitch) = format.get_pitch(self.header.width) {
                return Some(pitch);
            }
        }
        // Then try header.pitch
        if let Some(pitch) = self.header.pitch {
            return Some(pitch);
        }

        // Then try to calculate it ourselves
        if let Some(bpp) = self.get_bits_per_pixel() {
            return Some((bpp * self.get_width()).div_ceil(8));
        }
        None
    }

    /// Returns the number of pixel rows per pitch unit.
    ///
    /// Returns `4` for block-compressed formats (each block row covers 4 pixel
    /// rows), and `1` for everything else.
    pub fn get_pitch_height(&self) -> u32 {
        if let Some(format) = self.get_format() {
            format.get_pitch_height()
        } else {
            1
        }
    }

    /// Returns the size in bytes of the top-level (largest) mip of a single
    /// array layer.
    pub fn get_main_texture_size(&self) -> Option<u32> {
        get_texture_size(
            self.get_pitch(),
            self.header.linear_size,
            self.get_pitch_height(),
            self.header.height,
            self.header.depth,
        )
    }

    /// Returns the total size in bytes of one array layer's full mipmap chain.
    ///
    /// This is the stride between consecutive array layers in the data buffer.
    /// For a texture with no mipmaps, this equals the main texture size.
    pub fn get_array_stride(&self) -> Result<u32, Error> {
        let format = self.get_format().ok_or(Error::UnsupportedFormat)?;
        get_array_stride(
            self.header.width,
            self.header.height,
            self.header.depth,
            self.get_num_mipmap_levels(),
            &*format,
        )
        .ok_or(Error::UnsupportedFormat)
    }

    /// Returns the total number of array layers, **including cubemap faces**.
    ///
    /// - For a simple 2D texture: returns `1`.
    /// - For a legacy (D3D) cubemap: returns `6`.
    /// - For a DXGI cubemap: returns `array_size * 6`.
    /// - For a DXGI non-cubemap array: returns `array_size`.
    ///
    /// This value matches the range of valid `array_layer` indices for
    /// [`get_data`](Self::get_data) (i.e. `0..get_num_array_layers()`).
    pub fn get_num_array_layers(&self) -> u32 {
        if let Some(ref h10) = self.header10 {
            if h10.misc_flag.contains(MiscFlag::TEXTURECUBE) {
                h10.array_size * 6
            } else {
                h10.array_size
            }
        } else if self.header.caps2.contains(Caps2::CUBEMAP) {
            6
        } else {
            1 // just the 1 layer
        }
    }

    /// Returns the number of mipmap levels. Returns `1` if no mipmaps are present
    /// (the base level always counts).
    pub fn get_num_mipmap_levels(&self) -> u32 {
        self.header.mip_map_count.unwrap_or(1)
    }

    /// Returns the minimum size in bytes of any single mipmap level.
    ///
    /// Even the smallest mip (e.g. 1×1) occupies at least this many bytes.
    /// For block-compressed formats this equals the block size (8 or 16 bytes);
    /// for uncompressed formats it equals the minimum number of bytes for one pixel.
    pub fn get_min_mipmap_size_in_bytes(&self) -> u32 {
        if let Some(format) = self.get_format() {
            if let Some(min) = format.get_minimum_mipmap_size_in_bytes() {
                return min;
            }
        }
        if let Some(bpp) = self.get_bits_per_pixel() {
            bpp.div_ceil(8)
        } else {
            1
        }
    }

    /// Returns a reference to the data for the given array layer.
    ///
    /// Pass `0` for textures with a single image. Valid indices are
    /// `0..`[`get_num_array_layers()`](Self::get_num_array_layers). Each layer's
    /// data includes all mipmap levels packed contiguously.
    pub fn get_data(&self, array_layer: u32) -> Result<&[u8], Error> {
        let (offset, size) = self.get_offset_and_size(array_layer)?;
        let offset = offset as usize;
        let size = size as usize;
        self.data
            .get(offset..offset + size)
            .ok_or(Error::OutOfBounds)
    }

    /// Returns a mutable reference to the data for the given array layer.
    ///
    /// See [`get_data`](Self::get_data) for details on array layer indexing.
    pub fn get_mut_data(&mut self, array_layer: u32) -> Result<&mut [u8], Error> {
        let (offset, size) = self.get_offset_and_size(array_layer)?;
        let offset = offset as usize;
        let size = size as usize;
        self.data
            .get_mut(offset..offset + size)
            .ok_or(Error::OutOfBounds)
    }

    fn get_offset_and_size(&self, array_layer: u32) -> Result<(u32, u32), Error> {
        // Verify request bounds
        if array_layer >= self.get_num_array_layers() {
            return Err(Error::OutOfBounds);
        }
        let array_stride = self.get_array_stride()?;
        let offset = array_layer * array_stride;

        Ok((offset, array_stride))
    }
}

fn get_texture_size(
    pitch: Option<u32>,
    linear_size: Option<u32>,
    pitch_height: u32,
    height: u32,
    depth: Option<u32>,
) -> Option<u32> {
    let depth = depth.unwrap_or(1);

    if let Some(ls) = linear_size {
        Some(ls)
    } else if let Some(pitch) = pitch {
        let row_height = height.div_ceil(pitch_height);
        Some(pitch * row_height * depth)
    } else {
        None
    }
}

fn get_array_stride(
    width: u32,
    height: u32,
    depth: Option<u32>,
    mipmap_levels: u32,
    format: &dyn DataFormat,
) -> Option<u32> {
    let mut stride: u32 = 0;
    let mut mip_width = width;
    let mut mip_height = height;
    let mut mip_depth = depth.unwrap_or(1);
    let pitch_height = format.get_pitch_height();

    for _ in 0..mipmap_levels {
        let pitch = format.get_pitch(mip_width)?;
        let row_height = mip_height.div_ceil(pitch_height);
        stride += pitch * row_height * mip_depth;

        mip_width = (mip_width / 2).max(1);
        mip_height = (mip_height / 2).max(1);
        mip_depth = (mip_depth / 2).max(1);
    }

    Some(stride)
}

impl fmt::Debug for Dds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Dds:")?;
        if let Some(d3dformat) = self.get_d3d_format() {
            writeln!(f, "  Format: {:?}", d3dformat)?;
        } else if let Some(dxgiformat) = self.get_dxgi_format() {
            writeln!(f, "  Format: {:?}", dxgiformat)?;
        } else if let Some(ref fourcc) = self.header.spf.fourcc {
            writeln!(f, "  Format: FOURCC={:?} (Unknown)", fourcc)?;
        } else {
            writeln!(f, "  Format UNSPECIFIED")?;
        }
        write!(f, "{:?}", self.header)?;
        if let Some(ref h10) = self.header10 {
            write!(f, "{:?}", h10)?;
        }
        writeln!(f, "  (data elided)")?;
        Ok(())
    }
}
