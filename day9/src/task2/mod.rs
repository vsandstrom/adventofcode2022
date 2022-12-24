use std::collections::HashSet;
fn need_to_move_segment (prev: (i32, i32), next: (i32, i32)) -> (i32, i32) {

    let mut next = next;
    let prev = prev;
    match (prev.0 - next.0, prev.1 - next.1) {
        (0, 2) => { next.1 += 1; }
        (0, -2) => { next.1 -= 1; }
        (2, 0) => { next.0 += 1; }
        (-2, 0) => { next.0 -= 1; }
        (1, 2) => { 
            next.0 += 1;
            next.1 += 1;
        }
        (-1, 2) => {
            next.0 -= 1;
            next.1 += 1;
        }
        (1, -2) => {
            next.0 += 1;
            next.1 -= 1;
        }
        (-1, -2) => {
            next.0 -= 1;
            next.1 -= 1;
        }
        (2, 1) => {
            next.0 += 1;
            next.1 += 1;
        }
        (2, -1) => {
            next.0 += 1;
            next.1 -= 1;
        }
        (-2, 1) => {
            next.0 -= 1;
            next.1 += 1;
        }
        (-2, -1) => {
            next.0 -= 1;
            next.1 -= 1;
        }
        (2, 2) => {
            next.0 += 1;
            next.1 += 1;
        }
        (2, -2) => {
            next.0 += 1;
            next.1 -= 1;
        }
        (-2, 2) => {
            next.0 -= 1;
            next.1 += 1;
        }
        (-2, -2) => {
            next.0 -= 1;
            next.1 -= 1;
        }
        _ => {}
    }
    return next
}

pub fn solve(input: &str) -> i32 {

    // repet kan förflyttas i sidled om den behöver, segmenten följer inte nödvändigtvis föregående
    // position


    let mut rope: [(i32, i32); 10] = Default::default();

    let mut history: HashSet<(i32, i32)> = HashSet::new();
    history.insert(rope[9]);

    for line in input.lines() {
        let (dir, num) = line.split_once(' ').unwrap();
        let num = num.parse::<usize>().unwrap();
        match dir {
            "R" => {
                for _ in 0..num {
                    rope[0].1 += 1;
                    for i in 1..rope.len() {
                        rope[i] = need_to_move_segment(rope[i-1], rope[i]);

                    }
                    history.insert(rope[9]);
                }
            },
            "L" => {
                for _ in 0..num {
                    rope[0].1 -= 1;
                    for i in 1..rope.len() {
                        rope[i] = need_to_move_segment(rope[i-1], rope[i]);
                    }
                    history.insert(rope[9]);

                    };
            },
            "U" => {
                for _ in 0..num {
                    rope[0].0 +=1;
                    for i in 1..rope.len() {
                        rope[i] = need_to_move_segment(rope[i-1], rope[i]);
                        }
                    history.insert(rope[9]);
                    };
            },
            "D" => {

                for _ in 0..num {
                    rope[0].0 -=1;
                    for i in 1..rope.len() {
                        rope[i] =  need_to_move_segment(rope[i-1], rope[i]);
                    }
                    history.insert(rope[9]);
                    };
                }
            _ => panic!("hello"),
        }
    }
    return history.len() as i32
}
