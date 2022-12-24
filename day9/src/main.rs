use std::collections::HashSet;

fn parse(input: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(input)
}

fn need_to_move(head: (i32, i32), tail: (i32, i32)) -> bool{
    if 
        head.0.abs_diff(tail.0) > 1 || 
        head.1.abs_diff(tail.1) > 1 {
        true
    } else {
        false
    }
}

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

fn task1(input: &str) -> i32{
    let mut head = (0,0);
    let mut tail = (0,0);
    let mut history: HashSet<(i32, i32)> = HashSet::new();
    history.insert(tail);

    for line in input.lines() {
        // R / L is the same number
        // U / D is the same number
        // U and R are positive
        
        // the head can only move in 4 directions U D L R,
        // if tail is at least 1 away from head,
        // do not update position the first course-change

        // if head only 1 step away from tail, do not move tail
        // if head IS more than 1 step away after move, move to heads previous position
        
        let (dir, num) = line.split_once(' ').unwrap();
        let num = num.parse::<usize>().unwrap();
        match dir {
            "R" => {
                for _ in 0..num {
                    let prevhead = head;
                    head.1 +=1;
                    if need_to_move(head, tail) {
                        tail = prevhead;
                        history.insert(tail);
                    };
                }
            },

            "L" => {
                for _ in 0..num {
                    let prevhead = head;
                    head.1 -=1;
                    if need_to_move(head, tail) {
                        tail = prevhead;
                        history.insert(tail);
                    };
                }
            },
            "U" => {
                for _ in 0..num {
                    let prevhead = head;
                    head.0 +=1;
                    if need_to_move(head, tail) {
                        tail = prevhead;
                        history.insert(tail);
                    }; 
                }
            },
            "D" => {

                for _ in 0..num {
                    let prevhead = head;
                    head.0 -=1;
                    if need_to_move(head, tail) {
                        tail = prevhead;
                        history.insert(tail);
                    }; 
                }
            }
            _ => panic!("hello"),
        }
    }
    history.len() as i32
}

fn task2(input: &str) -> i32 {

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


fn main() {
    println!("{}", task1(&parse("input.txt").unwrap()));
    println!("{}", task2(&parse("input.txt").unwrap()));

} 


#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "test.txt";
    const TEST2: &str = "test2.txt";

    #[test]
    fn test1() {
        let test = parse(TEST).unwrap();
        assert_eq!(13, task1(&test));
    }
    
    #[test]
    fn test2() {
        let test = parse(TEST2).unwrap();
        assert_eq!(36, task2(&test));
    }
}
