use std::collections::HashSet;

const STARTPOSITION: u32 = 4;
const STARTPOSITION2: u32 = 14;


fn parse(path: &str) -> Result<String, std::io::Error>{
    return std::fs::read_to_string(path);
}

fn task(input: &Vec<char>, limit: u32) -> u32 {
    let mut index = limit;
    for win in input.windows(limit as usize) {
        let mut hs: HashSet<char> = HashSet::new();
        let mut count = 0;
        for i in 0..limit {
            match hs.insert(win[i as usize]) {
                true => {
                    count += 1;
                    if count == limit {
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

    
    println!("||==================|| DAY 6 ||==================||");
    println!("|| Start-of-packet: {}                         ||", task(&input, STARTPOSITION));
    println!("|| Start-of-message: {}                        ||", task(&input, STARTPOSITION2));
    println!("||===============================================||");
}
