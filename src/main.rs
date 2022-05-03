#![feature(slice_split_at_unchecked)]

use clap::{ArgEnum, Parser};
use image::{EncodableLayout, GrayImage};
use rand::random;

fn main() {
    let args = Args::parse();

    match args.mode {
        Mode::Encrypt => {
            let src = image::open(args.files[0].as_str()).unwrap().into_luma8();
            let (src, first, second) = prepare_canvas(src, args.scale);
            let (first, second) = encode(src, first, second);
            first.save("res/first.png").unwrap();
            second.save("res/second.png").unwrap();
        }
        Mode::Decrypt => {
            let first = image::open(args.files[0].as_str()).unwrap().into_luma8();
            let second = image::open(args.files[1].as_str()).unwrap().into_luma8();

            let decoded = decode(first.clone(), second.clone(), args.scale);
            decoded.save("res/decoded.png").unwrap();
        }
    }
}

fn decode(first: GrayImage, second: GrayImage, scale: Scale) -> GrayImage {
    let mut decoded: Vec<u8> = Vec::with_capacity(first.width() as usize * first.height() as usize);

    let parts: Vec<u8> = first
        .as_bytes()
        .iter()
        .zip(second.as_bytes())
        .map(|(&x, &y)| x & y)
        .collect();

    let parts = if Scale::Proportional == scale {
        let width = first.width() as usize;
        parts[..]
            .chunks(width)
            .enumerate()
            .filter(|(i, _)| i & 0x01 == 0)
            .map(|(_, row)| row)
            .flatten()
            .copied()
            .collect()
    } else {
        parts
    };

    for pair in parts[..].chunks(2) {
        match pair {
            [x, y] => {
                if x != y {
                    decoded.push(255);
                } else {
                    decoded.push(0);
                }
            }
            _ => continue,
        }
    }

    let (width, height) = match scale {
        Scale::Proportional => (first.width() / 2, first.height() / 2),
        Scale::Width => (first.width() / 2, first.height()),
    };

    GrayImage::from_vec(width, height, decoded).unwrap()
}

fn encode(
    src: GrayImage,
    mut first_canvas: Vec<u8>,
    mut second_canvas: Vec<u8>,
) -> (GrayImage, GrayImage) {
    for &pixel in src.as_bytes() {
        let first = layout();
        let second = if pixel < 128 {
            complement(&first)
        } else {
            first.clone()
        };

        first_canvas.extend_from_slice(&first);
        second_canvas.extend_from_slice(&second);
    }

    let width = src.width() * 2;
    let height = src.height();

    let first = GrayImage::from_vec(width, height, first_canvas).unwrap();
    let second = GrayImage::from_vec(width, height, second_canvas).unwrap();

    (first, second)
}

fn prepare_canvas(src: GrayImage, scale: Scale) -> (GrayImage, Vec<u8>, Vec<u8>) {
    let width = src.width() as usize;
    let height = src.height() as usize;
    let size = match scale {
        Scale::Proportional => (width * 2 * height * 2) as usize,
        Scale::Width => (width * 2 * height) as usize,
    };

    let src = match scale {
        Scale::Proportional => {
            let mut start = 0usize;

            let mut extended_src: Vec<u8> = Vec::with_capacity(size);
            unsafe {
                extended_src.set_len(size);
            }

            for row in src.as_bytes().chunks(width) {
                extended_src[start..start + width].copy_from_slice(row);
                start += width;
                extended_src[start..start + width].copy_from_slice(row);
                start += width;
            }

            GrayImage::from_vec(width as u32, height as u32 * 2, extended_src).unwrap()
        }
        Scale::Width => src,
    };

    let canvas = Vec::with_capacity(size);

    (src, canvas.clone(), canvas)
}

#[derive(PartialEq, ArgEnum, Clone)]
enum Scale {
    Proportional,
    Width,
}

#[inline(always)]
fn layout() -> [u8; 2] {
    if random::<bool>() {
        [0, 255]
    } else {
        [255, 0]
    }
}

#[inline(always)]
fn complement(layout: &[u8; 2]) -> [u8; 2] {
    let &[x, y] = layout;
    [y, x]
}

#[cfg(test)]
mod test_visual_coding {
    use super::*;

    #[test]
    fn test_layout_complement() {
        let layout = [0u8, 255];
        let complement = complement(&layout);

        assert_eq!(complement, [255, 0u8]);
    }
}

#[derive(Parser)]
struct Args {
    #[clap(arg_enum, short, long, help = "Available scaling modes.")]
    scale: Scale,

    #[clap(arg_enum, short, long)]
    mode: Mode,

    #[clap(short, long)]
    files: Vec<String>,
}

#[derive(ArgEnum, Clone, Copy)]
enum Mode {
    Encrypt,
    Decrypt,
}
