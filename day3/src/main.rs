#![allow(non_snake_case)]

const LOWER: u32 = 96;
const UPPER: u32 = 38;

fn parse(path: &str) -> Result<String, std::io::Error>{
    let f = std::fs::read_to_string(path);
    f
}

fn findBadge(first: &str, second: &str, third: &str) -> Option<char> {
    for char in first.chars() {
        if second.contains(char) {
            if third.contains(char) {
                return Some(char)
            }
        } 
    }
    None
}

fn priority(c: char) -> u32 {
    if c as u32 >= 97 {
        return *&c as u32 - LOWER;
    } else {
        return *&c as u32 - UPPER;
    }
}



fn task1(input: &str) -> u32 {
    let mut resp = 0;
    for line in input.lines() {
        let compartments = line.split_at(line.len() / 2);
        match compartments {
            (x, y) => {
                for char in x.chars() {
                    if y.contains(char) {
                        if char as u32 >= 97 {
                            resp += priority(char);
                            break
                        } else {
                            resp += priority(char);
                            break
                        }
                    }
                }
            },
        }
    }
    resp
}

fn task2(input: &str) -> u32 {
    let mut resp = 0;
    let mut i = 3;
    let input = input.lines().collect::<Vec<&str>>();

    while i <= input.len() {
        // let badge = findBadge(input[i-3], input[i-2], input[i-1]);
        let badge = findBadge(&input[i-3], &input[i-2], &input[i-1]);
        match badge {
            Some(c) => {
                resp += priority(c)
            },
            None => resp += 0
        }
        i += 3;
    }
    resp

}

fn main() {
    // Normalizing numbers
    let input = match parse("input.txt") {
        Ok(input) => input,
        Err(err) => panic!("failed to read file: {}", err)
    };

    println!("||==================|| DAY 3 ||==================||");
    println!("|| Sum of priority: {}                         ||", task1(&input));
    println!("|| Sum of priority of badge groups: {}         ||", task2(&input));
    println!("||===============================================||");
}


#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn test1() {
        let input = parse("test.txt").unwrap();
        assert_eq!(task1(&input), 157);
    }

    #[test]
    fn test2() {
        let input = parse("test2.txt").unwrap();
        assert_eq!(task2(&input), 70);
    }

    #[test]
    fn test3() {
        let input = parse("test2.txt").unwrap();
        let input = input.lines().collect::<Vec<&str>>();
        let badge = findBadge(input[3], input[4], input[5]);
        assert_eq!(badge.unwrap(), 'Z');
    }
    
    #[test]
    fn test4() {
        let mut sum = 0;
        let input = parse("test2.txt").unwrap();
        let input = input.lines().collect::<Vec<&str>>();
        let badge = findBadge(&input[0], &input[1], &input[2]);
        match badge {
            Some(c) =>  sum += priority(c),
            None => sum += 0
        }
        let badge = findBadge(input[3], input[4], input[5]);
        match badge {
            Some(c) => sum += priority(c),
            None => sum += 0
        }

        assert_eq!(sum, 70);
    }

    #[test]
    fn test5 (){
        let input = parse("test2.txt").unwrap();
        let input = input.lines().collect::<Vec<&str>>();
        let mut i = 3;
        let mut resp = 0;
        while i <= input.len() {
            // let badge = findBadge(input[i-3], input[i-2], input[i-1]);
            let badge = findBadge(&input[i-3], &input[i-2], &input[i-1]);
            match badge {
                Some(c) => {
                    resp += priority(c)
                },
                None => resp += 0
            }
            i += 3;
        }

        assert_eq!(resp, 70);
    }
}
