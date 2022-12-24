mod monkey;
use monkey::{populate_monkeytown, populate_monkeytest, Monkey};


fn main() {
    let mut monkeys: Vec<Monkey> = populate_monkeytown();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].op() {
                let next = monkeys[i].test(item) -1;
                let reciever = &mut monkeys[next];
                reciever.items.push(item);
                reciever.mb += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut monkeys = populate_monkeytest();
        // println!("{:?}", monkeys);
        for _ in 0..20 {
            for i in 0..monkeys.len() {
                // for i in 0..monkeys[i].items.len() {
                    while let Some(item) = monkeys[i].op() {
                        let next = monkeys[i].test(item);
                        let reciever = &mut monkeys[next];
                        reciever.items.push(item);
                        reciever.mb += 1;
                    }
                // }
            }
        }
        for monkey in monkeys {
            println!("{}", monkey.mb);
        }
    }
}
