use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;

fn main() {
    // 1
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let input = fs::read_to_string(input_file).expect("Err when reading file");

    // 43210
    //let phases: Vec<isize> = vec![4,3,2,1,0];
    let mut amps: Vec<IntComputer> = Vec::new();
    for _ in 0..5 {
        amps.push(IntComputer::new(&input));
    }

    let mut val = 0;
    let mut max = 0;

    // for i1 in 0..5 {
    //     for i2 in 0..5 {
    //         if i1 == i2 {
    //             continue;
    //         }
    //         for i3 in 0..5 {
    //             if i1 == i3 || i2 == i3 {
    //                 continue;
    //             }
    //             for i4 in 0..5 {
    //                 if i1 == i4 || i2 == i4 || i3 == i4 {
    //                     continue;
    //                 }
    //                 for i5 in 0..5 {
    //                     if i1 == i5 || i2 == i5 || i3 == i5 || i4 == i5 {
    //                         continue;
    //                     }
    //                     val = 0;
    //                     let phases: Vec<isize> = vec![i1, i2, i3, i4, i5];
    //                     let mut check = 0;
    //                     if check == 1 {
    //                         continue;
    //                     }
    //                     for i in 0..5 {
    //                         amps[i].reset(&input);
    //                         amps[i].in_buf.clear();
    //                         amps[i].out_buf.clear();
    //                     }
    //                     for i in 0..5 {
    //                         amps[i].in_buf.push(val);
    //                         amps[i].in_buf.push(phases[i]);
    //                         amps[i].exec_program();
    //                         val = amps[i].out_buf.pop().unwrap();
    //                     }
    //                     if val > max {
    //                         max = val;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    // println!("{:?}", max);

    // 2

    for i1 in 5..10 {
        for i2 in 5..10 {
            if i1 == i2 {
                continue;
            }
            for i3 in 5..10 {
                if i1 == i3 || i2 == i3 {
                    continue;
                }
                for i4 in 5..10 {
                    if i1 == i4 || i2 == i4 || i3 == i4 {
                        continue;
                    }
                    for i5 in 5..10 {
                        if i1 == i5 || i2 == i5 || i3 == i5 || i4 == i5 {
                            continue;
                        }
                        val = 0;
                        let phases: Vec<isize> = vec![i1, i2, i3, i4, i5];
                        for i in 0..5 {
                            amps[i].reset(&input);
                            amps[i].in_buf.clear();
                            amps[i].out_buf.clear();
                        }
                        for i in 0..5 {
                            amps[i].in_buf.push(val);
                            amps[i].in_buf.push(phases[i]);
                            amps[i].exec_program();
                            val = amps[i].out_buf.pop().unwrap();
                        }

                        'outer: loop {
                            for i in 0..5 {
                                amps[i].in_buf.push(val);
                                amps[i].exec_program();
                                println!("{:?}", amps[i].state);
                                if amps[i].state == ComputerState::Halted {
                                    break 'outer;
                                }
                                val = amps[i].out_buf.pop().unwrap();
                            }
                            
                        }

                        //println!("{:?}", val);
                        if val > max {
                            max = val;
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", max);
}

#[derive(Clone)]
struct IntComputer {
    program: HashMap<usize, isize>,
    ip: usize,
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
            opcode: cmd % 10,
            args: Vec::new(),
        };
        //println!("op: {}", inst.opcode);
        let args_num = match inst.opcode {
            1 | 2 | 7 | 8 => 3isize,
            3 | 4 => 1,
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
            })
        }
        self.ip += 1 + args_num as usize;
        inst
    }

    fn exec_intcode(&mut self, inst: &mut Instruction) -> bool {
        for mut a in &mut inst.args {
            a.val = match a.mode {
                0 => *self
                    .program
                    .get(&(a.arg as usize))
                    .expect("Bad get in exec intcode"),
                1 => a.arg,
                _ => panic!("Unexpected mode"),
            }
        }
        match inst.opcode {
            1 => exec_add(&inst.args, &mut self.program),
            2 => exec_mul(&inst.args, &mut self.program),
            3 => exec_in(&inst.args, &mut self.program, &mut self.in_buf),
            4 => {
                exec_out(&inst.args, &mut self.out_buf);
                self.state = ComputerState::Paused;
                return true;
            }
            5 => exec_jmp_if_true(&inst.args, &mut self.ip),
            6 => exec_jmp_if_false(&inst.args, &mut self.ip),
            7 => exec_less_than(&inst.args, &mut self.program),
            8 => exec_equals(&inst.args, &mut self.program),
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
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).unwrap();
    // match input.trim().parse::<isize>() {
    //     Ok(i) => {data.insert(args[0].arg as usize, i).unwrap(); ()},
    //     _ => {panic!("Bad input!")}
    // }
    let input = in_buf.pop().unwrap();
    //println!("Input val: {}", input);
    data.insert(args[0].arg as usize, input).unwrap();
}

fn exec_out(args: &Vec<Arg>, out_buf: &mut Vec<isize>) {
    //println!("Output val: {}", args[0].val);
    out_buf.push(args[0].val);
}

fn exec_jmp_if_true(args: &Vec<Arg>, ip: &mut usize) {
    if args[0].val != 0 {
        *ip = args[1].val as usize;
        //*ip -= 3;
    }
}

fn exec_jmp_if_false(args: &Vec<Arg>, ip: &mut usize) {
    if args[0].val == 0 {
        *ip = args[1].val as usize;
        //*ip -= 3;
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]

// }
