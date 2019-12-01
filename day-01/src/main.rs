use std::fs;
use std::env;

fn main() {

    // 1
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("1 argument expected - list of modules");
    }
    let fname = &args[1];
    let ship_modules = fs::read_to_string(fname).expect("File read error");
    let mut fuel_required: u64 = 0;
    for module in ship_modules.lines() {
        let weight: u64 = module.parse().unwrap();
        fuel_required += weight / 3 - 2;
    }
    println!("Fuel required for the spacecraft is {}", fuel_required);

    // 2
    let mut actual_fuel_required: i64 = 0;
    for module in ship_modules.lines() {
        let weight: i64 = module.parse().unwrap();
        let mut module_fuel: i64 = weight / 3 - 2;
        let mut additional_fuel: i64 = module_fuel / 3 - 2;
        while additional_fuel >= 0 {
            module_fuel += additional_fuel;
            additional_fuel = additional_fuel / 3 - 2;
        }
        actual_fuel_required += module_fuel;
    }
    println!("Actually, fuel required for the spacecraft is {}", actual_fuel_required);
}
