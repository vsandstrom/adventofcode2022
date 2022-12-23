use std::collections::{HashSet};

fn parse(input: &str) -> Result<String, std::io::Error> {
    return std::fs::read_to_string(input)
}

fn parse_forest(input: &str) -> Vec<Vec<i8>>{
    let mut trees: Vec<Vec<i8>> = vec![];
    for line in input.lines() {
        // let lat: Vec<u32> = line.chars().collect::<Vec<char>>().into_iter().map(|x| x.to_string().parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let lat = line.chars().collect::<Vec<char>>().into_iter().map(|x| x as i8 - '0' as i8).collect::<Vec<i8>>();
        trees.push(lat);
    }
    trees
}

fn parse_forest32(input: &str) -> Vec<Vec<i32>>{
    let mut trees: Vec<Vec<i32>> = vec![];
    for line in input.lines() {
        // let lat: Vec<u32> = line.chars().collect::<Vec<char>>().into_iter().map(|x| x.to_string().parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let lat = line.chars().collect::<Vec<char>>().into_iter().map(|x| x as i32 - '0' as i32).collect::<Vec<i32>>();
        trees.push(lat);
    }
    trees
}

fn calc(coord: (usize, usize), forest: &Vec<Vec<i32>>) -> i32 {
    let mut dirs: [i32; 4] = [0,0,0,0];
    let c = coord;

    // outer loop variable translation
    // coord.0 == i
    // coord.1 == j

    for j in (0..c.1).rev() {
        // upp
        if forest[c.0][c.1] <= forest[c.0][j] {
            dirs[0] += 1;
            break;
        } else {
            dirs[0]+=1;
        }
    }
    
    for i in (0..c.0).rev() {
        // väst
        if forest[c.0][c.1] <= forest[i][c.1] {
            dirs[1] += 1;
            break;
        } else {
            dirs[1]+=1;
        }
    }

    for i in c.0+1..forest.len() {
        // öst 
        if forest[c.0][c.1] <= forest[i][c.1] {
            dirs[2] += 1;
            break;
        } else {
            dirs[2]+=1;
        }
    }
    
    for j in c.1+1..forest[0].len() {
        // ner 

        if forest[c.0][c.1] <= forest[c.0][j] {
            dirs[3] += 1;
            break;
        } else {
            dirs[3]+=1;
        }
    }

    // let mut sum = 1;
    // for num in dirs {
    //     sum = sum * num;
    //
    // }
    // sum
    //

    dirs.iter().fold(1, |acc, &x|{ acc*x})

}

fn task1(trees: &Vec<Vec<i8>>) -> HashSet<(i8, i8)>{
    let mut visible: HashSet<(i8, i8)> = HashSet::new();
    // -1 for the last row on each run

    for i in 0..trees.len() {
        let mut ltall = -1;
        for j in 0..trees[i].len() {
            if trees[i][j] > ltall {
                visible.insert((i as i8, j as i8));
                ltall = trees[i][j];
            }
        } 
    }
    
    for i in 0..trees.len() {
        let mut ltall = -1;
        for j in (0..trees[i].len()).rev() {
            if trees[i][j] > ltall {
                visible.insert((i as i8, j as i8));
                ltall = trees[i][j];
            }
        } 
    }
    
    for j in 0..trees[0].len() {
        let mut ltall = -1;
        for i in 0..trees.len() {
            if trees[i][j] > ltall {
                visible.insert((i as i8, j as i8));
                ltall = trees[i][j];
        }
        } 
    }
    
    for j in 0..trees[0].len() {
        let mut ltall = -1;
        for i in (0..trees.len()).rev() {
            if trees[i][j] > ltall {
                visible.insert((i as i8, j as i8));
                ltall = trees[i][j];
            }
        } 
    }

    visible
}


fn task2(forest: &mut Vec<Vec<i32>>) -> i32 {
    // räkna hur många träd bort man ser, inklusive det träd som blockar vyn
    // nord * väst * syd * öst
    let mut highest = 0;
    for i in 1..forest.len()-1 {
        for j in 1..forest[i].len()-1 {
            let temp = calc((i,j), forest);
            if temp > highest {
                highest = temp;
            }
        } 
    }
    highest
}

fn main() {
    let input = parse("input.txt").unwrap();
    let forest = parse_forest(&input);

    let mut forest2: Vec<Vec<i32>> = parse_forest32(&input);
    let visible = task1(&forest);
    println!("{}", visible.len());
    println!("{}", task2(&mut forest2));

}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = "test.txt";

    #[test]
    fn test1_result(){
        let forest = parse_forest(&parse(TEST1).unwrap());
        let visible = task1(&forest);
        assert_eq!(21, visible.len());
    }

    #[test]
    fn test_case1_1() {
        let forest = parse_forest(&parse(TEST1).unwrap());
        let visible = task1(&forest);
        let one: (i8, i8) = (1, 1);
        assert_eq!(&one, visible.get(&(1, 1)).unwrap());
    }

    #[test]
    fn test_case1_2() {
        let forest = parse_forest(&parse(TEST1).unwrap());
        let visible = task1(&forest);
        let two: (i8, i8) = (2, 1);
        assert_eq!(&two, visible.get(&(2, 1)).unwrap());
    }

    #[test]
    fn test_case1_3() {
        let forest = parse_forest(&parse(TEST1).unwrap());
        let visible = task1(&forest);
        let three: (i8, i8) = (1, 2);
        assert_eq!(&three, visible.get(&(1, 2)).unwrap());
    }

    #[test]
    fn test_case1_4() {
        let forest = parse_forest(&parse(TEST1).unwrap());
        let visible = task1(&forest);
        let four: (i8, i8) = (3, 2);
        assert_eq!(&four, visible.get(&(3,2)).unwrap());
    }

    #[test]
    fn test_case1_5() {
        let forest = parse_forest(&parse(TEST1).unwrap());
        let visible = task1(&forest);
        let five: (i8, i8) = (2, 3);
        assert_eq!(&five, visible.get(&(2, 3)).unwrap());
    }

    // task 2

    #[test]
    fn test_case2_1() {
        let mut forest = parse_forest32(&parse(TEST1).unwrap());
        assert_eq!(4, calc((1, 2), &mut forest));
    }
    
    #[test]
    fn test_case2_2() {
        let mut forest = parse_forest32(&parse(TEST1).unwrap());
        assert_eq!(8, calc((3,2), &mut forest));
    }
}
