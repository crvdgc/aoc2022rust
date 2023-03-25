fn mix(numbers: &Vec<i64>, indices: &mut [usize]) {
    // dbg!(&numbers);
    let l = numbers.len();
    // dbg!(l);
    for i in 0..l {
        let cur_index = indices[i];
        let new_index = (numbers[i] + cur_index as i64).rem_euclid((l - 1) as i64) as usize;
        // if numbers[i] == -3 {
        //     dbg!(numbers[i], cur_index, new_index);
        // }
        use std::cmp::Ordering;
        match cur_index.cmp(&new_index) {
            Ordering::Less => {
                for i in indices.iter_mut() {
                    if *i > cur_index && *i <= new_index {
                        *i -= 1;
                    }
                }
            }
            Ordering::Greater => {
                for i in indices.iter_mut() {
                    if *i >= new_index && *i < cur_index {
                        *i += 1;
                    }
                }
            }
            Ordering::Equal => {}
        }
        indices[i] = new_index;

        // let mut mixed = vec![0; numbers.len()];
        // for (i, &n) in numbers.iter().enumerate() {
        //     mixed[indices[i]] = n;
        // }
        // dbg!(mixed);
    }
}
pub fn part0(input: &str) {
    let numbers = parse_input(input);
    let l = numbers.len();
    let mut indicies: Vec<usize> = (0..l).collect();
    mix(&numbers, &mut indicies);
    let mut mixed = vec![0; numbers.len()];
    for (i, &n) in numbers.iter().enumerate() {
        mixed[indicies[i]] = n;
    }
    // dbg!(&mixed);
    let n_idx = mixed.iter().position(|&n| n == 0).unwrap();
    // dbg!(n_idx);
    let coord: i64 = vec![1000, 2000, 3000]
        .iter()
        .map(|i| mixed[(i + n_idx) % l])
        // .map(|i| dbg!(i))
        .sum();
    // dbg!(mixed);
    println!("{}", coord);
}

pub fn part1(input: &str) {
    let numbers = {
        let mut numbers = parse_input(input);
        const KEY: i64 = 811589153;
        for number in numbers.iter_mut() {
            *number *= KEY;
        }
        numbers
    };
    let mut indicies: Vec<usize> = (0..numbers.len()).collect();
    for _ in 0..10 {
        mix(&numbers, &mut indicies);
        // let mut mixed = vec![0; numbers.len()];
        // for (i, &n) in numbers.iter().enumerate() {
        //     mixed[indicies[i]] = n;
        // }
        // dbg!(mixed);
    }
    let mut mixed = vec![0; numbers.len()];
    for (i, &n) in numbers.iter().enumerate() {
        mixed[indicies[i]] = n;
    }
    // dbg!(&mixed);
    let n_idx = mixed.iter().position(|&n| n == 0).unwrap();
    let l = numbers.len();
    let coord: i64 = vec![1000, 2000, 3000]
        .iter()
        .map(|i| mixed[(i + n_idx) % l])
        // .map(|i| dbg!(i))
        .sum();
    println!("{}", coord);
}

fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn example_input() -> &'static str {
    r#"1
2
-3
3
-2
0
4"#
}
