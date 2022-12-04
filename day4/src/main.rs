// 501 is too high

mod util;

use regex::Regex;
use crate::util::parse;

fn task1(input: &str, regex: &Regex) -> u32 {
    let mut sum: u32 = 0;

    for line in input.lines() {
        let elves = regex
            .split(line)
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u32>>();


        if elves[0] <= elves [2] && elves[1] >= elves[3] {
            sum+=1;
        } else if elves[2] <= elves [0] && elves[3] >= elves[1] {
            sum+=1;
        }
    }
    sum
}

fn task2(input: &str, regex: &Regex) -> u32 {
    // Condition is true for all lines. 
    // Kan vara för att vissa har bara en region


    let mut sum: u32 = 0;
    for line in input.lines() {
        let elves = regex
            .split(line)
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u32>>();

        let elf1 = (elves[0]..=elves[1]).collect::<Vec<u32>>();
        let elf2 = (elves[2]..=elves[3]).collect::<Vec<u32>>();

        if elf1.contains(&elves[2]) || elf1.contains(&elves[3]) {
            sum+=1;
        } else if elf2.contains(&elves[0]) || elf2.contains(&elves[1]) {
            sum+=1;
        }
    }
    sum
}

fn main() {
    let input = parse("input.txt").unwrap();
    let r = Regex::new(r"([,-]+)").unwrap();
    println!("{}", task1(&input, &r));
    println!("{}", task2(&input, &r));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = parse("test.txt").unwrap();
        let r = Regex::new(r"([,-]+)").unwrap();
        assert_eq!(task1(&input, &r), 2);


    }

    #[test]
    fn test2() {
        let input = parse("test.txt").unwrap();
        let r = Regex::new(r"([,-]+)").unwrap();
        assert_eq!(task2(&input, &r), 4);


    }

    #[test]
    fn len_input(){
        let input = parse("input.txt").unwrap().lines().collect::<Vec<&str>>().len();

        assert_eq!(input, 1000);


    }
}
