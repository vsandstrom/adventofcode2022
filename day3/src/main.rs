#![allow(non_snake_case)]

const LOWER: u32 = 96;
const UPPER: u32 = 38;

fn parse(path: &str) -> Result<String, std::io::Error>{
    let f = std::fs::read_to_string(path);
    f
}

fn task1(input: &str) -> u32 {
    let mut resp = 0;
    for line in input.lines() {
        // varje ficka innehåller exakt lika många objekt.
        let comp = line.split_at(line.len() / 2);
        match comp {
            (x, y) => {
                for char in x.chars() {
                    match y.chars().position(|x| x == char) {
                        Some(_) => {
                            if char as u32 >= 97 {
                                resp += *&char as u32 - LOWER;
                                break

                            } else {
                                resp += *&char as u32 - UPPER;
                                break
                            }
                        },
                        None => resp += 0
                    }
                }
            },
        }
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
    println!("|| Sum of priority: {}", task1(&input));
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
    todo!()
}


#[test]
#[ignore]
fn asciitest(){
    // let a = getAcsciiValue(&'a');
    assert_eq!(a, 97);
}

#[test]
#[ignore]
fn asciitest2(){
    // let aa = getAcsciiValue(&'A');
    assert_eq!(aa, 65);
}
}
