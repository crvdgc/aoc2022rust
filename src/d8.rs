fn parse_grid(input: &str) -> Vec<Vec<u8>> {
    let mut grid = Vec::new();
    for line in input.lines() {
        grid.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
        )
    }
    grid
}

fn mark_left_visbible(
    visible: &mut Vec<Vec<bool>>,
    grid: &Vec<Vec<u8>>,
    n_row: usize,
    n_col: usize,
) {
    let mut tallest = vec![0; n_row];
    for c in 0..n_col {
        for r in 0..n_row {
            if c == 0 {
                // left edge
                visible[r][c] = true;
                tallest[r] = grid[r][c];
            } else {
                let cur = grid[r][c];
                if cur > tallest[r] {
                    visible[r][c] = true;
                    tallest[r] = cur;
                }
            }
        }
    }
}

fn mark_right_visbible(
    visible: &mut Vec<Vec<bool>>,
    grid: &Vec<Vec<u8>>,
    n_row: usize,
    n_col: usize,
) {
    let mut tallest = vec![0; n_row];
    for rc in 0..n_col {
        let c = n_col - 1 - rc;
        for r in 0..n_row {
            if c == (n_col - 1) {
                // right edge
                visible[r][c] = true;
                tallest[r] = grid[r][c];
            } else {
                let cur = grid[r][c];
                if cur > tallest[r] {
                    visible[r][c] = true;
                    tallest[r] = cur;
                }
            }
        }
    }
}

fn mark_up_visbible(visible: &mut Vec<Vec<bool>>, grid: &Vec<Vec<u8>>, n_row: usize, n_col: usize) {
    let mut tallest = vec![0; n_col];
    for r in 0..n_row {
        for c in 0..n_col {
            if r == 0 {
                // up edge
                visible[r][c] = true;
                tallest[c] = grid[r][c];
            } else {
                let cur = grid[r][c];
                if cur > tallest[c] {
                    visible[r][c] = true;
                    tallest[c] = cur;
                }
            }
        }
    }
}

fn mark_down_visbible(
    visible: &mut Vec<Vec<bool>>,
    grid: &Vec<Vec<u8>>,
    n_row: usize,
    n_col: usize,
) {
    let mut tallest = vec![0; n_col];
    for rr in 0..n_row {
        let r = n_row - 1 - rr;
        for c in 0..n_col {
            if r == (n_row - 1) {
                // up edge
                visible[r][c] = true;
                tallest[c] = grid[r][c];
            } else {
                let cur = grid[r][c];
                if cur > tallest[c] {
                    visible[r][c] = true;
                    tallest[c] = cur;
                }
            }
        }
    }
}

fn print_grid<T>(visible: &Vec<Vec<T>>, printer: fn(&T) -> String) {
    for row in visible {
        for x in row {
            print!("{}", printer(&x))
        }
        print!("\n")
    }
}

fn count_visible(visible: &Vec<Vec<bool>>) -> u32 {
    visible
        .iter()
        .map(|row| row.iter().map(|x| if *x { 1 } else { 0 }).sum::<u32>() as u32)
        .sum()
}

pub fn part0(input: String) {
    let grid = parse_grid(&input);
    // print_grid(&grid, |x| {
    //     char::from_digit(*x as u32, 10).unwrap().to_string()
    // });
    // dbg!(&grid);
    let n_col = grid[0].len();
    let n_row = grid.len();
    let mut visible: Vec<Vec<bool>> = vec![vec![false; n_col]; n_row];
    mark_left_visbible(&mut visible, &grid, n_row, n_col);
    mark_right_visbible(&mut visible, &grid, n_row, n_col);
    mark_up_visbible(&mut visible, &grid, n_row, n_col);
    mark_down_visbible(&mut visible, &grid, n_row, n_col);
    // print_grid(
    //     &visible,
    //     |x| if *x { "O".to_string() } else { "X".to_string() },
    // );
    println!("{}", count_visible(&visible))
}

fn visible_count<I, T>(cur: T, iter: I) -> usize
where
    T: Ord,
    I: Iterator<Item = T>,
{
    let mut acc = 0;
    for v in iter {
        // dbg!(&v);
        if v >= cur {
            acc += 1;
            return acc;
        } else {
            acc += 1;
        }
    }
    acc
}

fn scenic_score(grid: &Vec<Vec<u8>>, r: usize, c: usize) -> usize {
    let n_col = grid[0].len();
    let n_row = grid.len();
    let cur = grid[r][c];
    let up_score = visible_count(cur, (0..r).rev().map(|ri| grid[ri][c]));
    let down_score = visible_count(cur, (r + 1..n_row).map(|ri| grid[ri][c]));
    let left_score = visible_count(cur, (0..c).rev().map(|ci| grid[r][ci]));
    let right_score = visible_count(cur, (c + 1..n_col).map(|ci| grid[r][ci]));
    // dbg!(up_score, down_score, left_score, right_score);
    up_score * down_score * left_score * right_score
}

pub fn part1(input: String) {
    let grid = parse_grid(&input);
    let n_col = grid[0].len();
    let n_row = grid.len();
    // dbg!(scenic_score(&grid, 1, 2));
    // dbg!(scenic_score(&grid, 3, 2));
    // panic!("bye");
    let ans = (0..n_row)
        .map(|r| (0..n_col).map(|c| scenic_score(&grid, r, c)).max().unwrap())
        .max()
        .unwrap();
    println!("{}", ans)
}

pub fn example_input() -> String {
    r#"30373
25512
65332
33549
35390"#
        .to_string()
}
