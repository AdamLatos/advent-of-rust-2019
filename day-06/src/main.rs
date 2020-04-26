use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    
    // 1
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("1 argument expected - list of modules");
    }
    let fname = &args[1];
    let f = File::open(fname).unwrap();
    let f = BufReader::new(f);
    let mut bodies: HashMap<String, String> = HashMap::new();

    for line in f.lines() {
        let line = line.unwrap();
        let ids: Vec<&str> = line.split(")").collect();
        bodies.insert(ids[1].to_owned(), ids[0].to_owned()); // 0 orbits 1
    }
    let mut total = 0;
    for (k, _v) in &bodies {
        total += count_orbits(&bodies, &k);
    }
    println!("{}", total);

    // 2
    let mut my_path_to_com: Vec<String> = Vec::new();
    let mut santa_path_to_com: Vec<String> = Vec::new();
    path_to_com(&bodies, "YOU", &mut my_path_to_com);
    path_to_com(&bodies, "SAN", &mut santa_path_to_com);
    //println!("{:?}", my_path_to_com);
    //println!("{:?}", santa_path_to_com);
    while santa_path_to_com.pop() == my_path_to_com.pop() {}
    //println!("{:?}", my_path_to_com);
    //println!("{:?}", santa_path_to_com);
    println!("{:?}", my_path_to_com.len() + santa_path_to_com.len());
}

fn count_orbits(bodies: &HashMap<String, String>, start: &str) -> usize {
    if bodies.contains_key(bodies.get(start).expect("Error: no orbit")) == false {
        //println!("{} orbits COM", start);
        return 1;
    }
    else {
        //println!("{} orbits {}", start, bodies.get(start).expect("Fug"));
        return 1 + count_orbits(bodies, bodies.get(start).expect("Error: no orbit"));
    }
}

fn path_to_com(bodies: &HashMap<String, String>, start: &str, path: &mut Vec<String>) {
    if bodies.contains_key(bodies.get(start).expect("Error: no orbit")) == false {
        path.push(start.to_owned());
        return;
    }
    else {
        path.push(start.to_owned());
        path_to_com(bodies, bodies.get(start).expect("Error: no orbit"), path);
        return;
    }
}