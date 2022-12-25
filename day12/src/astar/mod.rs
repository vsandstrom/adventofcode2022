use priority_queue::PriorityQueue;
use std::collections::HashMap;

type Coord = (usize, usize);

fn calc_priority(distance_traveled: usize, coord: &(usize, usize), goal: &(usize, usize)) -> u32 {
    let max: u32 = u32::MAX;
    max - manhattan(coord, goal) as u32 
}

fn manhattan(start: &(usize, usize), goal: &(usize, usize)) -> usize {
    start.0.abs_diff(goal.0) + start.1.abs_diff(goal.1)
}

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

fn expand_frontier(
    pos: (usize, usize),
    distance_traveled: usize,
    height: usize,
    que: &mut PriorityQueue<(usize, usize), u32>,
    hashmap: &mut HashMap<Coord, usize>,
    map: &Vec<Vec<usize>>, 
    goal: &(usize, usize)) {

    for pos in [
        (pos.0 as isize-1, pos.1 as isize),
        (pos.0 as isize+1, pos.1 as isize),
        (pos.0 as isize, pos.1 as isize-1),
        (pos.0 as isize, pos.1 as isize+1)] {
        if pos.0 >= 0 && pos.1 >= 0 && pos.0 < map.len() as isize && pos.1 < map[0].len() as isize {
            if map[pos.0 as usize][pos.1 as usize] == height + 1 || map[pos.0 as usize][pos.1 as usize] <= height {
                if !hashmap.contains_key(&(pos.0 as usize, pos.1 as usize)) {
                    hashmap.insert((pos.0 as usize, pos.1 as usize), distance_traveled+1);
                    que.push((pos.0 as usize, pos.1 as usize), calc_priority(distance_traveled, &(pos.0 as usize, pos.1 as usize), goal));

                }

            }
        }
    }
}

pub fn asearch(input: &str) -> usize {
    let start: Coord = find_pos(&input, 'S');
    let goal: Coord = find_pos(&input, 'E');
    let init_cost: u32 = calc_priority(0, &start, &goal);
    // testa manhattan distance
    // println!("{:?}, {:?}, {}", start, goal, init_manhattan);
    let map = transform_map(&input);

    let mut frontier: PriorityQueue<Coord, u32> = PriorityQueue::new();
    let mut visited: HashMap<Coord, usize> = HashMap::new();
    frontier.push(start, init_cost);
    visited.insert(start, 0);

    while let Some(((row, col),_)) = frontier.pop() {
        let distance_traveled = visited[&(row, col)];
        if goal == (row, col) {
            return distance_traveled;
        }
        let map = &map.clone();
        let height = map[row][col];
        expand_frontier((row,col), distance_traveled, height, &mut frontier, &mut visited, map, &goal);
    }
    0
}
