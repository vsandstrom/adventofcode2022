mod monkey;
use monkey::{populate_monkeytown, populate_monkeytest, Monkey, calc_monkeybusiness};


fn main() {
    let mut monkeys: Vec<Monkey> = populate_monkeytown();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].op() {
                let next = monkeys[i].test(item);
                let reciever = &mut monkeys[next];
                reciever.items.push(item);
                monkeys[i].mb += 1;
            }
        }
    }

    let monkeybusiness = calc_monkeybusiness(monkeys);
    println!("{}", monkeybusiness);
    
    let mut monkeys: Vec<Monkey> = populate_monkeytown();
    let mut gcd = 1;
    for monkey in monkeys.as_slice() {
        gcd *= monkey.test_value;
    }

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].op2(gcd) {
                let next = monkeys[i].test(item);
                let reciever = &mut monkeys[next];
                reciever.items.push(item);
                monkeys[i].mb += 1;
            }
        }
    }
    let monkeybusiness = calc_monkeybusiness(monkeys);
    println!("{}", monkeybusiness);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut monkeys = populate_monkeytest();

        for _ in 0..20 {
            for i in 0..monkeys.len() {
                while let Some(item) = monkeys[i].op() {
                    let next = monkeys[i].test(item);
                    let reciever = &mut monkeys[next];
                    reciever.items.push(item);
                    monkeys[i].mb += 1;
                }
            }
        }
        assert_eq!([101, 95, 7, 105], [monkeys[0].mb, monkeys[1].mb, monkeys[2].mb, monkeys[3].mb]);
    }

    #[test]
    fn test_2() {
        let mut monkeys = populate_monkeytest();
        let mut gcd = 1;
        for monkey in monkeys.as_slice() {
            gcd *= monkey.test_value;
        }
        for _ in 0..10000 {
            for i in 0..monkeys.len() {
                while let Some(item) = monkeys[i].op2(gcd) {
                    let next = monkeys[i].test(item);
                    let reciever = &mut monkeys[next];
                    reciever.items.push(item);
                    monkeys[i].mb += 1;
                }
            }
        }
        assert_eq!([52166, 47830, 1938, 52013], [monkeys[0].mb, monkeys[1].mb, monkeys[2].mb, monkeys[3].mb]);
    }
}
