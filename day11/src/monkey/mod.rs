#[derive (Debug, Default, Clone)]
pub struct Monkey {
    pub items: Vec<i64>,
    op_value: i64,
    delim: char,
    test_value: i64,
    r#true: usize,
    r#false: usize,
    pub mb: i64,
}

impl Monkey {
    fn new(items: Vec<i64>, op_value: i64, delim: char, test_value: i64, r#true: usize, r#false: usize) -> Monkey {
        Monkey { items, op_value, delim, test_value, r#true, r#false, mb: 0 }
    }

    pub fn test(&self, item: i64) -> usize {
        // tar reda på vilken apa objektet ska skickas till
        if item % &self.test_value == 0 {
            return self.r#true
        } else {
            return self.r#false
        }
    }

    pub fn op(&mut self) -> Option<i64> {
        if self.items.len() > 0 {
            let mut item = self.items.remove(0);
            match self.delim {
                '*' => { if self.op_value == -1 {
                        item *= item;
                    } else {
                        item *= self.op_value;
                    } 
                },
                '+' =>  {
                    item += self.op_value ;
                },
                _ => panic!("damn")
            }

            return Some(item / 3);
        }
        None

    }
}

pub fn populate_monkeytown() -> Vec<Monkey> { 
    let mut monkeylist: Vec<Monkey> = vec!();

    // 0
    let m = Monkey::new(
                        [89, 95, 92, 64, 87, 68].to_vec(),
                        11, 
                        '*', 
                        2, 
                        7, 
                        4);
    monkeylist.push(m);
    
    // 1
    let m = Monkey::new(
                        [87, 67].to_vec(),
                        1, 
                        '+', 
                        13, 
                        3, 
                        6);
    monkeylist.push(m);

    // 2
    let m = Monkey::new(
                        [95, 79, 92, 82, 60].to_vec(),
                        6, 
                        '+', 
                        3, 
                        1, 
                        6);
    monkeylist.push(m);

    // 3
    let m = Monkey::new(
                        [67, 97, 56].to_vec(),
                        -1, 
                        '*', 
                        17, 
                        7, 
                        0);
    monkeylist.push(m);
    
    // 4
    let m = Monkey::new(
                        [80, 68, 87, 94, 61, 59, 50, 68].to_vec(),
                        7, 
                        '*', 
                        19, 
                        5, 
                        2);
    monkeylist.push(m);
    
    // 5
    let m = Monkey::new(
                        [73, 51, 76, 59].to_vec(),
                        8, 
                        '+', 
                        7, 
                        2, 
                        1);
    monkeylist.push(m);
    
    // 6
    let m = Monkey::new(
                        [92].to_vec(),
                        5, 
                        '+', 
                        11, 
                        3, 
                        0);
    monkeylist.push(m);

    // 7
    let m = Monkey::new(
                        [99, 76, 78, 76, 79, 90, 89].to_vec(),
                        7, 
                        '+', 
                        5, 
                        4, 
                        5);
    monkeylist.push(m);

    monkeylist
}


pub fn populate_monkeytest() -> Vec<Monkey> { 
    let mut monkeylist: Vec<Monkey> = vec!();

    // 0
    let m = Monkey::new(
                        [79, 98].to_vec(),
                        19, 
                        '*', 
                        23, 
                        2, 
                        3);
    monkeylist.push(m);
    
    // 1
    let m = Monkey::new(
                        [54, 65, 75, 74].to_vec(),
                        6, 
                        '+', 
                        19, 
                        2, 
                        0);
    monkeylist.push(m);

    // 2
    let m = Monkey::new(
                        [79, 60, 97].to_vec(),
                        -1, 
                        '*', 
                        13, 
                        1, 
                        3);
    monkeylist.push(m);

    // 3
    let m = Monkey::new(
                        [74].to_vec(),
                        3, 
                        '+', 
                        17, 
                        0, 
                        1);
    monkeylist.push(m);

    monkeylist
}
