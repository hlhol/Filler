use std::collections::HashSet;
use std::io::{self, BufRead, Write};
mod helper;
use crate::helper::*;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let player_line = match lines.next() {
        Some(Ok(line)) => line,
        _ => {
            eprintln!("Failed to read player line");
            println!("0 0");
            return;
        }
    };
    let is_player1 = player_line.contains("p1");
    let player_char = if is_player1 { '@' } else { '$' };

    loop {
        let (board, board_w, board_h) = match read_board(&mut lines) {
            Some(b) => b,
            None => break,
        };

        let (piece, piece_w, piece_h) = match read_piece(&mut lines) {
            Some(p) => p,
            None => break,
        };

        if board_h < piece_h || board_w < piece_w {
            println!("0 0");
            io::stdout().flush().unwrap();
            continue;
        }

        let mut stars = Vec::new();
        for (dy, row) in piece.iter().enumerate() {
            for (dx, c) in row.chars().enumerate() {
                if c == 'O' {
                    stars.push((dx, dy));
                }
            }
        }

        //get player cell positions
        let our_pieces = get_player_territory(&board, player_char);
        let n_our = our_pieces.len();
        let n_stars = stars.len();

        if n_stars == 0 {
            println!("0 0");
            io::stdout().flush().unwrap();
            continue;
        }

        // check the distance betwenn your cell and the opponinte 
        let dist_map = build_distance_map(&board, opponent_char(player_char));

        // selecting search in near or fulll
        let cost_territory = n_our * n_stars * n_stars;
        let cost_grid = (board_h - piece_h + 1) * (board_w - piece_w + 1) * n_stars;
        let use_territory_search = cost_territory < cost_grid;

        let mut best_score = i32::MAX;
        let mut best_move = None;

        if use_territory_search {
            let mut candidates = HashSet::new();
            for &(x, y) in &our_pieces {
                for &(dx, dy) in &stars {
                    if x >= dx && y >= dy {
                        let x0 = x - dx;
                        let y0 = y - dy;
                        if x0 <= board_w - piece_w && y0 <= board_h - piece_h {
                            candidates.insert((x0, y0));
                        }
                    }
                }
            }

            for (x, y) in candidates {
                if is_valid_placement(&board, &stars, x, y, player_char) {
                    let score = placement_score(&stars, x, y, board_h, board_w, &dist_map);
                    if score < best_score {
                        best_score = score;
                        best_move = Some((x, y));
                    }
                }
            }
        } else {
            for y in 0..=(board_h - piece_h) {
                for x in 0..=(board_w - piece_w) {
                    if is_valid_placement(&board, &stars, x, y, player_char) {
                        let score = placement_score(&stars, x, y, board_h, board_w, &dist_map);
                        if score < best_score {
                            best_score = score;
                            best_move = Some((x, y));
                        }
                    }
                }
            }
        }

        match best_move {
            Some((x, y)) => println!("{} {}", x, y),
            None => println!("0 0"),
        }
        io::stdout().flush().unwrap();
    }
}
