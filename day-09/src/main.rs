use std::collections::HashMap;
use std::collections::hash_map::Entry;
//use std::env;
use std::fs;
//use std::io;

fn main() {
    // 1
    //let args: Vec<String> = env::args().collect();
    //let input_file = &args[1];
    let input = fs::read_to_string("input").expect("Err when reading file");
    //let input = "1102,34915192,34915192,7,4,7,99,0".to_string();
    //let input = "104,1125899906842624,99".to_string();
    //let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99".to_string();
    //let input = "109,1,203,11,209,8,204,1,99,10,0,42,0".to_string();

    let mut computer = IntComputer::new(&input);
    computer.in_buf.push(2);
    computer.exec_program();
    for c in computer.out_buf {
        print!("{},", c);
    }
    println!();
}

#[derive(Clone)]
struct IntComputer {
    program: HashMap<usize, isize>,
    ip: usize,
    relative_base: isize,
    in_buf: Vec<isize>,
    out_buf: Vec<isize>,
    state: ComputerState,
}

#[derive(PartialEq, Clone, Debug)]
enum ComputerState {
    Running,
    Paused,
    Halted,
}

impl IntComputer {
    fn new(input: &String) -> IntComputer {
        let mut computer = IntComputer {
            program: HashMap::new(),
            ip: 0,
            relative_base: 0,
            in_buf: Vec::new(),
            out_buf: Vec::new(),
            state: ComputerState::Running,
        };
        let mut i = 0;
        for num in input.trim().split(",") {
            computer.program.insert(i, num.parse().unwrap());
            i += 1;
        }
        computer
    }

    fn reset(&mut self, input: &String) {
        self.ip = 0;
        self.state = ComputerState::Running;
        let mut i = 0;
        for num in input.trim().split(",") {
            self.program.insert(i, num.parse().unwrap());
            i += 1;
        }
    }

    fn exec_program(&mut self) {
        self.state = ComputerState::Running;
        while self.state == ComputerState::Running {
            //println!("ip: {}", self.ip);
            //print_map(&self.program);
            let mut inst = self.parse_next_opcode();
            // let ip_pre = self.ip;
            self.exec_intcode(&mut inst);
        }
    }

    fn parse_next_opcode(&mut self) -> Instruction {
        let cmd = self.program.get(&self.ip).expect("Bad cmd");
        let mut inst = Instruction {
            opcode: cmd % 100,
            args: Vec::new(),
        };
        let args_num = match inst.opcode {
            1 | 2 | 7 | 8 => 3isize,
            3 | 4 | 9 => 1,
            5 | 6 => 2,
            _ => 0,
            //_ => panic!("Undefined instruction!"),
        };
        
        for i in 1..(args_num + 1) {
            inst.args.push(Arg {
                arg: *self
                    .program
                    .get(&(self.ip + i as usize))
                    .expect("Bad get in next opcode"),
                mode: (cmd / (10isize.pow(i as u32 + 1))) % (10 * i),
                val: 0,
            });
        }
        // println!("{:#?}", inst);
        self.ip += 1 + args_num as usize;
        inst
    }

    fn exec_intcode(&mut self, inst: &mut Instruction) -> bool {
        for mut a in &mut inst.args {
            a.val = match a.mode {
                0 => *self
                    .program
                    .entry(a.arg as usize)
                    .or_insert(0),
                1 => a.arg,
                2 => *self
                    .program
                    .entry((a.arg + self.relative_base) as usize)
                    .or_insert(0),
                _ => panic!("Unexpected mode"),
            };
            // Modify the address for functions that input
            match a.mode {
                2 => a.arg += self.relative_base,
                _ => {},
            };
        }

        // println!("{:#?}", inst);
        // println!("ip : {:?}", self.ip);
        // println!("bs : {:?}", self.relative_base);

        match inst.opcode {
            1 => exec_add(&inst.args, &mut self.program),
            2 => exec_mul(&inst.args, &mut self.program),
            3 => exec_in(&inst.args, &mut self.program, &mut self.in_buf),
            4 => {
                exec_out(&inst.args, &mut self.out_buf);
                //self.state = ComputerState::Paused;
                //return true;
            }
            5 => exec_jmp_if_true(&inst.args, &mut self.ip),
            6 => exec_jmp_if_false(&inst.args, &mut self.ip),
            7 => exec_less_than(&inst.args, &mut self.program),
            8 => exec_equals(&inst.args, &mut self.program),
            9 => exec_relative_base_offset(&inst.args, &mut self.relative_base),
            _ => {
                self.state = ComputerState::Halted;
                return true;
            },
            //_ => panic!("Unexpected instruction"),
        }
        false
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
    args: Vec<Arg>,
}

fn print_map(hmap: &HashMap<usize, isize>) {
    for i in 0..10 {
        print!("{0: <4}| ", i);
    }
    println!("");
    for i in 0..10 {
        print!("{0: <4}| ", hmap.get(&i).unwrap());
    }
    println!("");
}

fn exec_add(args: &Vec<Arg>, data: &mut HashMap<usize, isize>) {
    //println!("Index {} = {} + {} ", args[2].arg, args[0].val, args[1].val);
    data.insert(args[2].arg as usize, args[0].val + args[1].val);
}

fn exec_mul(args: &Vec<Arg>, data: &mut HashMap<usize, isize>) {
    data.insert(args[2].arg as usize, args[0].val * args[1].val);
}

fn exec_in(args: &Vec<Arg>, data: &mut HashMap<usize, isize>, in_buf: &mut Vec<isize>) {
    let input = in_buf.pop().unwrap();
    // println!("Input val: {}", input);
    data.insert(args[0].arg as usize, input).unwrap();
}

fn exec_out(args: &Vec<Arg>, out_buf: &mut Vec<isize>) {
    //println!("Output val: {}", args[0].val);
    out_buf.push(args[0].val);
}

fn exec_jmp_if_true(args: &Vec<Arg>, ip: &mut usize) {
    if args[0].val != 0 {
        *ip = args[1].val as usize;
    }
}

fn exec_jmp_if_false(args: &Vec<Arg>, ip: &mut usize) {
    if args[0].val == 0 {
        *ip = args[1].val as usize;
    }
}

fn exec_less_than(args: &Vec<Arg>, data: &mut HashMap<usize, isize>) {
    if args[0].val < args[1].val {
        data.insert(args[2].arg as usize, 1);
    } else {
        data.insert(args[2].arg as usize, 0);
    }
}

fn exec_equals(args: &Vec<Arg>, data: &mut HashMap<usize, isize>) {
    if args[0].val == args[1].val {
        data.insert(args[2].arg as usize, 1);
    } else {
        data.insert(args[2].arg as usize, 0);
    }
}

fn exec_relative_base_offset(args: &Vec<Arg>, offset: &mut isize) {
    *offset += args[0].val;
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]

// }