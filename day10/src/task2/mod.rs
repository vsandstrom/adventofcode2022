use regex::Regex;
pub fn solve(input: &str) -> () {
    let noop = Regex::new(r"(noop)").unwrap();
    let addx = Regex::new(r"(addx\s[-0-9]+)").unwrap();
    let mut x = 1;
    let mut op = 0;

    for line in input.lines() {
        if noop.is_match(line) {
            if [x-1, x, x+1].contains(&(&op % 40)) {
                print!("{}", "#");
            } else {
                print!("{}", ".");
            }
            if op%40 == 39 {
                println!();
            }
            op += 1;

        }
        if addx.is_match(line) {
            let add = line.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
            for _ in 0..2 {
                if [x-1, x, x+1].contains(&(&op % 40)) {
                    print!("{}", "#");
                } else {
                    print!("{}", ".");
                }
                if op%40 == 39 {
                    println!();
                }
                op += 1;
                
            }
            x += add;
        }
    }
}
