use image::GenericImageView;
use std::{io, fs::File, f32::consts::PI};

#[derive(Copy, Clone)] 
struct LumenSize{
    luma: u64,
    count: u32,
}

fn main() {

    println!("Enter file path:");
    
    let mut file_name = String::new();

    io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to read line");


    let img = image::open(file_name.trim()).unwrap();

    
    let (x_len, y_len) = img.dimensions();
    const SCALE: u32 = 15;
    let x_size = (x_len / SCALE) + 1;
    let y_size = (y_len / SCALE) + 1;
    let mut pixel_set = vec![vec![LumenSize{luma:0, count:0}; x_size as usize]; y_size as usize];
    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    // println!("{:?}", img.color());
    for pixel in img.pixels(){
        // println!("{} {} {}", pixel.0, pixel.1, pixel.2.to_luma()[0]);
        let y: usize = (pixel.0/SCALE) as usize;
        let x: usize = (pixel.1/SCALE) as usize;
        // println!("x:{} y:{}", x, y);
        pixel_set[x][y].count+=1;
        pixel_set[x][y].luma+=pixel.2[0] as u64;
    }
    // let out_string: Vec<char> = Vec::with_capacity(((x_size+1)*y_len) as usize);

    println!("x:{} y:{}", x_size, y_size);
    
    for row in pixel_set{
        for pixel in row{
            if pixel.count < 1{
                print!(" ");
                continue;
            }
            let luma = pixel.luma/pixel.count as u64;
            if luma > 200 {
                print!("#");
            }else if luma > 100 {
                print!("/");
            }else {
                print!(" ");
            }
        }
        print!("\n");
    }

}
