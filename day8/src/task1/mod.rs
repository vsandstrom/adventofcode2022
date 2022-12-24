use std::collections::{HashSet};

pub fn solve(trees: &Vec<Vec<i32>>) -> HashSet<(i32, i32)> {
    let mut visible: HashSet<(i32, i32)> = HashSet::new();
    // -1 for the last row on each run

    for i in 0..trees.len() {
        let mut ltall = -1;
        for j in 0..trees[i].len() {
            if trees[i][j] > ltall {
                visible.insert((i as i32, j as i32));
                ltall = trees[i][j];
            }
        } 
    }
    
    for i in 0..trees.len() {
        let mut ltall = -1;
        for j in (0..trees[i].len()).rev() {
            if trees[i][j] > ltall {
                visible.insert((i as i32, j as i32));
                ltall = trees[i][j];
            }
        } 
    }
    
    for j in 0..trees[0].len() {
        let mut ltall = -1;
        for i in 0..trees.len() {
            if trees[i][j] > ltall {
                visible.insert((i as i32, j as i32));
                ltall = trees[i][j];
        }
        } 
    }
    
    for j in 0..trees[0].len() {
        let mut ltall = -1;
        for i in (0..trees.len()).rev() {
            if trees[i][j] > ltall {
                visible.insert((i as i32, j as i32));
                ltall = trees[i][j];
            }
        } 
    }
    visible
}
