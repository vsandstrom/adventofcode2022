mod task1;
mod task2;
mod util;

fn main() {
    let input = "input.txt";
    let mut forest = util::parse_forest(&input);

    let visible = task1::solve(&forest);
    println!("{}", visible.len());
    println!("{}", task2::solve(&mut forest));

}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = "test.txt";

    #[test]
    fn test1_result(){
        let forest = util::parse_forest(TEST1);
        let visible = task1::solve(&forest);
        assert_eq!(21, visible.len());
    }

    #[test]
    fn test_case1_1() {
        let forest = util::parse_forest(TEST1);
        let visible = task1::solve(&forest);
        let one: (i32, i32) = (1, 1);
        assert_eq!(&one, visible.get(&(1, 1)).unwrap());
    }

    #[test]
    fn test_case1_2() {
        let forest = util::parse_forest(TEST1);
        let visible = task1::solve(&forest);
        let two: (i32, i32) = (2, 1);
        assert_eq!(&two, visible.get(&(2, 1)).unwrap());
    }

    #[test]
    fn test_case1_3() {
        let forest = util::parse_forest(TEST1);
        let visible = task1::solve(&forest);
        let three: (i32, i32) = (1, 2);
        assert_eq!(&three, visible.get(&(1, 2)).unwrap());
    }

    #[test]
    fn test_case1_4() {
        let forest = util::parse_forest(TEST1);
        let visible = task1::solve(&forest);
        let four: (i32, i32) = (3, 2);
        assert_eq!(&four, visible.get(&(3,2)).unwrap());
    }

    #[test]
    fn test_case1_5() {
        let forest = util::parse_forest(TEST1);
        let visible = task1::solve(&forest);
        let five: (i32, i32) = (2, 3);
        assert_eq!(&five, visible.get(&(2, 3)).unwrap());
    }

    // task 2

    #[test]
    fn test_case2_1() {
        let mut forest = util::parse_forest(TEST1);
        assert_eq!(4, task2::calc((1, 2), &mut forest));
    }
    
    #[test]
    fn test_case2_2() {
        let mut forest = util::parse_forest(TEST1);
        assert_eq!(8, task2::calc((3,2), &mut forest));
    }
}
