use std::collections::HashSet;
fn need_to_move(head: (i32, i32), tail: (i32, i32)) -> bool{
    if 
        head.0.abs_diff(tail.0) > 1 || 
        head.1.abs_diff(tail.1) > 1 {
        true
    } else {
        false
    }
}


pub fn solve(input: &str) -> i32{
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
