
fn parse(input: &str) -> Result<String, std::io::Error> {
    return std::fs::read_to_string(input)
}

pub fn parse_forest(input: &str) -> Vec<Vec<i32>>{
    let input = parse(input).unwrap();
    let mut trees: Vec<Vec<i32>> = vec![];
    for line in input.lines() {
        let lat = line.chars().collect::<Vec<char>>().into_iter().map(|x| x as i32 - '0' as i32).collect::<Vec<i32>>();
        trees.push(lat);
    }
    trees
}
