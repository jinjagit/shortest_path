use apng::{load_dynamic_image, Encoder, Frame, PNGImage};

use std::fs::File;
use std::io::{BufWriter, Read};
use std::path::Path;

pub fn create_apng(n: usize) {
    let mut files: Vec<String> = vec![];

    for i in 0..10 {
        let file_path: String = format!("images/series_2/ants_{}_{}.png", n, i);
        files.push(file_path);
    }

    let mut png_images: Vec<PNGImage> = Vec::new();

    for f in files.iter() {
        let mut file = File::open(f).unwrap();
        let mut buffer = vec![];
        file.read_to_end(&mut buffer).unwrap();
        let img = image::load_from_memory(&buffer).unwrap();
        png_images.push(load_dynamic_image(img).unwrap());
    }

    let path = Path::new(r"images/apng/animation_2.png");
    let mut out = BufWriter::new(File::create(path).unwrap());

    let config = apng::create_config(&png_images, None).unwrap();
    let mut encoder = Encoder::new(&mut out, config).unwrap();

    println!("\nEncoding animation from 10 PNG files:");

    let mut i = 0;
    for image in png_images.iter() {
        i += 1;
        let mut t = 1;
        if i == 10 {
            t = 5;
        }

        let frame = Frame {
            delay_num: Some(t),
            delay_den: Some(2),
            ..Default::default()
        };
        encoder.write_frame(image, frame).unwrap();
    }

    match encoder.finish_encode() {
        Ok(_n) => println!("success"),
        Err(err) => eprintln!("{}", err),
    }
}
