pub fn calc(coord: (usize, usize), forest: &Vec<Vec<i32>>) -> i32 {
    let mut dirs: [i32; 4] = [0,0,0,0];
    let c = coord;

    // outer loop variable translation
    // coord.0 == i
    // coord.1 == j

    for j in (0..c.1).rev() {
        // upp
        if forest[c.0][c.1] <= forest[c.0][j] {
            dirs[0] += 1;
            break;
        } else {
            dirs[0]+=1;
        }
    }
    
    for i in (0..c.0).rev() {
        // väst
        if forest[c.0][c.1] <= forest[i][c.1] {
            dirs[1] += 1;
            break;
        } else {
            dirs[1]+=1;
        }
    }

    for i in c.0+1..forest.len() {
        // öst 
        if forest[c.0][c.1] <= forest[i][c.1] {
            dirs[2] += 1;
            break;
        } else {
            dirs[2]+=1;
        }
    }
    
    for j in c.1+1..forest[0].len() {
        // ner 

        if forest[c.0][c.1] <= forest[c.0][j] {
            dirs[3] += 1;
            break;
        } else {
            dirs[3]+=1;
        }
    }

    // let mut sum = 1;
    // for num in dirs {
    //     sum = sum * num;
    //
    // }
    // sum
    //

    dirs.iter().fold(1, |acc, &x|{ acc*x})

}

pub fn solve(forest: &mut Vec<Vec<i32>>) -> i32 {
    // räkna hur många träd bort man ser, inklusive det träd som blockar vyn
    // nord * väst * syd * öst
    let mut highest = 0;
    for i in 1..forest.len()-1 {
        for j in 1..forest[i].len()-1 {
            let temp = calc((i,j), forest);
            if temp > highest {
                highest = temp;
            }
        } 
    }
    highest
}
