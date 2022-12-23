use std::collections::HashMap;

/// (rock) [A, X] = 1, (paper) [B, Y] = 2, (scissor) [C, Z] = 3
///
/// loss = 0, draw = 3, win = 6
///

fn parse(text: &str) -> Vec<String> {
    let data = std::fs::read_to_string(text)
        .expect("error reading file")
        .trim()
        .lines()
        .map(|x| x.to_string())
        .collect();
    data
}

fn calc1(lhs: &str, rhs: &str, rps: &HashMap<String, u32>, combo: &HashMap<(String, String), u32>) -> u32 {
    let mut val: u32 = 0;
    match combo.get(&(lhs.to_string(), rhs.to_string())) {
        Some(v) => val += v,
        None => ()
    }

    val += rps[rhs];
    val
}

fn calc2(lhs: &str, rhs: &str) -> u32 {
    let mut val = 0;
    match rhs {
        "Z" => val += 6 + match lhs{
            "A" => 2,
            "B" => 3,
            "C" => 1,
            _ => 0,
        },
        "Y" => val += 3 + match lhs{
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => 0,
            },
        "X" => val += match lhs{
            "A" => 3,
            "B" => 1,
            "C" => 2,
            _ => 0,
            },
        _ => () 

        }
    val
}

fn main() {
    let data = parse("input.txt");
    let data = data
        .iter()
        .map(|x| x.to_string()
             .split(' ')
             .map(|y| y.to_string())
                .collect::<Vec<String>>())
        .into_iter();

    let combo: HashMap<(String, String), u32> = HashMap::from([
                                                              (('A'.to_string(), 'Y'.to_string()), 6),
                                                              (('B'.to_string(), 'Z'.to_string()), 6),
                                                              (('C'.to_string(), 'X'.to_string()), 6),
                                                              (('A'.to_string(), 'X'.to_string()), 3),
                                                              (('B'.to_string(), 'Y'.to_string()), 3),
                                                              (('C'.to_string(), 'Z'.to_string()), 3)
    ]);

    let rps: HashMap<String, u32> = HashMap::from([
                                                      ('A'.to_string(), 1),
                                                      ('X'.to_string(), 1),
                                                      ('B'.to_string(), 2),
                                                      ('Y'.to_string(), 2),
                                                      ('C'.to_string(), 3),
                                                      ('Z'.to_string(), 3),
    ]);

    let mut sum: u32 = 0;
    for item in data.clone() {
        sum += calc1(&item[0], &item[1], &rps, &combo);
    }
    println!("{}", sum);

    sum = 0;
    for item in data.clone() {
        sum += calc2(&item[0], &item[1]);
    }

    println!("{}", sum);
}


#[cfg(test)]
mod tests {
    use super::{calc1, calc2, parse};
    use std::collections::HashMap;

    #[test]
    fn test1() {
    let data = parse("test.txt");
    let data = data.iter().map(|x| x.to_string().split(' ').map(|y| y.to_string()).collect::<Vec<String>>()).into_iter();
    let combo: HashMap<(String, String), u32> = HashMap::from([
                                                              (('A'.to_string(), 'Y'.to_string()), 6),
                                                              (('B'.to_string(), 'Z'.to_string()), 6),
                                                              (('C'.to_string(), 'X'.to_string()), 6),
                                                              (('A'.to_string(), 'X'.to_string()), 3),
                                                              (('B'.to_string(), 'Y'.to_string()), 3),
                                                              (('C'.to_string(), 'Z'.to_string()), 3)
    ]);

    let rps: HashMap<String, u32> = HashMap::from([
                                                      ('X'.to_string(), 1),
                                                      ('Y'.to_string(), 2),
                                                      ('Z'.to_string(), 3),
    ]);


    let mut sum = 0;
    for item in data.clone() {
        sum += calc1(&item[0], &item[1], &rps, &combo);
    }
    assert_eq!(15, sum);
    }

    #[test]
    fn test2() {
    let data = parse("test.txt");
    let data = data.iter().map(|x| x.to_string().split(' ').map(|y| y.to_string()).collect::<Vec<String>>()).into_iter();

    let mut sum = 0;
    for item in data.clone() {
        let temp = calc2(&item[0], &item[1]);
        println!("{}", temp);
        sum += temp;
    }
    assert_eq!(12, sum);

    }
}
