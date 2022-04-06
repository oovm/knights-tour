use std::vec::Vec;

// 存储马可以跟随的 8 种可能的移动组合
const DIR_X: [i32; 8] = [2, 1, -1, -2, -2, -1, 1, 2];
const DIR_Y: [i32; 8] = [1, 2, 2, 1, -1, -2, -2, -1];

// 查找马是否可以移动到 (i, j) 这个有效的单元格，它存在于棋盘中
fn is_safe(i: i32, j: i32, n: i32, board: &Vec<Vec<i32>>) -> bool {
    i >= 0 && j >= 0 && i < n && j < n && board[i as usize][j as usize] == 0
}

// 存储是否存在任何有效路径
static mut IS_POSSIBLE: bool = false;

// 递归函数，遍历马可以跟随的所有路径
fn knight_tour(
    chess_board: &mut Vec<Vec<i32>>,
    n: i32,
    x: i32,
    y: i32,
    visited: i32,
) {
    // 标记棋盘的当前方格
    chess_board[x as usize][y as usize] = visited;

    // 如果访问的方格数等于总方格数
    if visited == n * n {
        unsafe {
            IS_POSSIBLE = true;
        }

        // 打印 ChessBoard 的当前状态
        for i in 0..n {
            for j in 0..n {
                print!("{} ", chess_board[i as usize][j as usize]);
            }
            println!();
        }
        println!();

        // 回溯到上一个移动
        chess_board[x as usize][y as usize] = 0;
        return;
    }

    // 遍历马的所有八个可能的移动
    for i in 0..8 {
        // 存储马在移动后的新位置
        let new_x = x + DIR_X[i];
        let new_y = y + DIR_Y[i];

        // 如果新位置是有效位置，则递归调用下一个移动
        if is_safe(new_x, new_y, n, chess_board) && chess_board[new_x as usize][new_y as usize] == 0 {
            knight_tour(chess_board, n, new_x, new_y, visited + 1);
        }
    }

    // 回溯到上一个移动
    chess_board[x as usize][y as usize] = 0;
}

#[test]
fn main() {
    let mut chess_board = vec![vec![0; 5]; 5];
    let n = chess_board.len() as i32;
    let x = 1;
    let y = 1;

    knight_tour(&mut chess_board, n, x - 1, y - 1, 1);

    // 如果不存在有效的移动序列
    unsafe {
        if !IS_POSSIBLE {
            println!("-1");
        }
    }
}
