use image::GenericImageView;
use toml;
use std::fs;

#[derive(Copy, Clone)] 
struct LumenSize{
    luma: u64,
    count: u32,
}


pub fn read_config_from_toml() -> (String, u32, Vec<(char, u8)>) {
    let toml_content = fs::read_to_string("config.toml").expect("Failed to read file");
    let parsed_toml: toml::Value = toml::from_str(&toml_content).expect("Failed to parse TOML");

    let image_path = parsed_toml["IMAGE_PATH"]
    .as_str()
    .expect("Failed to parse IMAGE_PATH")
    .to_owned();
    
    let scale = parsed_toml["SCALE"]
        .as_integer()
        .unwrap() as u32;

    let toml_luma_character_map = parsed_toml["luma_character_map"]
        .as_table()
        .unwrap();

    let mut luma_character_map: Vec<(char, u8)> = toml_luma_character_map
        .iter()
        .map(|(key, value)| {
            let character = key
                .parse::<char>()
                .unwrap();
            let luma = value
                .as_integer()
                .unwrap() as u8;
            (character, luma)
        })
        .collect();
    luma_character_map.sort_unstable_by_key(|&(_, luma)| std::cmp::Reverse(luma));


    (image_path, scale, luma_character_map)
}

pub fn choose_char(luma: &u8, luma_character_map: &Vec<(char, u8)>) -> char{
    for char_u8 in luma_character_map{
        if *luma > char_u8.1{
            return char_u8.0;
        } 
    } 
    return ' ';
}


fn main() {

    let (image_path, scale, luma_character_map) = read_config_from_toml();

    let img = image::open(image_path.trim()).unwrap();
    
    let (x_len, y_len) = img.dimensions();
    let x_size = (x_len / scale) + 1;
    let y_size = (y_len / scale) + 1;
    let mut pixel_set = vec![vec![LumenSize{luma:0, count:0}; x_size as usize]; y_size as usize];
    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    // println!("{:?}", img.color());
    for pixel in img.pixels(){
        // println!("{} {} {}", pixel.0, pixel.1, pixel.2.to_luma()[0]);
        let y: usize = (pixel.0/scale) as usize;
        let x: usize = (pixel.1/scale) as usize;
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
            let luma: u8 = (pixel.luma/(pixel.count as u64)) as u8;
            let char_to_print = choose_char(&luma, &luma_character_map);
            print!("{}", char_to_print);
        }
        print!("\n");
    }

}
