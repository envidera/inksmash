//use image::codecs::avif::AvifEncoder;
use image::DynamicImage;
use image::ImageReader;
use webp::*;

use crate::args::Args;
use std::io::{Cursor, Write};
use std::path::Path;

use crate::err::*;

use md5;
use std::fs;
use std::fs::File;

const FOLDER: &str = "extracted";

#[derive(PartialEq, Debug)]
pub enum Img {
    Avif,
    Jpeg,
    Png,
    Webp,
    Unsupported,
}

impl Img {
    pub fn b64_prefix(&self) -> &'static str {
        match self {
            Self::Avif => "data:image/avif;base64,",
            Self::Jpeg => "data:image/jpeg;base64,",
            Self::Png => "data:image/png;base64,",
            Self::Webp => "data:image/webp;base64,",
            _ => "",
        }
    }
}

fn create_img_from_b46_data(data: &[u8]) -> DynamicImage {
    ImageReader::new(Cursor::new(data))
        .with_guessed_format()
        .unwrap_or_else(|err| err_exit!("image header", err))
        .decode()
        .unwrap_or_else(|err| err_exit!("image decode", err))
}

pub fn webp_from_b46_data(args: &Args, data: &[u8]) -> Vec<u8> {
    let img = create_img_from_b46_data(data);

    // Converta a imagem para RGBA8
    let rgba_img = img.to_rgba8();
    // Converta o ImageBuffer<Rgba<u8>> de volta para DynamicImage
    let dynamic_rgba_img = DynamicImage::ImageRgba8(rgba_img);

    // Create the WebP encoder for the above image
    let encoder: Encoder =
        Encoder::from_image(&dynamic_rgba_img).unwrap_or_else(|err| err_exit!("image encode", err));

    // Encode the image at a specified quality 0-100
    let quality: f32 = args.quality as f32;
    let webp: WebPMemory = encoder.encode(quality);

    let mut writer: Vec<u8> = Vec::new();
    writer.extend_from_slice(&webp);

    writer
}

pub fn save_image_from_b64_data(data: &[u8]) {
    if !Path::new(FOLDER).exists() {
        fs::create_dir_all(FOLDER).unwrap_or_else(|err| err_exit!("create extract path", err));
    }
    // Calculate  MD5
    let hash = md5::compute(data);

    let filename = format!("{}/{:x}.webp", FOLDER, hash);

    // if file not exit
    if !Path::new(&filename).exists() {
        // If it doesn't exist, create and write the file
        let mut file =
            File::create(&filename).unwrap_or_else(|err| err_exit!("extract image", err));
        file.write_all(data).unwrap();
    }
}

//=================

/*
// --------------------------------------------
ARCHIVED: AVIF works, but it is still too slow.
// --------------------------------------------
pub fn compress_avif(args: &Args, data: &Vec<u8>) -> Vec<u8> {
    // TODO: test speed https://lib.rs/crates/ravif

    let img = create_img_from_b46_data(data);

    let mut writer = Vec::new();
    /*
    Create a new encoder with a specified speed and quality that writes
    its output to w. speed accepts a value in the range 1-10, where 1 is the slowest and 10 is the fastest.
    Slower speeds generally yield better compression results. quality accepts a value in the range 1-100,
    where 1 is the worst and 100 is the best.
     */
    let encoder = AvifEncoder::new_with_speed_quality(&mut writer, args.speed, args.quality);

    img.write_with_encoder(encoder)
        .unwrap_or_else(|err| err_exit!("image encode", err));

    writer

    //=================
    //let decoded_filename = format!("decoded_image_{}.avif", get_id());
    //let mut file = File::create(&decoded_filename).unwrap();
    //file.write_all(&writer);
    //=================
}
*/
