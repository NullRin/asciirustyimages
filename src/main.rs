use image::GenericImageView;
use std::io;

fn main() {

    println!("Enter file path:");
    
    let mut file_name = String::new();

    io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to read line");


    let img = image::open(file_name.trim()).unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());
}
