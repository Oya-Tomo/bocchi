use std::time;
use std::thread::sleep;

use image::{self, GenericImageView};

fn main() {
    let right_bocchi_bytes = include_bytes!("../assets/right-bocchi.png");
    let left_bocchi_bytes = include_bytes!("../assets/left-bocchi.png");

    let right_bocchi = image::load_from_memory(right_bocchi_bytes).unwrap();
    let left_bocchi = image::load_from_memory(left_bocchi_bytes).unwrap();

    let r_b_code = get_img_code(right_bocchi);
    let l_b_code = get_img_code(left_bocchi);

    let dur = time::Duration::from_millis(200);

    loop {
        print!("\x1b[2J");
        print!("\x1b[1;1H");
        print!("{}\x1b[m", r_b_code);
        sleep(dur);

        print!("\x1b[2J");
        print!("\x1b[1;1H");
        print!("{}\x1b[m", l_b_code);
        sleep(dur);
    }
}

fn get_img_code(image: image::DynamicImage) -> String {
    let w = image.width();
    let h = image.height();
    let mut code = "".to_string();
    for y in 0..h {
        for x in 0..w {
            let pixel = image.get_pixel(x, y);
            code += format!("\x1b[48;2;{};{};{}m ", pixel[0], pixel[1], pixel[2]).as_str();
            // println!("\x1b[48;2;10;10;200m{} {} {}", pixel[0], pixel[1], pixel[2]);
        }
        code += "\n";
    }
    return code;
}
