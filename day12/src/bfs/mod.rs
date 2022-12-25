use std::collections::HashMap;
type Coord = (usize, usize);
fn find_pos(input: &str, x: char) -> (usize, usize){
    // if coordinates is 'S'
    let mut start = (0,0);
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == x {
                start = (i, j);
            }
        }
    }
    start
}

fn find_all_pos(input: &str, x: char) -> Vec<Coord>{
    let mut startpos = vec!();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == x {
                startpos.push((i, j));
            }
        }
    }
    startpos
}

fn transform_map(input: &str) -> Vec<Vec<usize>> {
    let mut map: Vec<Vec<usize>> = vec!();
    for line in input.lines() {
        let mut lat_map = vec!();
        for char in line.chars() {
            if char == 'S' {
                lat_map.push(0);
            } else if char == 'E' {
                lat_map.push('z' as usize - 'a' as usize);
            } else {
                lat_map.push(char as usize - 'a' as usize);
            }
        }
        map.push(lat_map);
    }
    map
}

// skriv om till bfs
fn expand_frontier(
    pos: Coord,
    distance_traveled: usize,
    height: usize,
    que: &mut Vec<Coord>,
    hashmap: &mut HashMap<Coord, usize>,
    map: &Vec<Vec<usize>>) {

    for pos in [
        (pos.0 as isize-1, pos.1 as isize),
        (pos.0 as isize+1, pos.1 as isize),
        (pos.0 as isize, pos.1 as isize-1),
        (pos.0 as isize, pos.1 as isize+1)] {
        if pos.0 >= 0 && pos.1 >= 0 && pos.0 < map.len() as isize && pos.1 < map[0].len() as isize {
            if map[pos.0 as usize][pos.1 as usize] <= height + 1 {
                if !hashmap.contains_key(&(pos.0 as usize, pos.1 as usize)) {
                    hashmap.insert((pos.0 as usize, pos.1 as usize), distance_traveled+1);
                    que.push((pos.0 as usize, pos.1 as usize));
                }
            }
        }
    }
}

pub fn solve(input: &str) -> usize {
    let mut frontier: Vec<Coord> = vec!();
    frontier.push(find_pos(&input, 'S'));
    // starts.append(&mut find_all_pos(&input, 'a'));

    let goal: Coord = find_pos(&input, 'E');
    let map = transform_map(&input);
    let mut visited: HashMap<Coord, usize> = HashMap::new();
    for pos in &frontier {
        visited.insert(*pos, 0);
    }
    while frontier.len() != 0 {
        let (row, col) = frontier.remove(0); 
        let distance_traveled = visited[&(row, col)];
        if goal == (row, col) {
            return distance_traveled;
        }
        let map = &map.clone();
        let height = map[row][col];
        expand_frontier((row,col), distance_traveled, height, &mut frontier, &mut visited, map);
    }
    0
}

pub fn solve2(input: &str) -> usize {
    let mut frontier: Vec<Coord> = vec!();
    frontier.push(find_pos(&input, 'S'));
    frontier.append(&mut find_all_pos(&input, 'a'));

    let goal: Coord = find_pos(&input, 'E');
    let map = transform_map(&input);
    let mut visited: HashMap<Coord, usize> = HashMap::new();
    for pos in &frontier {
        visited.insert(*pos, 0);
    }
    while frontier.len() != 0 {
        let (row, col) = frontier.remove(0); 
        let distance_traveled = visited[&(row, col)];
        if goal == (row, col) {
            return distance_traveled;
        }
        let map = &map.clone();
        let height = map[row][col];
        expand_frontier((row,col), distance_traveled, height, &mut frontier, &mut visited, map);
    }
    0
}
