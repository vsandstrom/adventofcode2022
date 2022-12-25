
// får bara röra sig till 1 nivå högre per steg:
// om position är på 'm' så får man gå till 'n' men inte till 'o'
// däremot får man gå neråt hur många steg som helst: 
// 'o' -> 'a'

// implementera A* search

fn parse(input: &str) -> Result<String, std::io::Error>{
    std::fs::read_to_string(input)
}

fn find_start(input: &str) -> (i32, i32){
    // if coordinates is 'S'
    let mut start = (0,0);
    start
}

fn find_goal(input: &str) -> (i32, i32){
    // if coordinates is 'E'
    let mut goal = (0,0);
    goal
}

fn manhattan(start: &(i32, i32), goal: &(i32, i32)) -> i32 {
    todo!()

}

fn main() {
    let input = parse("input.txt").unwrap();

    let start: (i32, i32) = find_start(&input);
    let goal: (i32, i32) = find_goal(&input);

    let manhattan = manhattan(&start, &goal);






    
}
