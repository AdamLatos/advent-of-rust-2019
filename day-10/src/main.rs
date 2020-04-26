use std::fs;

fn main() {
    
    let input = fs::read_to_string("test_input_1").expect("file read error");
    

    let arr: Vec<Vec<f64>> = input.lines()
        .map(|l| l.chars()


}
