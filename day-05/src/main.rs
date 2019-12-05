use std::env;
use std::fs;
use std::io;
use std::collections::HashMap;

fn main() {
    
    // 1
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let input = fs::read_to_string(input_file).expect("Err when reading file");
    let mut program: HashMap<usize, isize> = HashMap::new();
    let mut i = 0;
    for num in input.trim().split(",") {
        program.insert(i, num.parse().unwrap());
        i += 1;
    }
    let mut ip = 0;
    let mut halt = false;
    while !halt {
        //println!("ip: {}", ip);
        //print_map(&program);
        let mut inst = parse_next_opcode(&mut program, &mut ip);
        halt = exec_intcode(&mut program, &mut inst);
    }
}

#[derive(Debug)]
struct Arg {
    arg: isize,
    mode: isize,
    val: isize,
}

#[derive(Debug)]
struct Instruction {
    opcode: isize,
    args: Vec::<Arg>,
}

fn print_map(hmap: &HashMap<usize, isize>) {
    for i in 0..10 {
        print!("{0: <4}| ", i);
    }
    println!("");
    for i in 0..10 {
        print!("{0: <4}| ", hmap.get(&i).unwrap());
    }
}

fn parse_next_opcode(program: &mut HashMap<usize, isize>, ip: &mut usize) -> Instruction {
    
    let cmd = program.get(ip).expect("Bad cmd");
    let mut inst = Instruction {
        opcode: cmd % 10,
        args: Vec::new(),
    };
    //println!("op: {}", inst.opcode);
    let args_num = match inst.opcode {
        1 | 2 => 3isize,
        3 | 4 => 1,
        99 => 0,
        _ => panic!("Undefined instruction!"),
    };
    for i in 1..(args_num + 1) {
        inst.args.push(
            Arg {
                arg: *program.get(&(*ip + i as usize)).expect("Bad get in next opcode"),
                mode: (cmd / (10isize.pow(i as u32+1))) % (10 * i),
                val: 0,
            }
        )
    }
    *ip += 1 + args_num as usize;
    inst
}

fn exec_intcode(data: &mut HashMap<usize, isize>, inst: &mut Instruction) -> bool {

    for mut a in &mut inst.args {
        a.val = match a.mode {
            0 => *data.get(&(a.arg as usize)).expect("Bad get in exec intcode"),
            1 => a.arg,
            _ => panic!("Unexpected mode"),
        }
    }
    
    match inst.opcode {
        1 => exec_add(&inst.args, data),
        2 => exec_mul(&inst.args, data),
        3 => exec_in(&inst.args, data),
        4 => exec_out(&inst.args),
        99 => return true,
        _ => panic!("Unexpected instruction"),
    }
    false
}

fn exec_add(args: &Vec<Arg>, data: &mut HashMap<usize, isize>) {
    //println!("Index {} = {} + {} ", args[2].arg, args[0].val, args[1].val);
    data.insert(args[2].arg as usize, args[0].val + args[1].val);
}

fn exec_mul(args: &Vec<Arg>, data: &mut HashMap<usize, isize>) {
    data.insert(args[2].arg as usize, args[0].val * args[1].val);
}

fn exec_in(args: &Vec<Arg>, data: &mut HashMap<usize, isize>) {
    let mut input = String::new();
    println!("Input val:");
    io::stdin().read_line(&mut input).unwrap();
    match input.trim().parse::<isize>() {
        Ok(i) => {data.insert(args[0].arg as usize, i).unwrap(); ()},
        _ => {panic!("Bad input!")}
    }
}

fn exec_out(args: &Vec<Arg>) {
    println!("{}", args[0].val);
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]


// }