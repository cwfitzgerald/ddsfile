use bitflags::bitflags;

use crate::error::*;
use crate::format::DxgiFormat;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use enum_primitive_derive::Primitive;
use num_traits::FromPrimitive;
use std::fmt;
use std::io::{Read, Write};

/// The type of resource stored in the DDS file.
///
/// Used in [`Header10::resource_dimension`] and in
/// [`NewDxgiParams::resource_dimension`](crate::NewDxgiParams::resource_dimension).
/// Cubemaps use [`Texture2D`](Self::Texture2D).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Primitive)]
pub enum D3D10ResourceDimension {
    /// Resource type is unknown.
    Unknown = 0,
    /// Buffer resource (not typically used in DDS files).
    Buffer = 1,
    /// 1D texture.
    Texture1D = 2,
    /// 2D texture. Also used for cubemaps.
    Texture2D = 3,
    /// 3D (volume) texture.
    Texture3D = 4,
}

/// The DX10 extension header (20 bytes), present in DDS files that use
/// [`DxgiFormat`](crate::DxgiFormat).
///
/// Carries the actual DXGI format enum, resource dimension, array size, and
/// alpha mode. Its presence is signaled by `FourCC = "DX10"` in the main
/// header's [`PixelFormat`](crate::PixelFormat).
#[derive(Clone)]
pub struct Header10 {
    /// The DXGI pixel format.
    pub dxgi_format: DxgiFormat,
    /// The type of resource (1D, 2D, 3D texture).
    pub resource_dimension: D3D10ResourceDimension,
    /// Miscellaneous flags. Currently only [`MiscFlag::TEXTURECUBE`] is defined.
    pub misc_flag: MiscFlag,
    /// Number of array elements. For cubemaps, this is the number of _cubemaps_
    /// (not faces) — each cubemap has 6 faces. For non-array textures this is `1`.
    pub array_size: u32,
    /// How to interpret the alpha channel. Called `misc_flags2` in the official
    /// Microsoft documentation.
    pub alpha_mode: AlphaMode,
}

impl fmt::Debug for Header10 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "  Header10:")?;
        writeln!(f, "    dxgi_format: {:?}", self.dxgi_format)?;
        writeln!(f, "    resource_dimension: {:?}", self.resource_dimension)?;
        writeln!(f, "    misc_flag: {:?}", self.misc_flag)?;
        writeln!(f, "    array_size: {:?}", self.array_size)?;
        write!(f, "    alpha_mode: {:?}", self.alpha_mode)?;
        Ok(())
    }
}

impl Default for Header10 {
    fn default() -> Header10 {
        Header10 {
            dxgi_format: DxgiFormat::Unknown,
            resource_dimension: D3D10ResourceDimension::Unknown,
            misc_flag: MiscFlag::empty(),
            array_size: 0,
            alpha_mode: AlphaMode::Unknown,
        }
    }
}

impl Header10 {
    pub fn new(
        format: DxgiFormat,
        is_cubemap: bool,
        resource_dimension: D3D10ResourceDimension,
        array_size: u32,
        alpha_mode: AlphaMode,
    ) -> Header10 {
        let mut flags = MiscFlag::empty();
        if is_cubemap {
            flags |= MiscFlag::TEXTURECUBE
        };
        Header10 {
            dxgi_format: format,
            resource_dimension,
            misc_flag: flags,
            array_size,
            alpha_mode,
        }
    }

    pub fn read<R: Read>(mut r: R) -> Result<Header10, Error> {
        let dxgi_format = r.read_u32::<LittleEndian>()?;
        let resource_dimension = r.read_u32::<LittleEndian>()?;
        let misc_flag = MiscFlag::from_bits_truncate(r.read_u32::<LittleEndian>()?);
        let array_size = r.read_u32::<LittleEndian>()?;
        let alpha_mode = r.read_u32::<LittleEndian>()?;

        let dxgi_format_result: Result<DxgiFormat, Error> = DxgiFormat::from_u32(dxgi_format)
            .ok_or_else(|| Error::InvalidField("dxgi_format".to_owned()));
        let resource_dimension_result: Result<D3D10ResourceDimension, Error> =
            D3D10ResourceDimension::from_u32(resource_dimension)
                .ok_or_else(|| Error::InvalidField("resource_dimension".to_owned()));

        let alpha_mode: Result<AlphaMode, Error> = AlphaMode::from_u32(alpha_mode)
            .ok_or_else(|| Error::InvalidField("alpha mode (misc_flags2)".to_owned()));

        Ok(Header10 {
            dxgi_format: dxgi_format_result?,
            resource_dimension: resource_dimension_result?,
            misc_flag,
            array_size,
            alpha_mode: alpha_mode?,
        })
    }

    pub fn write<W: Write>(&self, w: &mut W) -> Result<(), Error> {
        w.write_u32::<LittleEndian>(self.dxgi_format as u32)?;
        w.write_u32::<LittleEndian>(self.resource_dimension as u32)?;
        w.write_u32::<LittleEndian>(self.misc_flag.bits())?;
        w.write_u32::<LittleEndian>(self.array_size)?;
        w.write_u32::<LittleEndian>(self.alpha_mode as u32)?;
        Ok(())
    }
}

bitflags! {
    /// Miscellaneous resource flags for [`Header10`].
    ///
    /// Currently only [`TEXTURECUBE`](Self::TEXTURECUBE) is defined by the
    /// specification. Set automatically when `is_cubemap` is `true` in
    /// [`NewDxgiParams`](crate::NewDxgiParams).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct MiscFlag: u32 {
        /// Indicates the texture is a cubemap.
        const TEXTURECUBE = 0x4;
    }
}

/// Describes how to interpret the alpha channel in a DDS texture.
///
/// Used in [`Header10::alpha_mode`] and
/// [`NewDxgiParams::alpha_mode`](crate::NewDxgiParams::alpha_mode).
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Primitive)]
pub enum AlphaMode {
    /// Alpha behavior is unspecified.
    Unknown = 0x0,
    /// Alpha is straight (non-premultiplied).
    Straight = 0x1,
    /// Alpha is premultiplied into the color channels.
    PreMultiplied = 0x2,
    /// Alpha channel should be ignored; the texture is fully opaque.
    Opaque = 0x3,
    /// Alpha is application-defined.
    Custom = 0x4,
}
