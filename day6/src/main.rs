use std::collections::HashSet;

const STARTPOSITION: u32 = 4;

fn parse(path: &str) -> Result<String, std::io::Error>{
    return std::fs::read_to_string(path);
}

fn task1(input: &Vec<char>) -> u32 {
    let mut index = STARTPOSITION;
    for win in input.windows(4) {
        let mut hs: HashSet<char> = HashSet::new();
        let mut count = 0;
        for i in 0..4 {
            match hs.insert(win[i]) {
                true => {
                    count += 1;
                    if count == 4 {
                        return index;
                    }
                },
                false => {
                    count = 0;
                    continue
                }
            } 
        }
        index += 1;
    }
    index
}

fn main() {
    let input: String = parse("input.txt").unwrap();
    let mut input: Vec<char> = input.chars().collect();
    // remove '\n' at the end
    input.pop();
    input.as_slice();
    println!("{}", task1(&input));
}
