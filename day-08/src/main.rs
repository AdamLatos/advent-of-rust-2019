use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let input = fs::read_to_string(input_file).expect("Err when reading file");

    let width = 25;
    let height = 6;

    let mut image = Vec::new();
    for c in input.chars() {
        image.push(c.to_digit(10).unwrap());
    }
    let mut i = 0;

    let mut zeros = 0;
    let mut ones = 0;
    let mut twos = 0;

    let mut min_zeros = width * height;
    let mut answer = 0;

    for px in &image {
        match px {
            0 => zeros += 1,
            1 => ones += 1,
            2 => twos += 1,
            _ => panic!("lol"),
        }
        i += 1;
        if i % (width * height) == 0 {
            if zeros < min_zeros {
                min_zeros = zeros;
                answer = ones * twos;
            }

            zeros = 0;
            ones = 0;
            twos = 0;
        }
    }
    println!("\nAnswer: {} \n", answer);

    let mut rendered = vec![2; width * height];

    let mut x = 0;
    let mut y = 0;
    for px in &image {
        let ix = x + width * y;
        if rendered[ix] == 2 && *px != 2 {
            rendered[ix] = *px;
        }
        x += 1;
        if x == width {
            y += 1;
            x = 0;
        }
        if y == height {
            x = 0;
            y = 0;
        }
    }
    for px in &rendered {
        match px {
            0 => print!(" "),
            1 => print!("\u{2588}", ),
            2 => print!("2"),
            _ => print!("{}", px),
        }
        i += 1;
        if i % width == 0 {
            println!();
        }
    }
    println!();
}
