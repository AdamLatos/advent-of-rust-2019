use std::env;
use std::fs;

fn main() {
    
    // 1
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let input = fs::read_to_string(input_file).expect("Err when reading file");
    let mut program: Vec<usize> = Vec::with_capacity(100);
    for num in input.trim().split(",") {
        program.push(num.parse().unwrap());
    }
    let program_copy = program.clone();
    run_intcode(&mut program);
    println!{"Answer is {:?}", program[0]}

    // 2
    for i1 in 0..100 {
        for i2 in 0..100 {
            program = program_copy.clone();
            program[1] = i1;
            program[2] = i2;
            run_intcode(&mut program);
            if program[0] == 19690720 {
                println!("Second answer is {:?}", i1 * 100 + i2);
            }
        }
    }



}

fn run_intcode(program: &mut Vec<usize>) {
    
    let mut ip = 0;
    let mut opcode = program[ip];

    while opcode != 99 {

        let ptr_arg_1 = program[ip+1];
        let ptr_arg_2 = program[ip+2];
        let ptr_res = program[ip+3];
        let arg_1 = program[ptr_arg_1];
        let arg_2 = program[ptr_arg_2];

        match opcode {
            1 => {
                program[ptr_res] = arg_1 + arg_2;
            }
            2 => {
                program[ptr_res] = arg_1 * arg_2;
            }
            _ => {
                panic!("Unexpected operation {} at instruction {}", opcode, ip);
            }
        }
        ip += 4;
        opcode = program[ip];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intcode() {
        let mut program: Vec<usize> = vec![1,9,10,3,2,3,11,0,99,30,40,50];
        let expected: Vec<usize> = vec![3500,9,10,70,2,3,11,0,99,30,40,50];
        run_intcode(&mut program);
        assert_eq!(program, expected);
    }

}