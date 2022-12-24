use regex::Regex;
pub fn solve(input: &str, goal: Vec<i32>) -> Vec<i32>{
    let noop = Regex::new(r"(noop)").unwrap();
    let addx = Regex::new(r"(addx\s[-0-9]+)").unwrap();

    let mut result: Vec<i32> = vec![];

    let mut x = 1;
    let mut op = 0;

    for line in input.lines() {
        if noop.is_match(line) {
            op += 1;
            if goal.contains(&op) {
                result.push(x * op);
            }

        }
        if addx.is_match(line) {
            let add = line.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
            for _ in 0..2 {
                op += 1;
                if goal.contains(&op) {
                    result.push(x * op);
                }
            }
            x += add;
        }
    }
    result

}
