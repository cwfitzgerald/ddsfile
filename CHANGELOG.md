# Changelog

All notable changes to this project will be documented in this file.

The format is loosely based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to cargo's version of [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Per Keep a Changelog there are 6 main categories of changes:
- Added
- Changed
- Deprecated
- Removed
- Fixed
- Security

#### Table of Contents

- [Unreleased](#unreleased)
- [v0.5.2](#v052)
- [v0.5.1](#v051)
- [v0.5.0](#v050)
- [v0.4.0](#v040)
- [v0.2.0](#v020)

## Unreleased

### Added
- Added support for non-standard ASTC LDR `DxgiFormat` variants (4x4 through 12x12) used by NVIDIA Texture Tools. Pitch and pitch height calculations correctly account for ASTC's variable block dimensions.
- Derived `Clone` for `Dds`. @ScanMountGoat ([#17](https://github.com/SiegeEngine/ddsfile/pull/17))
- Derived `Debug` and `Clone` for `NewD3dParams` and `NewDxgiParams`.
- Documentation of all items in the crate. @cwfitzgerald in 

### Changed
- Bumped MSRV to 1.73.
- Updated `enum-primitive-derive` to 0.3. @cwfitzgerald (#5)

### Fixed
- Fixed `A8` pixel format not being detected when `rgb_bit_count` is absent, by relaxing the bit count validation when `DDPF_RGB`, `DDPF_LUMINANCE`, and `DDPF_YUV` are not flagged. @LoopyAshy ([#16](https://github.com/SiegeEngine/ddsfile/pull/16))
- Fixed `get_array_stride` computing incorrect mipmap chain sizes for non-square block-compressed textures and volume textures. The function now computes each mip level's actual size from dimensions instead of assuming each level is 1/4 the previous. (#2)
- Fixed `get_num_array_layers` returning the number of cubemaps instead of the number of faces for DX10 cubemaps, making faces beyond the first inaccessible via `get_data`. (#1)
- Fixed incorrect `get_bits_per_pixel` for several DXGI video formats: `AI44`/`IA44` (44 → 8), `Y410` (10 → 32), `Y416` (16 → 64), `Y210` (10 → 32), `Y216` (16 → 32). Planar and opaque formats (`NV12`, `P010`, `P016`, `Format_420_Opaque`, `NV11`, `P208`, `V208`, `V408`) now correctly return `None`.

## v0.5.2

Released 2023-10-29

### Added
- Added `CUBEMAP_ALLFACES` flag on `Caps2`. @Kanabenki ([#14](https://github.com/SiegeEngine/ddsfile/pull/14))

### Changed
- Handle cubemaps as 6 array layers in D3D format conversions.
- Switched from abandoned `enum_primitive` crate to `enum-primitive-derive`.
- Updated to Rust edition 2021.
- Updated `bitflags` to 2.4.
- Updated `byteorder` to 1.5.

## v0.5.1

Released 2022-03-09

### Fixed
- Fixed incorrect `linear_size` calculation for compressed textures. @Danielmelody ([#11](https://github.com/SiegeEngine/ddsfile/pull/11))

## v0.5.0

Released 2022-01-20

### Changed
- **Breaking:** `new_d3d()` and `new_dxgi()` now take named parameter structs instead of positional arguments.
- `Dds::read()` now takes `Read` by value instead of by mutable reference. @Veykril ([#1](https://github.com/SiegeEngine/ddsfile/pull/1))

### Added
- Made `header10.misc_flag` public.
- Set `DEPTH` header flag when creating a file with depth. @expenses ([#2](https://github.com/SiegeEngine/ddsfile/pull/2))
- Set `MIPMAPCOUNT` header flag when writing mipmapped D3D files. @w-flo ([#4](https://github.com/SiegeEngine/ddsfile/pull/4))
- Set `MIPMAPCOUNT` header flag when writing mipmapped DXGI files.

### Fixed
- Include depth in `linear_size` calculation. @expenses ([#3](https://github.com/SiegeEngine/ddsfile/pull/3))
- DXGI cubemap `array_size` is now 1 per cube instead of 1 per face.

## v0.4.0

Released 2019-11-17

### Changed
- Replaced `error-chain` with a local `Error` type.
- Updated to Rust edition 2018.
- Updated `bitflags` and `byteorder` dependency versions.
- DXT1/DXT3/DXT5 formats are now recognized as sRGB (the assumed default when not specified).

### Added
- Recognize BC4 (ATI1), BC5 (ATI2), and other formats as `DxgiFormat` that are not listed by Microsoft in `D3DFormat`.

### Fixed
- Several bug fixes related to linear size calculation.

## v0.2.0

Released 2018-01-21

### Added
- `Dds` struct for reading and writing DDS files via `read()` and `write()`.
- `Dds::new_d3d()` and `Dds::new_dxgi()` constructors for creating DDS files.
- `Dds::get_data()` and `Dds::get_data_mut()` for accessing raw pixel data.
- `Dds::get_d3d_format()`, `Dds::get_dxgi_format()`, and `Dds::get_main_texture_data_size()`.
- `Dds::get_array_stride()` helper function.
- `Header`, `Header10`, and `PixelFormat` structs with constructors and bitflag types.
- `DataFormat` trait providing `pitch()`, `bits_per_pixel()`, `block_size()`, `pitch_height()`, `get_minimum_mipmap_size_in_bytes()`, `get_fourcc()`, and `requires_extension()`.
- `impl From<D3DFormat>` and `impl From<DxgiFormat>` for `PixelFormat`.
- `impl Default for PixelFormat`.
- `impl Debug` for `Dds`, `Header`, `Header10`, and `PixelFormat`.
- `D3DFormat` and `DxgiFormat` enums with comprehensive format variants.

### Fixed
- Correct minimum mipmap size calculation for compressed textures.
- Proper handling of depth vs. array layers.

## Diffs

- [Unreleased](https://github.com/cwfitzgerald/ddsfile/compare/v0.5.2...HEAD)
- [v0.5.2](https://github.com/cwfitzgerald/ddsfile/compare/v0.5.1...v0.5.2)
- [v0.5.1](https://github.com/cwfitzgerald/ddsfile/compare/v0.5.0...v0.5.1)
- [v0.5.0](https://github.com/cwfitzgerald/ddsfile/compare/v0.4.0...v0.5.0)
- [v0.4.0](https://github.com/cwfitzgerald/ddsfile/compare/v0.2.0...v0.4.0)
- [v0.2.0](https://github.com/cwfitzgerald/ddsfile/compare/9614f12...v0.2.0)
