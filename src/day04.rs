use itertools::{equal, Itertools};

use crate::parse::parse_i32;

#[derive(Clone, Debug)]
pub struct BoardItem {
    num: i32,
    marked: bool,
}
pub type Board = [BoardItem; 25];
#[derive(Clone)]
pub struct Game {
    draws: Vec<i32>,
    boards: Vec<Board>,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Game {
    let draws = input
        .trim()
        .lines()
        .into_iter()
        .next()
        .unwrap()
        .split(',')
        .map(parse_i32)
        .collect();

    let board_chunks = input.trim().lines().skip(2).chunks(6);
    let boards: Vec<Board> = board_chunks
        .into_iter()
        .map(|chunk| {
            let chunk = chunk.into_iter().join(" ");
            let chunk = Vec::from_iter(chunk.split_whitespace().map(|digit| BoardItem {
                num: parse_i32(digit),
                marked: false,
            }));
            let chunk: Board = chunk.try_into().expect("Failed to parse board");
            // dbg!(chunk);
            chunk
        })
        .collect_vec();

    Game { draws, boards }
}

#[aoc(day4, part1)]
pub fn part_1(input: &Game) -> i32 {
    let mut input = input.clone();
    for draw in input.draws.iter() {
        for (board_i, board_clone) in input.boards.clone().iter().enumerate() {
            let found = board_clone.iter().position(|item| item.num == *draw);
            if let Some(i) = found {
                input.boards[board_i][i].marked = true;
            }

            let is_win = check_board_win(&input.boards[board_i]);
            if is_win {
                let calc = calc_won_board(&input.boards[board_i]);
                return calc * draw;
            }
        }
    }
    0
}

fn find_board(boards: &[Board], find_board: &Board) -> Option<usize> {
    let found = boards.iter().position(|board| {
        let pure_board = board.clone().map(|b| b.num);
        let pure_board_compare = find_board.clone().map(|b| b.num);

        equal(pure_board, pure_board_compare)
    });

    found
}

#[aoc(day4, part2)]
pub fn part_2(input: &Game) -> i32 {
    let mut input = input.clone();
    for draw in input.draws.iter() {
        for board_clone in input.boards.clone().iter() {
            let found = board_clone.iter().position(|item| item.num == *draw);
            let real_board_i = find_board(&input.boards, board_clone).unwrap();
            if let Some(i) = found {
                input.boards[real_board_i][i].marked = true;
            }

            let is_win = check_board_win(&input.boards[real_board_i]);
            if is_win {
                if input.boards.len() == 1 {
                    let calc = calc_won_board(&input.boards[real_board_i]);
                    return calc * draw;
                }

                let board_idx_to_remove = find_board(&input.boards, board_clone);
                if let Some(i) = board_idx_to_remove {
                    input.boards.remove(i);
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::parse::read_file;

    use super::*;

    #[test]
    fn test_input_generator() {
        let input = read_file("test_input/input04.txt");
        let game = input_generator(&input);

        assert_eq!(
            game.draws,
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ]
        );
        assert_eq!(game.boards.len(), 3);
        for board in game.boards {
            assert_eq!(board.len(), 25);
            for item in board {
                assert!(item.num >= 0);
                assert_eq!(item.marked, false);
            }
        }
    }

    #[test]
    fn test_part_1() {
        let input = read_file("test_input/input04.txt");
        let game = input_generator(&input);
        let result = part_1(&game);
        assert_eq!(result, 4512);
    }

    #[test]
    fn test_part_2() {
        let input = read_file("test_input/input04.txt");
        let game = input_generator(&input);
        let result = part_2(&game);
        assert_eq!(result, 1924);
    }
}

pub fn check_board_win(board: &Board) -> bool {
    if board[0].marked && board[1].marked && board[2].marked && board[3].marked && board[4].marked {
        return true;
    }
    if board[5].marked && board[6].marked && board[7].marked && board[8].marked && board[9].marked {
        return true;
    }
    if board[10].marked
        && board[11].marked
        && board[12].marked
        && board[13].marked
        && board[14].marked
    {
        return true;
    }
    if board[15].marked
        && board[16].marked
        && board[17].marked
        && board[18].marked
        && board[19].marked
    {
        return true;
    }
    if board[20].marked
        && board[21].marked
        && board[22].marked
        && board[23].marked
        && board[24].marked
    {
        return true;
    }

    if board[0].marked
        && board[5].marked
        && board[10].marked
        && board[15].marked
        && board[20].marked
    {
        return true;
    }
    if board[1].marked
        && board[6].marked
        && board[11].marked
        && board[16].marked
        && board[21].marked
    {
        return true;
    }
    if board[2].marked
        && board[7].marked
        && board[12].marked
        && board[17].marked
        && board[22].marked
    {
        return true;
    }
    if board[3].marked
        && board[8].marked
        && board[13].marked
        && board[18].marked
        && board[23].marked
    {
        return true;
    }
    if board[4].marked
        && board[9].marked
        && board[14].marked
        && board[19].marked
        && board[24].marked
    {
        return true;
    }

    false
}

pub fn calc_won_board(board: &Board) -> i32 {
    board
        .iter()
        .map(|item| match item.marked {
            true => 0,
            false => item.num,
        })
        .sum()
}
