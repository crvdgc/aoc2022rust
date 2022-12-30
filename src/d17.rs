type P = (usize, usize);
type Block = Vec<Vec<usize>>;

fn blocks() -> Vec<Block> {
    vec![
        vec![vec![0, 1, 2, 3]],                // horizon
        vec![vec![1], vec![0, 1, 2], vec![1]], // cross
        vec![vec![0, 1, 2], vec![2], vec![2]], // l
        vec![vec![0]; 4],                      // vertical
        vec![vec![0, 1], vec![0, 1]],          // rock
    ]
}

fn points(block: &Block, (x, y): P) -> Vec<P> {
    block
        .iter()
        .enumerate()
        .flat_map(|(dx, dys)| dys.iter().map(move |dy| (dx, dy)))
        .map(|(dx, dy)| (x + dx, y + dy))
        .collect()
}

fn shift(is_left: bool, )

pub fn part0(input: &str) {
    let blocks = blocks();
    let points: Vec<_> = blocks.iter().map(|block| |p| points(block, p)).collect();
}
pub fn part1(input: &str) {}
pub fn example_input() -> &'static str {
    r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#
}
