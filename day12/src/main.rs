mod astar;
mod bfs;

use astar::asearch;
// får bara röra sig till 1 nivå högre per steg:
// om position är på 'm' så får man gå till 'n' men inte till 'o'
// däremot får man gå neråt hur många steg som helst: 
// 'o' -> 'a'
// implementera A* search

// BFS


fn parse(input: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(input)
}

fn main() {
    let input = parse("input.txt").unwrap();
    println!("BFS - Single starting position: {}", bfs::solve(&input));
    println!("BFS - All starting positions: {}", bfs::solve2(&input));
    println!("A* - does NOT always give the absolute best path...: {}", asearch(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = parse("test.txt").unwrap();
        assert_eq!(31, asearch(&input));
    }
    
    #[test]
    fn test2() {
        let input = parse("test.txt").unwrap();
        assert_eq!(31, bfs::solve(&input));
    }
}
