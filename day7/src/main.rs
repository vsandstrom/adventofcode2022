// Return mängden dirs som innehåller mindre än 10000 enheter
// skapa en hashmap som kan kolla undermappar rekursivt.
mod rub;
use crate::rub::{task1, task2};
use std::{collections::HashMap, cell::RefCell};
use regex::Regex;

#[derive (Debug, Clone)]
struct Dir {
    dirs: Vec<String>,
    sum: u32
}

fn traverse_hashtable(startpos: &str,hashdir: &mut RefCell<HashMap<&str, Dir>>) -> u32 {
    // let mut dirs: Vec<String> = vec!();
    let mut hs = hashdir.clone();
    let mut sum = 0;
    if let Some(temp) = hs.get_mut().get_mut(startpos) {
        if temp.dirs.len() == 0 {
            return temp.sum;
        } else {
            for key in temp.dirs.as_slice() {
                // let key = temp.dirs.pop().unwrap();
                sum += traverse_hashtable(&key.to_string(), hashdir);
                println!("{} {}", key, sum);
            }
            // for dir in temp.dirs.pop() {
            //     let mut dirhs = hashdir.clone();
            //     let key: &str = &dir.clone();
            //     let mut sum = 0; 
            //     if let Some(obj) = dirhs.get_mut().get_mut(key){
            //         sum += traverse_hashtable(key, hashdir);
            //         obj.sum += sum;
            //     }
            //
            // }
        }
        return temp.sum + sum;
    } else {
        0
    }
}


fn populate_hashtable(input: &str) -> u32 {
    let root: regex::Regex =  Regex::new(r"(\$\scd\s/)").unwrap();
    let cd_dir: regex::Regex = Regex::new(r"(\$\scd\s[a-z]+)").unwrap();
    let subdir: regex::Regex = Regex::new(r"(dir\s[a-z])").unwrap();
    let file: regex::Regex = Regex::new(r"([0-9]+\s[a-z.])").unwrap();

    let mut sum: u32 = 0;

    let mut hashdir: HashMap<&str, Dir> = Default::default();

    let mut dir = "";
    let mut entry = Dir {dirs: vec!(), sum: 0};
    for line in input.lines() {
        if root.is_match(line) {
            dir = "/";
        }
        if subdir.is_match(line) {
            let subdir = line.split(" ").nth(1).unwrap();
            entry.dirs.push(subdir.to_string());
        }
        if file.is_match(line) {
            entry.sum += line.split(" ").nth(0).unwrap().parse::<u32>().unwrap();
        }

        if cd_dir.is_match(line) {
            hashdir.insert(dir.clone(), entry.clone());
            entry.dirs.clear();
            entry.sum = 0;
            dir = line.split(" ").nth(2).unwrap();
        }
    }
    hashdir.insert(dir.clone(), entry.clone());


    let mut hd = RefCell::new(hashdir.clone());

    println!("{:?}", hashdir);

    let mut rootsum = 0;
    if let Some(obj) = hashdir.get("/") {
        
            for dir in obj.dirs.as_slice() {
                rootsum += traverse_hashtable(dir, &mut hd);


        }
    }

    hd.get_mut().get_mut("/").unwrap().sum += rootsum;



    for e in hd.get_mut().iter() {
        match e {
            (str, obj) => {
                if obj.dirs.len() > 0 {
                    println!("{} {:?}",str,  obj);
                }
                if obj.sum <= 100000 {
                    sum += obj.sum;
                }
            }
        }
    }

    sum
}

fn parse_input(path: &str) -> Result<String, std::io::Error> {
    return std::fs::read_to_string(path);
}


fn main() {
    let input = parse_input("input.txt").unwrap();

    // wip solution hashtable
    println!("{:?}", populate_hashtable(&input));

    // solution borrowed - stack
    println!("{}", task1(&input));
    println!("{}", task2(&input));

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = parse_input("test1.txt").unwrap();
        // den räknar inte alla. den missar ett värde här: 584 summan för en mapp i mappträdet
        assert_eq!(populate_hashtable(&input), 95437);
    }


}

