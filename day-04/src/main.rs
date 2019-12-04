fn main() {
    let low = 271973;
    let hi = 785961;
    let mut cnt = 0;

    // 1
    for i in low..hi {
        let digits = digits(i);
        let mut stop = 0;
        for pair in digits.windows(2) {
            if pair[0] > pair[1] {
                stop = 1;
                break;
            }
        }
        if stop == 1 {
            continue;
        }
        stop = 1;
        for pair in digits.windows(2) {
            if pair[0] == pair[1] {
                stop = 0;
                break;
            }
        }
        if stop == 1 {
            continue;
        }
        cnt += 1;
        //println!("{}", i);
        //break;
    }
    println!("There are {} possibilities", cnt);

    // 2
    cnt = 0;
    for i in low..hi {
        let digits = digits(i);
        let mut stop = 0;
        for pair in digits.windows(2) {
            if pair[0] > pair[1] {
                stop = 1;
                break;
            }
        }
        if stop == 1 {
            continue;
        }
        stop = 1;
        if digits[0] == digits[1] && digits[1] != digits[2] {
            stop = 0;
        }
        if digits[5] == digits[4] && digits[4] != digits[3] {
            stop = 0;
        }
        for quad in digits.windows(4) {
            if quad[0] != quad[1] && quad[1] == quad[2] && quad[2] != quad[3] {
                stop = 0;
                break;
            }
        }
        if stop == 1 {
            continue;
        }
        cnt += 1;
        //println!("{}", i);
        //break;
    }
    println!("Actually, there are {} possibilities", cnt);
}

fn digits(num: i32) -> Vec<i32> {
    let d1 = num % 10;
    let d2 = num % 100 / 10;
    let d3 = num % 1000 / 100;
    let d4 = num % 10000 / 1000;
    let d5 = num % 100000 / 10000;
    let d6 = num / 100000;
    vec!(d6, d5, d4, d3, d2, d1)
}
