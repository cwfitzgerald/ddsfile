use super::pixel_format::{FourCC, PixelFormat};
use super::DataFormat;
use enum_primitive_derive::Primitive;

/// Pixel formats from the DirectX Graphics Infrastructure (DXGI), introduced
/// with DirectX 10.
///
/// Variant names encode the channel layout, bit depth, and data type:
/// - Channel letters: `R`, `G`, `B`, `A` (color), `D` (depth), `S` (stencil),
///   `X` (unused bits).
/// - Suffixes: `UNorm` (unsigned normalized), `SNorm` (signed normalized),
///   `UInt`, `SInt`, `Float`, `Typeless` (no interpretation), `sRGB`.
/// - `BC1`–`BC7`: Block-compressed formats (BC1/BC4 = 8 bytes/block,
///   BC2/BC3/BC5/BC6H/BC7 = 16 bytes/block).
///
/// Files using this format system have a [`Header10`](crate::Header10)
/// extension. Use [`Dds::new_dxgi`](crate::Dds::new_dxgi) to create them.
#[rustfmt::skip]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Primitive)]
#[repr(u32)]
pub enum DxgiFormat {
    Unknown                     = 0,
    R32G32B32A32_Typeless       = 1,
    R32G32B32A32_Float          = 2,
    R32G32B32A32_UInt           = 3,
    R32G32B32A32_SInt           = 4,
    R32G32B32_Typeless          = 5,
    R32G32B32_Float             = 6,
    R32G32B32_UInt              = 7,
    R32G32B32_SInt              = 8,
    R16G16B16A16_Typeless       = 9,
    R16G16B16A16_Float          = 10,
    R16G16B16A16_UNorm          = 11,
    R16G16B16A16_UInt           = 12,
    R16G16B16A16_SNorm          = 13,
    R16G16B16A16_SInt           = 14,
    R32G32_Typeless             = 15,
    R32G32_Float                = 16,
    R32G32_UInt                 = 17,
    R32G32_SInt                 = 18,
    R32G8X24_Typeless           = 19,
    D32_Float_S8X24_UInt        = 20,
    R32_Float_X8X24_Typeless    = 21,
    X32_Typeless_G8X24_UInt     = 22,
    R10G10B10A2_Typeless        = 23,
    R10G10B10A2_UNorm           = 24,
    R10G10B10A2_UInt            = 25,
    R11G11B10_Float             = 26,
    R8G8B8A8_Typeless           = 27,
    R8G8B8A8_UNorm              = 28,
    R8G8B8A8_UNorm_sRGB         = 29,
    R8G8B8A8_UInt               = 30,
    R8G8B8A8_SNorm              = 31,
    R8G8B8A8_SInt               = 32,
    R16G16_Typeless             = 33,
    R16G16_Float                = 34,
    R16G16_UNorm                = 35,
    R16G16_UInt                 = 36,
    R16G16_SNorm                = 37,
    R16G16_SInt                 = 38,
    R32_Typeless                = 39,
    D32_Float                   = 40,
    R32_Float                   = 41,
    R32_UInt                    = 42,
    R32_SInt                    = 43,
    R24G8_Typeless              = 44,
    D24_UNorm_S8_UInt           = 45,
    R24_UNorm_X8_Typeless       = 46,
    X24_Typeless_G8_UInt        = 47,
    R8G8_Typeless               = 48,
    R8G8_UNorm                  = 49,
    R8G8_UInt                   = 50,
    R8G8_SNorm                  = 51,
    R8G8_SInt                   = 52,
    R16_Typeless                = 53,
    R16_Float                   = 54,
    D16_UNorm                   = 55,
    R16_UNorm                   = 56,
    R16_UInt                    = 57,
    R16_SNorm                   = 58,
    R16_SInt                    = 59,
    R8_Typeless                 = 60,
    R8_UNorm                    = 61,
    R8_UInt                     = 62,
    R8_SNorm                    = 63,
    R8_SInt                     = 64,
    A8_UNorm                    = 65,
    R1_UNorm                    = 66,
    R9G9B9E5_SharedExp          = 67,
    R8G8_B8G8_UNorm             = 68,
    G8R8_G8B8_UNorm             = 69,
    BC1_Typeless                = 70,
    BC1_UNorm                   = 71,
    BC1_UNorm_sRGB              = 72,
    BC2_Typeless                = 73,
    BC2_UNorm                   = 74,
    BC2_UNorm_sRGB              = 75,
    BC3_Typeless                = 76,
    BC3_UNorm                   = 77,
    BC3_UNorm_sRGB              = 78,
    BC4_Typeless                = 79,
    BC4_UNorm                   = 80,
    BC4_SNorm                   = 81,
    BC5_Typeless                = 82,
    BC5_UNorm                   = 83,
    BC5_SNorm                   = 84,
    B5G6R5_UNorm                = 85,
    B5G5R5A1_UNorm              = 86,
    B8G8R8A8_UNorm              = 87,
    B8G8R8X8_UNorm              = 88,
    R10G10B10_XR_Bias_A2_UNorm  = 89,
    B8G8R8A8_Typeless           = 90,
    B8G8R8A8_UNorm_sRGB         = 91,
    B8G8R8X8_Typeless           = 92,
    B8G8R8X8_UNorm_sRGB         = 93,
    BC6H_Typeless               = 94,
    BC6H_UF16                   = 95,
    BC6H_SF16                   = 96,
    BC7_Typeless                = 97,
    BC7_UNorm                   = 98,
    BC7_UNorm_sRGB              = 99,
    AYUV                        = 100,
    Y410                        = 101,
    Y416                        = 102,
    NV12                        = 103,
    P010                        = 104,
    P016                        = 105,
    Format_420_Opaque           = 106,
    YUY2                        = 107,
    Y210                        = 108,
    Y216                        = 109,
    NV11                        = 110,
    AI44                        = 111,
    IA44                        = 112,
    P8                          = 113,
    A8P8                        = 114,
    B4G4R4A4_UNorm              = 115,
    P208                        = 130,
    V208                        = 131,
    V408                        = 132,

    // Non-standard ASTC formats. These are not part of the official DXGI
    // specification but are used by NVIDIA Texture Tools and other NVIDIA
    // software to store ASTC-compressed textures in DDS files. Each block
    // size has Typeless, UNorm, and UNorm_sRGB variants. All ASTC blocks
    // are 128 bits (16 bytes) regardless of block footprint.
    ASTC_4x4_Typeless           = 133,
    ASTC_4x4_UNorm              = 134,
    ASTC_4x4_UNorm_sRGB         = 135,
    ASTC_5x4_Typeless           = 137,
    ASTC_5x4_UNorm              = 138,
    ASTC_5x4_UNorm_sRGB         = 139,
    ASTC_5x5_Typeless           = 141,
    ASTC_5x5_UNorm              = 142,
    ASTC_5x5_UNorm_sRGB         = 143,
    ASTC_6x5_Typeless           = 145,
    ASTC_6x5_UNorm              = 146,
    ASTC_6x5_UNorm_sRGB         = 147,
    ASTC_6x6_Typeless           = 149,
    ASTC_6x6_UNorm              = 150,
    ASTC_6x6_UNorm_sRGB         = 151,
    ASTC_8x5_Typeless           = 153,
    ASTC_8x5_UNorm              = 154,
    ASTC_8x5_UNorm_sRGB         = 155,
    ASTC_8x6_Typeless           = 157,
    ASTC_8x6_UNorm              = 158,
    ASTC_8x6_UNorm_sRGB         = 159,
    ASTC_8x8_Typeless           = 161,
    ASTC_8x8_UNorm              = 162,
    ASTC_8x8_UNorm_sRGB         = 163,
    ASTC_10x5_Typeless          = 165,
    ASTC_10x5_UNorm             = 166,
    ASTC_10x5_UNorm_sRGB        = 167,
    ASTC_10x6_Typeless          = 169,
    ASTC_10x6_UNorm             = 170,
    ASTC_10x6_UNorm_sRGB        = 171,
    ASTC_10x8_Typeless          = 173,
    ASTC_10x8_UNorm             = 174,
    ASTC_10x8_UNorm_sRGB        = 175,
    ASTC_10x10_Typeless         = 177,
    ASTC_10x10_UNorm            = 178,
    ASTC_10x10_UNorm_sRGB       = 179,
    ASTC_12x10_Typeless         = 181,
    ASTC_12x10_UNorm            = 182,
    ASTC_12x10_UNorm_sRGB       = 183,
    ASTC_12x12_Typeless         = 185,
    ASTC_12x12_UNorm            = 186,
    ASTC_12x12_UNorm_sRGB       = 187,

    Force_UInt                  = 0xffffffff_u32
}

impl DataFormat for DxgiFormat {
    fn get_pitch(&self, width: u32) -> Option<u32> {
        // see https://msdn.microsoft.com/en-us/library/bb943991.aspx
        match *self {
            DxgiFormat::R8G8_B8G8_UNorm | DxgiFormat::G8R8_G8B8_UNorm => {
                return Some(((width + 1) >> 1) * 4);
            }
            _ => {}
        };

        if let Some(bpp) = self.get_bits_per_pixel() {
            Some((width * bpp as u32).div_ceil(8))
        } else {
            let (block_width, _) = self.get_block_dimensions()?;
            let bs = self.get_block_size()?;
            Some(1.max(width.div_ceil(block_width)) * bs)
        }
    }

    fn get_pitch_height(&self) -> u32 {
        match self.get_block_dimensions() {
            Some((_, block_height)) => block_height,
            None => 1,
        }
    }

    fn get_bits_per_pixel(&self) -> Option<u8> {
        match *self {
            DxgiFormat::Unknown => None,

            DxgiFormat::R32G32B32A32_Typeless
            | DxgiFormat::R32G32B32A32_Float
            | DxgiFormat::R32G32B32A32_UInt
            | DxgiFormat::R32G32B32A32_SInt => Some(128),

            DxgiFormat::R32G32B32_Typeless
            | DxgiFormat::R32G32B32_Float
            | DxgiFormat::R32G32B32_UInt
            | DxgiFormat::R32G32B32_SInt => Some(96),

            DxgiFormat::R16G16B16A16_Typeless
            | DxgiFormat::R16G16B16A16_Float
            | DxgiFormat::R16G16B16A16_UNorm
            | DxgiFormat::R16G16B16A16_UInt
            | DxgiFormat::R16G16B16A16_SNorm
            | DxgiFormat::R16G16B16A16_SInt
            | DxgiFormat::R32G32_Typeless
            | DxgiFormat::R32G32_Float
            | DxgiFormat::R32G32_UInt
            | DxgiFormat::R32G32_SInt
            | DxgiFormat::R32G8X24_Typeless
            | DxgiFormat::D32_Float_S8X24_UInt
            | DxgiFormat::R32_Float_X8X24_Typeless
            | DxgiFormat::X32_Typeless_G8X24_UInt => Some(64),

            DxgiFormat::R10G10B10A2_Typeless
            | DxgiFormat::R10G10B10A2_UNorm
            | DxgiFormat::R10G10B10A2_UInt
            | DxgiFormat::R11G11B10_Float
            | DxgiFormat::R8G8B8A8_Typeless
            | DxgiFormat::R8G8B8A8_UNorm
            | DxgiFormat::R8G8B8A8_UNorm_sRGB
            | DxgiFormat::R8G8B8A8_UInt
            | DxgiFormat::R8G8B8A8_SNorm
            | DxgiFormat::R8G8B8A8_SInt
            | DxgiFormat::R16G16_Typeless
            | DxgiFormat::R16G16_Float
            | DxgiFormat::R16G16_UNorm
            | DxgiFormat::R16G16_UInt
            | DxgiFormat::R16G16_SNorm
            | DxgiFormat::R16G16_SInt
            | DxgiFormat::R32_Typeless
            | DxgiFormat::D32_Float
            | DxgiFormat::R32_Float
            | DxgiFormat::R32_UInt
            | DxgiFormat::R32_SInt
            | DxgiFormat::R24G8_Typeless
            | DxgiFormat::D24_UNorm_S8_UInt
            | DxgiFormat::R24_UNorm_X8_Typeless
            | DxgiFormat::X24_Typeless_G8_UInt => Some(32),

            DxgiFormat::R8G8_Typeless
            | DxgiFormat::R8G8_UNorm
            | DxgiFormat::R8G8_UInt
            | DxgiFormat::R8G8_SNorm
            | DxgiFormat::R8G8_SInt
            | DxgiFormat::R16_Typeless
            | DxgiFormat::R16_Float
            | DxgiFormat::D16_UNorm
            | DxgiFormat::R16_UNorm
            | DxgiFormat::R16_UInt
            | DxgiFormat::R16_SNorm
            | DxgiFormat::R16_SInt => Some(16),

            DxgiFormat::R8_Typeless
            | DxgiFormat::R8_UNorm
            | DxgiFormat::R8_UInt
            | DxgiFormat::R8_SNorm
            | DxgiFormat::R8_SInt
            | DxgiFormat::A8_UNorm => Some(8),

            DxgiFormat::R1_UNorm => Some(1),

            DxgiFormat::R9G9B9E5_SharedExp => Some(32),

            DxgiFormat::R8G8_B8G8_UNorm | DxgiFormat::G8R8_G8B8_UNorm => Some(16),

            DxgiFormat::B5G6R5_UNorm | DxgiFormat::B5G5R5A1_UNorm => Some(16),

            DxgiFormat::B8G8R8A8_UNorm
            | DxgiFormat::B8G8R8X8_UNorm
            | DxgiFormat::R10G10B10_XR_Bias_A2_UNorm
            | DxgiFormat::B8G8R8A8_Typeless
            | DxgiFormat::B8G8R8A8_UNorm_sRGB
            | DxgiFormat::B8G8R8X8_Typeless
            | DxgiFormat::B8G8R8X8_UNorm_sRGB => Some(32),

            DxgiFormat::AYUV => Some(32),
            DxgiFormat::Y410 => Some(32),
            DxgiFormat::Y416 => Some(64),
            DxgiFormat::YUY2 => Some(16),
            DxgiFormat::Y210 => Some(32),
            DxgiFormat::Y216 => Some(32),
            DxgiFormat::AI44 => Some(8),
            DxgiFormat::IA44 => Some(8),
            DxgiFormat::P8 => Some(8),
            DxgiFormat::A8P8 => Some(16),
            DxgiFormat::B4G4R4A4_UNorm => Some(16),

            _ => None,
        }
    }

    fn get_block_size(&self) -> Option<u32> {
        match *self {
            DxgiFormat::BC1_Typeless | DxgiFormat::BC1_UNorm | DxgiFormat::BC1_UNorm_sRGB => {
                Some(8)
            }

            DxgiFormat::BC2_Typeless
            | DxgiFormat::BC2_UNorm
            | DxgiFormat::BC2_UNorm_sRGB
            | DxgiFormat::BC3_Typeless
            | DxgiFormat::BC3_UNorm
            | DxgiFormat::BC3_UNorm_sRGB => Some(16),

            DxgiFormat::BC4_Typeless | DxgiFormat::BC4_UNorm | DxgiFormat::BC4_SNorm => Some(8),

            DxgiFormat::BC5_Typeless
            | DxgiFormat::BC5_UNorm
            | DxgiFormat::BC5_SNorm
            | DxgiFormat::BC6H_Typeless
            | DxgiFormat::BC6H_UF16
            | DxgiFormat::BC6H_SF16
            | DxgiFormat::BC7_Typeless
            | DxgiFormat::BC7_UNorm
            | DxgiFormat::BC7_UNorm_sRGB => Some(16),

            // All ASTC blocks are 128 bits (16 bytes) regardless of footprint
            DxgiFormat::ASTC_4x4_Typeless
            | DxgiFormat::ASTC_4x4_UNorm
            | DxgiFormat::ASTC_4x4_UNorm_sRGB
            | DxgiFormat::ASTC_5x4_Typeless
            | DxgiFormat::ASTC_5x4_UNorm
            | DxgiFormat::ASTC_5x4_UNorm_sRGB
            | DxgiFormat::ASTC_5x5_Typeless
            | DxgiFormat::ASTC_5x5_UNorm
            | DxgiFormat::ASTC_5x5_UNorm_sRGB
            | DxgiFormat::ASTC_6x5_Typeless
            | DxgiFormat::ASTC_6x5_UNorm
            | DxgiFormat::ASTC_6x5_UNorm_sRGB
            | DxgiFormat::ASTC_6x6_Typeless
            | DxgiFormat::ASTC_6x6_UNorm
            | DxgiFormat::ASTC_6x6_UNorm_sRGB
            | DxgiFormat::ASTC_8x5_Typeless
            | DxgiFormat::ASTC_8x5_UNorm
            | DxgiFormat::ASTC_8x5_UNorm_sRGB
            | DxgiFormat::ASTC_8x6_Typeless
            | DxgiFormat::ASTC_8x6_UNorm
            | DxgiFormat::ASTC_8x6_UNorm_sRGB
            | DxgiFormat::ASTC_8x8_Typeless
            | DxgiFormat::ASTC_8x8_UNorm
            | DxgiFormat::ASTC_8x8_UNorm_sRGB
            | DxgiFormat::ASTC_10x5_Typeless
            | DxgiFormat::ASTC_10x5_UNorm
            | DxgiFormat::ASTC_10x5_UNorm_sRGB
            | DxgiFormat::ASTC_10x6_Typeless
            | DxgiFormat::ASTC_10x6_UNorm
            | DxgiFormat::ASTC_10x6_UNorm_sRGB
            | DxgiFormat::ASTC_10x8_Typeless
            | DxgiFormat::ASTC_10x8_UNorm
            | DxgiFormat::ASTC_10x8_UNorm_sRGB
            | DxgiFormat::ASTC_10x10_Typeless
            | DxgiFormat::ASTC_10x10_UNorm
            | DxgiFormat::ASTC_10x10_UNorm_sRGB
            | DxgiFormat::ASTC_12x10_Typeless
            | DxgiFormat::ASTC_12x10_UNorm
            | DxgiFormat::ASTC_12x10_UNorm_sRGB
            | DxgiFormat::ASTC_12x12_Typeless
            | DxgiFormat::ASTC_12x12_UNorm
            | DxgiFormat::ASTC_12x12_UNorm_sRGB => Some(16),

            _ => None,
        }
    }

    fn get_fourcc(&self) -> Option<FourCC> {
        // note: we never use this. For Dxgi formats, we set FourCC to DX10 and
        // set the format in the header10 field. But these were the FourCCs that
        // were used prior to the header10 extension to DDS.
        match *self {
            DxgiFormat::BC1_UNorm => Some(FourCC(FourCC::BC1_UNORM)),
            DxgiFormat::BC2_UNorm => Some(FourCC(FourCC::BC2_UNORM)),
            DxgiFormat::BC3_UNorm => Some(FourCC(FourCC::BC3_UNORM)),
            DxgiFormat::BC4_UNorm => Some(FourCC(FourCC::BC4_UNORM)),
            DxgiFormat::BC4_SNorm => Some(FourCC(FourCC::BC4_SNORM)),
            DxgiFormat::BC5_UNorm => Some(FourCC(FourCC::BC5_UNORM)),
            DxgiFormat::BC5_SNorm => Some(FourCC(FourCC::BC5_SNORM)),
            DxgiFormat::R8G8_B8G8_UNorm => Some(FourCC(FourCC::R8G8_B8G8_UNORM)),
            DxgiFormat::G8R8_G8B8_UNorm => Some(FourCC(FourCC::G8R8_G8B8_UNORM)),
            DxgiFormat::R16G16B16A16_UNorm => Some(FourCC(FourCC::R16G16B16A16_UNORM)),
            DxgiFormat::R16G16B16A16_SNorm => Some(FourCC(FourCC::R16G16B16A16_SNORM)),
            DxgiFormat::R16_Float => Some(FourCC(FourCC::R16_FLOAT)),
            DxgiFormat::R16G16_Float => Some(FourCC(FourCC::R16G16_FLOAT)),
            DxgiFormat::R16G16B16A16_Float => Some(FourCC(FourCC::R16G16B16A16_FLOAT)),
            DxgiFormat::R32_Float => Some(FourCC(FourCC::R32_FLOAT)),
            DxgiFormat::R32G32_Float => Some(FourCC(FourCC::R32G32_FLOAT)),
            DxgiFormat::R32G32B32A32_Float => Some(FourCC(FourCC::R32G32B32A32_FLOAT)),
            _ => None,
        }
    }

    // sRGB, float, and compressed, and larger than u32, will all yield None.
    fn requires_extension(&self) -> bool {
        matches!(
            *self,
            // Too big, and many are also not maskable types
            DxgiFormat::R32G32B32A32_Typeless |
            DxgiFormat::R32G32B32A32_Float |
            DxgiFormat::R32G32B32A32_UInt |
            DxgiFormat::R32G32B32A32_SInt |
            DxgiFormat::R32G32B32_Typeless |
            DxgiFormat::R32G32B32_Float |
            DxgiFormat::R32G32B32_UInt |
            DxgiFormat::R32G32B32_SInt |
            DxgiFormat::R16G16B16A16_Typeless |
            DxgiFormat::R16G16B16A16_Float |
            DxgiFormat::R16G16B16A16_UNorm |
            DxgiFormat::R16G16B16A16_UInt |
            DxgiFormat::R16G16B16A16_SNorm |
            DxgiFormat::R16G16B16A16_SInt |
            DxgiFormat::R32G32_Typeless |
            DxgiFormat::R32G32_Float |
            DxgiFormat::R32G32_UInt |
            DxgiFormat::R32G32_SInt |
            DxgiFormat::R32G8X24_Typeless |
            DxgiFormat::D32_Float_S8X24_UInt |
            DxgiFormat::R32_Float_X8X24_Typeless |
            DxgiFormat::X32_Typeless_G8X24_UInt |
            // Not maskable types
            DxgiFormat::R10G10B10A2_Typeless |
            DxgiFormat::R11G11B10_Float |
            DxgiFormat::R8G8B8A8_Typeless |
            DxgiFormat::R8G8B8A8_UNorm_sRGB |
            DxgiFormat::R16G16_Typeless |
            DxgiFormat::R16G16_Float |
            DxgiFormat::R32_Typeless |
            DxgiFormat::D32_Float |
            DxgiFormat::R32_Float |
            DxgiFormat::R24G8_Typeless |
            DxgiFormat::R24_UNorm_X8_Typeless |
            DxgiFormat::R8G8_Typeless |
            DxgiFormat::R16_Typeless |
            DxgiFormat::R16_Float |
            // Not maskable types
            DxgiFormat::R8_Typeless |
            // Not maskable types
            DxgiFormat::R9G9B9E5_SharedExp |
            // Not maskable types
            DxgiFormat::R10G10B10_XR_Bias_A2_UNorm |
            DxgiFormat::B8G8R8A8_Typeless |
            DxgiFormat::B8G8R8A8_UNorm_sRGB |
            DxgiFormat::B8G8R8X8_Typeless |
            DxgiFormat::B8G8R8X8_UNorm_sRGB |
            // Channels are not actual rgb
            DxgiFormat::AYUV |
            DxgiFormat::Y410 |
            DxgiFormat::Y416 |
            DxgiFormat::NV12 |
            DxgiFormat::P010 |
            DxgiFormat::P016 |
            DxgiFormat::Format_420_Opaque |
            DxgiFormat::YUY2 |
            DxgiFormat::Y210 |
            DxgiFormat::Y216 |
            DxgiFormat::NV11 |
            DxgiFormat::AI44 |
            DxgiFormat::IA44 |
            DxgiFormat::P8 |
            DxgiFormat::A8P8 |
            DxgiFormat::P208 |
            DxgiFormat::V208 |
            DxgiFormat::V408 |
            // Non-standard ASTC formats (NVIDIA extension)
            DxgiFormat::ASTC_4x4_Typeless |
            DxgiFormat::ASTC_4x4_UNorm |
            DxgiFormat::ASTC_4x4_UNorm_sRGB |
            DxgiFormat::ASTC_5x4_Typeless |
            DxgiFormat::ASTC_5x4_UNorm |
            DxgiFormat::ASTC_5x4_UNorm_sRGB |
            DxgiFormat::ASTC_5x5_Typeless |
            DxgiFormat::ASTC_5x5_UNorm |
            DxgiFormat::ASTC_5x5_UNorm_sRGB |
            DxgiFormat::ASTC_6x5_Typeless |
            DxgiFormat::ASTC_6x5_UNorm |
            DxgiFormat::ASTC_6x5_UNorm_sRGB |
            DxgiFormat::ASTC_6x6_Typeless |
            DxgiFormat::ASTC_6x6_UNorm |
            DxgiFormat::ASTC_6x6_UNorm_sRGB |
            DxgiFormat::ASTC_8x5_Typeless |
            DxgiFormat::ASTC_8x5_UNorm |
            DxgiFormat::ASTC_8x5_UNorm_sRGB |
            DxgiFormat::ASTC_8x6_Typeless |
            DxgiFormat::ASTC_8x6_UNorm |
            DxgiFormat::ASTC_8x6_UNorm_sRGB |
            DxgiFormat::ASTC_8x8_Typeless |
            DxgiFormat::ASTC_8x8_UNorm |
            DxgiFormat::ASTC_8x8_UNorm_sRGB |
            DxgiFormat::ASTC_10x5_Typeless |
            DxgiFormat::ASTC_10x5_UNorm |
            DxgiFormat::ASTC_10x5_UNorm_sRGB |
            DxgiFormat::ASTC_10x6_Typeless |
            DxgiFormat::ASTC_10x6_UNorm |
            DxgiFormat::ASTC_10x6_UNorm_sRGB |
            DxgiFormat::ASTC_10x8_Typeless |
            DxgiFormat::ASTC_10x8_UNorm |
            DxgiFormat::ASTC_10x8_UNorm_sRGB |
            DxgiFormat::ASTC_10x10_Typeless |
            DxgiFormat::ASTC_10x10_UNorm |
            DxgiFormat::ASTC_10x10_UNorm_sRGB |
            DxgiFormat::ASTC_12x10_Typeless |
            DxgiFormat::ASTC_12x10_UNorm |
            DxgiFormat::ASTC_12x10_UNorm_sRGB |
            DxgiFormat::ASTC_12x12_Typeless |
            DxgiFormat::ASTC_12x12_UNorm |
            DxgiFormat::ASTC_12x12_UNorm_sRGB
        )
    }
}

impl DxgiFormat {
    /// Returns the block dimensions `(width, height)` for block-compressed formats.
    ///
    /// BC1–BC7 always use 4×4 blocks. ASTC formats use varying block sizes
    /// from 4×4 up to 12×12. Returns `None` for non-block-compressed formats.
    fn get_block_dimensions(&self) -> Option<(u32, u32)> {
        match *self {
            DxgiFormat::BC1_Typeless
            | DxgiFormat::BC1_UNorm
            | DxgiFormat::BC1_UNorm_sRGB
            | DxgiFormat::BC2_Typeless
            | DxgiFormat::BC2_UNorm
            | DxgiFormat::BC2_UNorm_sRGB
            | DxgiFormat::BC3_Typeless
            | DxgiFormat::BC3_UNorm
            | DxgiFormat::BC3_UNorm_sRGB
            | DxgiFormat::BC4_Typeless
            | DxgiFormat::BC4_UNorm
            | DxgiFormat::BC4_SNorm
            | DxgiFormat::BC5_Typeless
            | DxgiFormat::BC5_UNorm
            | DxgiFormat::BC5_SNorm
            | DxgiFormat::BC6H_Typeless
            | DxgiFormat::BC6H_UF16
            | DxgiFormat::BC6H_SF16
            | DxgiFormat::BC7_Typeless
            | DxgiFormat::BC7_UNorm
            | DxgiFormat::BC7_UNorm_sRGB
            | DxgiFormat::ASTC_4x4_Typeless
            | DxgiFormat::ASTC_4x4_UNorm
            | DxgiFormat::ASTC_4x4_UNorm_sRGB => Some((4, 4)),

            DxgiFormat::ASTC_5x4_Typeless
            | DxgiFormat::ASTC_5x4_UNorm
            | DxgiFormat::ASTC_5x4_UNorm_sRGB => Some((5, 4)),
            DxgiFormat::ASTC_5x5_Typeless
            | DxgiFormat::ASTC_5x5_UNorm
            | DxgiFormat::ASTC_5x5_UNorm_sRGB => Some((5, 5)),
            DxgiFormat::ASTC_6x5_Typeless
            | DxgiFormat::ASTC_6x5_UNorm
            | DxgiFormat::ASTC_6x5_UNorm_sRGB => Some((6, 5)),
            DxgiFormat::ASTC_6x6_Typeless
            | DxgiFormat::ASTC_6x6_UNorm
            | DxgiFormat::ASTC_6x6_UNorm_sRGB => Some((6, 6)),
            DxgiFormat::ASTC_8x5_Typeless
            | DxgiFormat::ASTC_8x5_UNorm
            | DxgiFormat::ASTC_8x5_UNorm_sRGB => Some((8, 5)),
            DxgiFormat::ASTC_8x6_Typeless
            | DxgiFormat::ASTC_8x6_UNorm
            | DxgiFormat::ASTC_8x6_UNorm_sRGB => Some((8, 6)),
            DxgiFormat::ASTC_8x8_Typeless
            | DxgiFormat::ASTC_8x8_UNorm
            | DxgiFormat::ASTC_8x8_UNorm_sRGB => Some((8, 8)),
            DxgiFormat::ASTC_10x5_Typeless
            | DxgiFormat::ASTC_10x5_UNorm
            | DxgiFormat::ASTC_10x5_UNorm_sRGB => Some((10, 5)),
            DxgiFormat::ASTC_10x6_Typeless
            | DxgiFormat::ASTC_10x6_UNorm
            | DxgiFormat::ASTC_10x6_UNorm_sRGB => Some((10, 6)),
            DxgiFormat::ASTC_10x8_Typeless
            | DxgiFormat::ASTC_10x8_UNorm
            | DxgiFormat::ASTC_10x8_UNorm_sRGB => Some((10, 8)),
            DxgiFormat::ASTC_10x10_Typeless
            | DxgiFormat::ASTC_10x10_UNorm
            | DxgiFormat::ASTC_10x10_UNorm_sRGB => Some((10, 10)),
            DxgiFormat::ASTC_12x10_Typeless
            | DxgiFormat::ASTC_12x10_UNorm
            | DxgiFormat::ASTC_12x10_UNorm_sRGB => Some((12, 10)),
            DxgiFormat::ASTC_12x12_Typeless
            | DxgiFormat::ASTC_12x12_UNorm
            | DxgiFormat::ASTC_12x12_UNorm_sRGB => Some((12, 12)),

            _ => None,
        }
    }

    /// Attempts to determine the `DxgiFormat` from a legacy [`PixelFormat`].
    ///
    /// Only succeeds for a small set of FourCC codes that map unambiguously to
    /// DXGI formats (`DXT1` → `BC1_UNorm_sRGB`, `DXT3` → `BC2_UNorm_sRGB`,
    /// `DXT5` → `BC3_UNorm_sRGB`, `ATI1` → `BC4_UNorm`, `ATI2` → `BC5_UNorm`).
    /// Returns `None` for all other formats including bitmask-identified ones.
    pub fn try_from_pixel_format(pixel_format: &PixelFormat) -> Option<DxgiFormat> {
        if let Some(ref fourcc) = pixel_format.fourcc {
            match fourcc.0 {
                FourCC::DXT1 => Some(DxgiFormat::BC1_UNorm_sRGB),
                FourCC::DXT3 => Some(DxgiFormat::BC2_UNorm_sRGB),
                FourCC::DXT5 => Some(DxgiFormat::BC3_UNorm_sRGB),
                FourCC::ATI1 => Some(DxgiFormat::BC4_UNorm),
                FourCC::ATI2 => Some(DxgiFormat::BC5_UNorm),
                _ => None,
            }
        } else {
            None
        }
    }
}
