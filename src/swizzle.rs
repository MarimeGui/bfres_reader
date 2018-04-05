use fres::ftex::{format::Format, tile_mode::TileMode, FTEX};
use std::cmp::{max, min};
use std::error::Error;

const M_BANKS: i64 = 4;
const M_BANKS_BIT_COUNT: i64 = 2;
const M_PIPES: i64 = 2;
const M_PIPES_BIT_COUNT: i64 = 1;
const M_PIPE_INTERLEAVE_BYTES: i64 = 256;
const M_PIPE_INTERLEAVE_BYTES_BIT_COUNT: i64 = 8;
const M_ROW_SIZE: i64 = 2048;
const M_SWAP_SIZE: i64 = 256;
const M_SPLIT_SIZE: i64 = 2048;
// const M_CHIP_FAMILY: i64 = 2;
const MICRO_TILE_PIXELS: i64 = 8 * 8;

pub fn deswizzle(ftex: &FTEX, data: &[u8]) -> Result<Vec<u8>, Box<Error>> {
    let mut out = data.to_vec();

    let dims = match ftex.header.texture_format.is_block_compressed() {
        true => (
            (i64::from(ftex.header.texture_width) + 3) / 4,
            (i64::from(ftex.header.texture_height) + 3) / 4,
        ),
        false => (
            i64::from(ftex.header.texture_width),
            i64::from(ftex.header.texture_height),
        ),
    };

    let bits_pp = get_format_bits_per_pixel(&(ftex.header.texture_format));
    let bytes_pp = bits_pp / 8;
    let pipe_swizzle = (i64::from(ftex.header.swizzle_value) >> 8) & 1;
    let bank_swizzle = (i64::from(ftex.header.swizzle_value) >> 9) & 3;

    for y in 0..dims.1 {
        for x in 0..dims.0 {
            let pos = match ftex.header.tile_mode {
                TileMode::Default | TileMode::LinearAligned => {
                    compute_surface_address_linear(x, y, bits_pp, i64::from(ftex.header.pitch))
                }
                TileMode::OneDTiledThin1 | TileMode::OneDTiledThick => {
                    compute_surface_address_micro_tiled(
                        x,
                        y,
                        bits_pp,
                        i64::from(ftex.header.pitch),
                        &ftex.header.tile_mode,
                    )
                }
                _ => compute_surface_address_macro_tiled(
                    x,
                    y,
                    bits_pp,
                    i64::from(ftex.header.pitch),
                    1,
                    &ftex.header.tile_mode,
                    pipe_swizzle,
                    bank_swizzle,
                ),
            };
            let pos2 = (y * dims.0 + x) * bytes_pp;
            if (pos < (data.len() as i64)) & (pos2 < (data.len() as i64)) {
                // result[pos2:pos2 + bytes_pp] = data[pos:pos + bytes_pp]
                out[pos2 as usize..(bytes_pp + pos2) as usize]
                    .clone_from_slice(&data[pos as usize..(bytes_pp + pos) as usize]);
            }
        }
    }

    Ok(out)
}

fn get_format_bits_per_pixel(format: &Format) -> i64 {
    match *format as u32 {
        0x1A => 32,
        0x31 | 0x431 | 0x34 | 0x234 => 64,
        0x32 | 0x432 | 0x33 | 0x433 | 0x35 | 0x235 => 128,
        _ => unimplemented!(),
    }
}

fn compute_surface_address_linear(x: i64, y: i64, bpp: i64, pitch: i64) -> i64 {
    (((y * pitch) + x) * bpp) / 8
}

fn compute_surface_address_micro_tiled(
    x: i64,
    y: i64,
    bpp: i64,
    pitch: i64,
    tile_mode: &TileMode,
) -> i64 {
    let micro_tile_thickness = i64::from(tile_mode.get_surface_thickness());
    let micro_tile_bytes = ((64 * micro_tile_thickness * bpp) + 7) / 8;
    let micro_tiles_per_row = pitch >> 3;
    let micro_tile_index = (x >> 3, y >> 3);
    let micro_tile_offset =
        micro_tile_bytes * (micro_tile_index.0 + micro_tile_index.1 * micro_tiles_per_row);
    let pixel_index = compute_pixel_index_micro_tile(x, y, 0, bpp, tile_mode);
    let pixel_offset = (bpp * pixel_index) >> 3;
    pixel_offset + micro_tile_offset
}

fn compute_surface_address_macro_tiled(
    x: i64,
    y: i64,
    bpp: i64,
    pitch: i64,
    height: i64,
    tile_mode: &TileMode,
    pipe_swizzle: i64,
    bank_swizzle: i64,
) -> i64 {
    let num_pipes = M_PIPES;
    let num_banks = M_BANKS;
    let num_group_bits = M_PIPE_INTERLEAVE_BYTES_BIT_COUNT;
    let num_pipe_bits = M_PIPES_BIT_COUNT;
    let num_bank_bits = M_BANKS_BIT_COUNT;
    let micro_tile_thickness = i64::from(tile_mode.get_surface_thickness());
    let micro_tile_bits = bpp * (micro_tile_thickness * MICRO_TILE_PIXELS);
    let micro_tile_bytes = (micro_tile_bits + 7) / 8;
    let pixel_index = compute_pixel_index_micro_tile(x, y, 0, bpp, tile_mode);
    let pixel_offset = bpp * pixel_index;
    let mut element_offset = pixel_offset;
    let bytes_per_sample = micro_tile_bytes;
    let samples_per_slice = M_SPLIT_SIZE / bytes_per_sample;
    let num_sample_splits = max(1, 1 / samples_per_slice);
    let (num_samples, sample_slice) = if micro_tile_bytes <= M_SPLIT_SIZE {
        (1, 0)
    } else {
        (
            samples_per_slice,
            element_offset / (micro_tile_bits / num_sample_splits),
        )
    };
    if !(micro_tile_bytes <= M_SPLIT_SIZE) {
        element_offset %= micro_tile_bits / num_sample_splits;
    }
    element_offset = (element_offset + 7) / 8;
    let mut pipe = compute_pipe_from_coord_no_rotation(x, y);
    let mut bank = compute_bank_from_coord_no_rotation(x, y);
    let mut bank_pipe = pipe + (num_pipes * bank);
    let swizzle2 = pipe_swizzle + (num_pipes * bank_swizzle);
    bank_pipe ^= num_pipes * sample_slice * ((num_banks >> 1) + 1) ^ swizzle2; // Pulled my hair off
    bank_pipe %= num_pipes * num_banks;
    pipe = bank_pipe % num_pipes;
    bank = bank_pipe / num_pipes;
    let slice_bytes = (height * pitch * micro_tile_thickness * bpp * num_samples + 7) / 8;
    let slice_offset = slice_bytes * (sample_slice / micro_tile_thickness);
    let (macro_tile_pitch, macro_tile_height) = match tile_mode {
        TileMode::TwoDTiledThin2 | TileMode::TwoBTiledThin2 => {
            ((8 * M_BANKS) >> 1, (8 * M_PIPES) * 2)
        }
        TileMode::TwoDTiledThin4 | TileMode::TwoBTiledThin4 => {
            ((8 * M_BANKS) >> 2, (8 * M_PIPES) * 4)
        }
        _ => (8 * M_BANKS, 8 * M_PIPES),
    };
    let macro_tiles_per_row = pitch / macro_tile_pitch;
    let macro_tile_bytes =
        (num_samples * micro_tile_thickness * bpp * macro_tile_height * macro_tile_pitch + 7) / 8;
    let macro_tile_index = (x / macro_tile_pitch, y / macro_tile_height);
    let macro_tile_offset =
        (macro_tile_index.0 + macro_tiles_per_row * macro_tile_index.1) * macro_tile_bytes;
    match tile_mode {
        TileMode::TwoBTiledThin1
        | TileMode::TwoBTiledThin2
        | TileMode::TwoBTiledThin4
        | TileMode::TwoBTiledThick
        | TileMode::ThreeBTiledThin1
        | TileMode::ThreeBTiledThick => {
            let bank_swap_order = [0, 1, 3, 2, 6, 7, 5, 4, 0, 0];
            let bank_swap_width = compute_surface_bank_swapped_width(tile_mode, bpp, pitch, 1);
            let swap_index = macro_tile_pitch * macro_tile_index.0 / bank_swap_width;
            bank ^= bank_swap_order[(swap_index & (M_BANKS - 1)) as usize];
        }
        _ => {}
    }
    let group_mask = (1 << num_group_bits) - 1;
    let num_swizzle_bits = num_bank_bits + num_pipe_bits;
    let total_offset = element_offset + ((macro_tile_offset + slice_offset) >> num_swizzle_bits);
    let offset_high = (total_offset & ((-group_mask) - 1)) << num_swizzle_bits;
    let offset_low = group_mask & total_offset;
    let pipe_bits = pipe << num_group_bits;
    let bank_bits = bank << (num_pipe_bits + num_group_bits);
    bank_bits | pipe_bits | offset_low | offset_high
}

fn compute_pixel_index_micro_tile(x: i64, y: i64, z: i64, bpp: i64, tile_mode: &TileMode) -> i64 {
    let thickness = tile_mode.get_surface_thickness();
    let mut pixel_bits = match bpp {
        0x08 => (
            x & 1,
            (x & 2) >> 1,
            (x & 4) >> 2,
            (y & 2) >> 1,
            (y & 1),
            (y & 4) >> 2,
            0,
            0,
            0,
        ),
        0x10 => (
            x & 1,
            (x & 2) >> 1,
            (x & 4) >> 2,
            y & 1,
            (y & 2) >> 1,
            (y & 4) >> 2,
            0,
            0,
            0,
        ),
        0x20 | 0x60 => (
            x & 1,
            (x & 2) >> 1,
            y & 1,
            (x & 4) >> 2,
            (y & 2) >> 1,
            (y & 4) >> 2,
            0,
            0,
            0,
        ),
        0x40 => (
            x & 1,
            y & 1,
            (x & 2) >> 1,
            (x & 4) >> 2,
            (y & 2) >> 1,
            (y & 4) >> 2,
            0,
            0,
            0,
        ),
        0x80 => (
            y & 1,
            x & 1,
            (x & 2) >> 1,
            (x & 4) >> 2,
            (y & 2) >> 1,
            (y & 4) >> 2,
            0,
            0,
            0,
        ),
        _ => (
            x & 1,
            (x & 2) >> 1,
            y & 1,
            (x & 4) >> 2,
            (y & 2) >> 1,
            (y & 4) >> 2,
            0,
            0,
            0,
        ),
    };
    if thickness > 1 {
        pixel_bits.6 = z & 1;
        pixel_bits.7 = (z & 2) >> 1;
    }
    if thickness == 8 {
        pixel_bits.8 = (z & 4) >> 2;
    }
    ((pixel_bits.8 << 8) | (pixel_bits.7 << 7) | (pixel_bits.6 << 6) | 32 * pixel_bits.5
        | 16 * pixel_bits.4 | 8 * pixel_bits.3 | 4 * pixel_bits.2 | pixel_bits.0
        | 2 * pixel_bits.1)
}

fn compute_pipe_from_coord_no_rotation(x: i64, y: i64) -> i64 {
    // I don't know what pipes are, but only two here !
    ((y >> 3) ^ (x >> 3)) & 1
}

fn compute_bank_from_coord_no_rotation(x: i64, y: i64) -> i64 {
    let num_pipes = M_PIPES;
    let num_banks = M_BANKS;
    if num_banks == 4 {
        let bank_bit_0 = ((y / (16 * num_pipes)) ^ (x >> 3)) & 1;
        bank_bit_0 | 2 * (((y / (8 * num_pipes)) ^ (x >> 4)) & 1)
    } else if num_banks == 8 {
        let bank_bit_0 = ((y / (32 * num_pipes)) ^ (x >> 3)) & 1;
        (bank_bit_0 | 2 * (((y / (32 * num_pipes)) ^ (y / (16 * num_pipes) ^ (x >> 4))) & 1)
            | 4 * (((y / (8 * num_pipes)) ^ (x >> 5)) & 1))
    } else {
        0
    }
}

fn compute_surface_bank_swapped_width(
    tile_mode: &TileMode,
    bpp: i64,
    pitch: i64,
    num_samples: i64,
) -> i64 {
    if !tile_mode.is_bank_swapped() {
        return 0;
    }
    let mut num_samples2 = num_samples;
    let num_banks = M_BANKS;
    let num_pipes = M_PIPES;
    let swap_size = M_SWAP_SIZE;
    let row_size = M_ROW_SIZE;
    let split_size = M_SPLIT_SIZE;
    let group_size = M_PIPE_INTERLEAVE_BYTES;
    let bytes_per_sample = 8 * bpp;
    let slices_per_tile = if bytes_per_sample != 0 {
        max(1, num_samples2 / (split_size / bytes_per_sample))
    } else {
        1
    };
    if tile_mode.is_thick() {
        num_samples2 = 4;
    }
    let bytes_per_tile_slice = num_samples2 * bytes_per_sample / slices_per_tile;
    let factor = i64::from(tile_mode.get_aspect_ratio());
    let swap_tiles = max(1, (swap_size >> 1) / bpp);
    let swap_width = swap_tiles * 8 * num_banks;
    let height_bytes = num_samples2 * factor * num_pipes * bpp / slices_per_tile;
    let swap_max = num_pipes * num_banks * row_size / height_bytes;
    let swap_min = group_size * 8 * num_banks / bytes_per_tile_slice;
    let mut bank_swap_width = min(swap_max, max(swap_min, swap_width));
    while bank_swap_width >= (2 * pitch) {
        bank_swap_width >>= 1;
    }
    bank_swap_width
}
