use std::collections::{HashSet, HashMap};

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

fn calc(coord: (i8, i8), forest: &Vec<Vec<i8>>) -> [u32; 4] {
    [0,0,0,0]
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


fn task2(trees: &Vec<Vec<i8>>) -> HashSet<(i8, i8)>{
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


fn task2(trees: &Vec<Vec<i8>>) -> HashSet<(i8, i8)>{
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

fn task2(forest: &Vec<Vec<i8>>) {
    // räkna hur många träd bort man ser, inklusive det träd som blockar vyn
    // nord * väst * syd * öst
    
    for i in 0..forest.len() {
        // north, south, west, east
        let mut dirs: [u32; 4] = [0,0,0,0];
        for j in 0..forest[i].len() {
            dirs = calc((i as i8, j as i8), &forest);


        }


    }

    todo!()
}

fn main() {
    let forest = parse_forest(&parse("input.txt").unwrap());
    let visible = task1(&forest);
    println!("{}", visible.len());

}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = "test.txt";
    const TEST2: &str = "test2.txt";

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
        let forest = parse_forest(&parse(TEST2).unwrap());
        assert_eq!(4, task2(&forest));
    }

    #[test]
    fn test_case2_1() {
        let forest = parse_forest(&parse(TEST2).unwrap());
        assert_eq!(4, task2(&forest));
    }

}
