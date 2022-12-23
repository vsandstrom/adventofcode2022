    pub fn sort(input: &str) -> Vec<u32> {
        let mut f: Vec<u32> = vec!();
        let mut r: Vec<u32> = vec!();
        let mut sum: u32 = 0;

        for line in input.lines() {
            if line == "$ cd .." {
                r.push(sum);
                sum += f.pop().unwrap();
            } else if &line[0..=3] == "$ cd" {
                f.push(sum);
                sum = 0;
            } else {
                let mem = line.split(" ").nth(0).unwrap().parse().unwrap_or(0);
                sum += mem;
            }
        }

        loop {
            let a = f.pop();
            match a {
                Some(dir) => {
                    r.push(sum);
                    sum += dir;
                },

                None => {break}
            }
        }

        r.sort();
        r
    }

    pub fn task1(input: &str) -> u32 {

        let result = sort(input);
        let mut sum = 0;
        
        for i in result {
            if i > 100000 { break; }
            sum += i;
        }
        return sum;
    }

    pub fn task2(input: &str) -> u32 {

        let result = sort(input);

        let max = result.last().unwrap();
        let req = 30_000_000 - (70_000_000 - max);

        for i in result {
            if i >= req {
                return i;
            }
        }
        return 0;
    }
