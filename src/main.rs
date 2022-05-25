use std::error::Error;

use image::RgbImage;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.mode {
        Mode::Embed => {
            let mut message_buffer = if let Ok(metadata) =
                std::fs::metadata(args.message.as_ref().expect("message is required"))
            {
                if metadata.is_file() {
                    std::fs::read_to_string(args.message.as_ref().unwrap())?
                } else {
                    panic!("given message path is not a file!");
                }
            } else {
                args.message.unwrap().clone()
            };
            message_buffer.push('\0');
            let source_image = image::io::Reader::open("res/image.png")?
                .with_guessed_format()?
                .decode()?;
            let with_message = embed_message(source_image.into_rgb8(), message_buffer.as_bytes());
            if let Some(name) = args.name {
                with_message.save(name)?;
            } else {
                with_message.save("test.png")?;
            }
        }
        Mode::Extract => {
            let img_with_message = image::io::Reader::open(args.image)?
                .with_guessed_format()?
                .decode()?
                .into_rgb8();
            let decoded = read_message(&img_with_message);
            let message = String::from_utf8_lossy(&decoded);
            println!("Encoded message was: {}", message);
        }
    }

    Ok(())
}

const LSB_ZERO: u8 = 0xFE;
const LSB_ONE: u8 = 0x01;
const MSB_ONE: u8 = 0x80;

fn embed_message(buffer: RgbImage, message: &[u8]) -> RgbImage {
    let width = buffer.width();
    let height = buffer.height();
    let mut buffer = buffer.into_raw();
    for (image_bytes, &message_byte) in buffer.chunks_mut(8).zip(message) {
        let mut message_byte = message_byte;
        for image_byte in image_bytes {
            if (message_byte & MSB_ONE) != 0 {
                *image_byte |= LSB_ONE;
            } else {
                *image_byte &= LSB_ZERO;
            }
            message_byte <<= 1;
        }
    }

    RgbImage::from_vec(width, height, buffer).expect("Incorrect image buffer size.")
}

fn read_message(buffer: &RgbImage) -> Vec<u8> {
    let mut message: Vec<u8> = Vec::with_capacity(buffer.as_raw().len() / 7);
    for image_bytes in buffer.as_raw().chunks(8) {
        let mut message_byte: u8 = 0;
        for (index, image_byte) in image_bytes.iter().enumerate() {
            if (image_byte & LSB_ONE) != 0 {
                message_byte |= LSB_ONE;
            } else {
                message_byte &= LSB_ZERO;
            }
            if index < 7 {
                message_byte <<= 1;
            }
        }
        message.push(message_byte);
    }

    message
        .iter()
        .take_while(|&&val| val != 0)
        .copied()
        .collect()
}

use clap::{ArgEnum, Parser};

#[derive(Parser)]
struct Args {
    #[clap(arg_enum, short, long)]
    mode: Mode,

    #[clap(short, long)]
    image: String,

    #[clap(short, long)]
    message: Option<String>,

    #[clap(short, long, help = "Name of output file.")]
    name: Option<String>,
}

#[derive(ArgEnum, Clone, Copy)]
enum Mode {
    Embed,
    Extract,
}
