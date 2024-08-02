use base64::prelude::*;

use super::*;
use crate::args::Args;
//use crate::err::*;
use crate::id::*;
use crate::img::*;

const IMAGE_TAG_PREFIX: &str = "xlink:href=";

pub fn process_line(args: &Args, index: usize, line: &str) -> (usize, String) {
    if let Some((b64_data, img_type)) = read_img_b64_data(line) {
        // decode b64 image data
        let data = b64_decode(&b64_data)
            .unwrap_or_else(|err| err_line!(format!("b64 decode, {}", err), index));

        match img_type {
            // already compressed
            Img::Avif | Img::Webp => {
                // avif and webp do not recompress,
                //let new_line = format_new_line(img_type, &b64_data);
                //println!(" graphic {:<4} | skipped, already compressed", get_id(),);
                //return (index, new_line);

                if args.extract {
                    save_image_from_b64_data(&data);
                }

                println!(" graphic {:<4} | skipped, already compressed", get_id(),);
                return (index, line.to_string());
            }

            Img::Unsupported => {
                err_line!("image type not supported", index);
            }
            _ => (),
        }

        let writer = webp_from_b46_data(args, &data);

        if args.extract {
            save_image_from_b64_data(&writer);
        }

        let b64_new_data = BASE64_STANDARD.encode(&writer);

        // print before and after sizes
        println!(
            " graphic {:<4} | {} > {}",
            get_id(),
            super::str_mb_size(&b64_data),
            super::str_mb_size(&b64_new_data)
        );

        let new_line = format!(
            "{}\"{}{};\"",
            IMAGE_TAG_PREFIX,
            Img::Avif.b64_prefix(),
            b64_new_data
        );

        (index, new_line.to_string())
    } else {
        (index, line.to_string())
    }
}

/*
fn format_new_line(img: Img, b64_new_data: &str) -> String {
    format!(
        "{}\"{}{}\"",
        IMAGE_TAG_PREFIX,
        img.b64_prefix(),
        b64_new_data
    )
}
 */
fn read_img_b64_data(line: &str) -> Option<(String, Img)> {
    if let Some(start) = line.find(IMAGE_TAG_PREFIX) {
        let raw_content = clean_b64(&line[start + IMAGE_TAG_PREFIX.len()..]);

        if raw_content.starts_with(Img::Png.b64_prefix()) {
            let data = raw_content[Img::Png.b64_prefix().len()..].to_string();
            return Some((data, Img::Png));
        }

        if raw_content.starts_with(Img::Jpeg.b64_prefix()) {
            let data = raw_content[Img::Jpeg.b64_prefix().len()..].to_string();
            return Some((data, Img::Jpeg));
        }

        if raw_content.starts_with(Img::Webp.b64_prefix()) {
            let data = raw_content[Img::Webp.b64_prefix().len()..].to_string();
            return Some((data, Img::Webp));
        }

        if raw_content.starts_with(Img::Avif.b64_prefix()) {
            let data = raw_content[Img::Avif.b64_prefix().len()..].to_string();
            return Some((data, Img::Avif));
        }

        // not match jpeg nether png, but is a image data, so its unsupported yet
        if raw_content.starts_with("data:image") {
            return Some((String::new(), Img::Unsupported));
        }
    }
    None
}

/*
base 64 standards allowed chars

0   A   16  Q   32  g   48  w
1   B   17  R   33  h   49  x
2   C   18  S   34  i   50  y
3   D   19  T   35  j   51  z
4   E   20  U   36  k   52  0
5   F   21  V   37  l   53  1
6   G   22  W   38  m   54  2
7   H   23  X   39  n   55  3
8   I   24  Y   40  o   56  4
9   J   25  Z   41  p   57  5
10  K   26  a   42  q   58  6
11  L   27  b   43  r   59  7
12  M   28  c   44  s   60  8
13  N   29  d   45  t   61  9
14  O   30  e   46  u   62  +
15  P   31  f   47  v   63  /
Padding 	=

https://en.wikipedia.org/wiki/Base64
*/

fn clean_b64(content: &str) -> String {
    // +1 and -1 to remove " char from beginning and and
    let mut clean = content[1..content.len() - 1]
        // Inkscape could add a ugly &#10; between lines, so also remove it
        .replace("&#10;", "")
        // Inkscape could also add  " " spaces, between lines, so also remove it
        .replace(" ", "")
        // or new line
        .replace("\n", "");

    // for re encode
    if clean.ends_with(";") {
        clean.pop();
    }

    clean
}

fn b64_decode(b64_data: &str) -> Result<Vec<u8>, base64::DecodeError> {
    BASE64_STANDARD.decode(b64_data)
}

//=====================================================================
// UNIT TESTS
//=====================================================================

#[cfg(test)]
mod tests {

    use super::*;

    // Inkscape b64 add &#10 strings between lines, so need also remove it
    #[test]
    fn test_read_img_b64() {
        {
            const LINE: &str = r#"xlink:href="data:image/avif;base64,base64-&#10;content""#;
            let (content, img_type) = read_img_b64_data(LINE).unwrap();
            assert_eq!(content, "base64-content");
            assert_eq!(img_type, Img::Avif);
        }
        {
            const LINE: &str = r#"xlink:href="data:image/jpeg;base64,base64-&#10;content""#;
            let (content, img_type) = read_img_b64_data(LINE).unwrap();
            assert_eq!(content, "base64-content");
            assert_eq!(img_type, Img::Jpeg);
        }
        {
            const LINE: &str = r#"xlink:href="data:image/webp;base64,base64-&#10;content""#;
            let (content, img_type) = read_img_b64_data(LINE).unwrap();
            assert_eq!(content, "base64-content");
            assert_eq!(img_type, Img::Webp);
        }
        // with indent
        {
            const LINE: &str = r#"  xlink:href="data:image/png;base64,base64-&#10;content""#;
            let (content, img_type) = read_img_b64_data(LINE).unwrap();
            assert_eq!(content, "base64-content");
            assert_eq!(img_type, Img::Png);
        }

        // unsupported
        {
            const LINE: &str = r#"xlink:href="data:image/xyz;base64,base64-&#10;content""#;
            let (content, img_type) = read_img_b64_data(LINE).unwrap();
            assert_eq!(content, "");
            assert_eq!(img_type, Img::Unsupported);
        }
    }
}
