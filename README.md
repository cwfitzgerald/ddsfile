# ddsfile

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)

[ddsfile on crates.io](https://crates.io/crates/ddsfile) — [Documentation](https://docs.rs/ddsfile)

A library for reading and writing Microsoft DirectDraw Surface (.DDS) files.
DDS is a container format for texture data, originally designed for DirectX but
widely used across graphics APIs (OpenGL, Vulkan, Metal) and asset pipelines.

This library handles the **container envelope** — parsing headers, computing
layout metadata (pitch, stride, mipmap sizes), and providing access to the raw
texture data. It does not decode or encode pixel data.

## Features

Both the legacy **D3DFormat** (Direct3D 9) and the modern **DxgiFormat**
(DirectX 10+) are supported, including files where the format is identified
only by bitmask. The library handles:

* Mipmapped textures, volume textures, texture arrays, and cubemaps
* Compressed formats (DXT1–DXT5, BC1–BC7)
* The DX10 extension header (`Header10`)
* Layout queries: dimensions, bits per pixel, pitch, stride, block size,
  array layer count, mipmap level count, and RGBA bitmasks

## Minimum Supported Rust Version (MSRV)

The MSRV of this crate is **1.73**. MSRV bumps are considered breaking changes
and will be accompanied by a major version bump.

## History

This crate was originally created and maintained by Mike Dilger at
[SiegeEngine/ddsfile](https://github.com/SiegeEngine/ddsfile) and was also
hosted at [PistonDevelopers/ddsfile](https://github.com/PistonDevelopers/ddsfile).
Thank you to Mike and all past contributors for their work on this library.

## License

Licensed under the MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed under the MIT license without
any additional terms or conditions.
