mod task1;
mod task2;
mod util;

fn main() {
    println!("{}", task1::solve(&util::parse("input.txt").unwrap()));
    println!("{}", task2::solve(&util::parse("input.txt").unwrap()));
} 

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "test.txt";
    const TEST2: &str = "test2.txt";

    #[test]
    fn test1() {
        let test = util::parse(TEST).unwrap();
        assert_eq!(13, task1::solve(&test));
    }
    
    #[test]
    fn test2() {
        let test = util::parse(TEST2).unwrap();
        assert_eq!(36, task2::solve(&test));
    }
}
