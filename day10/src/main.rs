mod util;
mod task1;
mod task2;

// fn task2(input: &str, )

fn main() {
    let goal = vec![20, 60, 100, 140, 180, 220];
    let input = util::parse("input.txt").unwrap();
    let res = task1::solve(&input, goal);
    let mut sum = 0;
    for i in res {
        sum += i;
    }

    println!("{}", sum);

    task2::solve(&input);
}
