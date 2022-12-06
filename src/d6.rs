fn first_n_diff<T>(n: usize, cs: T) -> usize
where
    T: Iterator<Item = char>,
{
    let mut buf: Vec<char> = Vec::with_capacity(n - 1);
    for (i, c) in cs.enumerate() {
        match buf.iter().position(|&x| x == c) {
            None => {
                if buf.len() == n - 1 {
                    return i;
                } else {
                    buf.push(c)
                }
            }
            Some(j) => {
                buf.drain(0..j + 1);
                buf.push(c);
            }
        }
    }
    panic!("Not found")
}

pub fn part0(input: String) -> () {
    let pos = first_n_diff(4, input.chars());
    print!("{}", pos + 1)
}

pub fn part1(input: String) -> () {
    let pos = first_n_diff(14, input.chars());
    print!("{}", pos + 1)
}

pub fn example_input() -> Vec<&'static str> {
    vec![
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ]
}
