se std::fs;

fn task(input: Vec<i32>) -> Vec<i32> {
    let mut top = vec![0, 0, 0];
    let mut temp = 0;
    for obj in input {
        if obj != 0 {
            temp += obj;
        } else {
            temp = 0;
        }

        for i in 0..3 {
            if temp > top[i] {
                top.insert(i, temp);
                top.pop();
                break;
            }
        }
    }
    top
}

fn main() {
    let input: Vec<i32> = fs::read_to_string("input.txt")
        .expect("read error")
        .trim()
        .lines()
        .map(|x| x.parse().unwrap_or(0))
        .collect();

    let result = task(input);
    println!("||==================|| DAY 1 ||==================||");
    println!("|| Elf with the most calories: {}             ||", result[0]);
    println!("|| Top three elves with most calories: {}    ||", result.iter().sum::<i32>());
    println!("||===============================================||");
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test1() {
//
//
//     }
// }
