use regex::Regex;


fn parse(path: &str) -> Result<String, std::io::Error>{
    return std::fs::read_to_string(path);
}

fn task1<'a>(crates: &'a mut Vec<Vec<&'a str>>, input: &str, stack: bool) -> &'a mut Vec<Vec<&'a str>>{// -> &'a str {
    let re = Regex::new(r"\d+").unwrap();

    for line in input.lines() {
        let op: Vec<usize> = re.find_iter(line).filter_map(|x| x.as_str().parse().ok()).collect();
        // op[0] == antalet objekt som ska flyttas
        // op[1] == index av origin
        // op[2] == index av destination
        let mut temp: Vec<&str> = [].to_vec();

        for _ in 0..op[0] {
            temp.push(crates[op[1]-1].pop().unwrap());
        }

        if !stack {
            temp.reverse();
        }

        crates[op[2] -1].append(&mut temp);
        
   }

    crates
}

fn find_top<'a>(crates: &'a mut Vec<Vec<&'a str>>) -> (){
    for i in 0..9 {
        print!("{}", crates[i].last().unwrap());
    }
    println!();
}

fn main() {
    let crates = vec![

            // 1
        vec!["J", "H", "P", "M", "S", "F", "N", "V"],
            // 2
        vec!["S", "R", "L", "M", "J", "D", "Q"],
            // 3
        vec!["N", "Q", "D", "H", "C", "S", "W", "B"],
            // 4
        vec!["R", "S", "C", "L"],
            // 5
        vec!["M", "V", "T", "P", "F", "B"],
            // 6
        vec!["T", "R", "Q", "N", "C"],
            // 7
        vec!["G", "V", "R"],
            // 8
        vec!["C", "Z", "S", "P", "D", "L", "R"],
            // 9
        vec!["D", "S", "J", "V", "G", "P", "B", "F"]
    ];

    let mut crane1 = crates.clone();
    let mut crane2 = crates.clone();

    let input = parse("input.txt").unwrap();
    println!("||-----------------|| DAY 5 ||------------------||");
    print!("|| ");
    find_top(task1(&mut crane1, &input, true));
    print!("|| ");
    find_top(task1(&mut crane2, &input, false));
    println!("||----------------------------------------------||");

}

