use std::io;

pub fn read_board(lines: &mut impl Iterator<Item = io::Result<String>>) -> Option<(Vec<String>, usize, usize)> {
    let mut board = Vec::new();
    let (mut board_w, mut board_h) = (0, 0);

    while let Some(Ok(line)) = lines.next() {
        if line.starts_with("Anfield") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 3 {
                eprintln!("Malformed Anfield line");
                return None;
            }
            board_w = parts[1].parse().unwrap_or(0);
            board_h = parts[2].trim_end_matches(':').parse().unwrap_or(0);
            
            if let Some(Ok(_)) = lines.next() {
                for _ in 0..board_h {
                    if let Some(Ok(row_line)) = lines.next() {
                        if row_line.len() < 4 {
                            eprintln!("Malformed board row");
                            continue;
                        }
                        board.push(row_line[4..].to_string());
                    }
                }
            }
            return Some((board, board_w, board_h));
        }
    }
    None
}

pub fn read_piece(lines: &mut impl Iterator<Item = io::Result<String>>) -> Option<(Vec<String>, usize, usize)> {
    let mut piece = Vec::new();
    let (mut piece_w, mut piece_h) = (0, 0);

    while let Some(Ok(line)) = lines.next() {
        if line.starts_with("Piece") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 3 {
                eprintln!("Malformed Piece line");
                return None;
            }
            piece_w = parts[1].parse().unwrap_or(0);
            piece_h = parts[2].trim_end_matches(':').parse().unwrap_or(0);
            
            for _ in 0..piece_h {
                if let Some(Ok(piece_line)) = lines.next() {
                    piece.push(piece_line);
                }
            }
            return Some((piece, piece_w, piece_h));
        }
    }
    None
}

pub fn get_player_territory(board: &[String], player_char: char) -> Vec<(usize, usize)> {
    let mut territory = Vec::new();
    let symbols = if player_char == '@' {
        ['@', 'a']
    } else {
        ['$', 's']
    };

    for (y, row) in board.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if symbols.contains(&c) {
                territory.push((x, y));
            }
        }
    }
    territory
}

pub fn opponent_char(player: char) -> char {
    if player == '@' { '$' } else { '@' }
}
 // search algorithm for opponent cell
pub fn build_distance_map(board: &[String], opp_char: char) -> Vec<Vec<i32>> {
    let h = board.len();
    let w = board[0].len();
    let mut dist = vec![vec![i32::MAX; w]; h];
    let mut queue = std::collections::VecDeque::new();
    let opp_symbols = if opp_char == '@' {
        ['@', 'a']
    } else {
        ['$', 's']
    };

    for (y, row) in board.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if opp_symbols.contains(&c) {
                dist[y][x] = 0;
                queue.push_back((x, y));
            }
        }
    }

    while let Some((x, y)) = queue.pop_front() {
        let current = dist[y][x];
        for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < w as i32 && ny >= 0 && ny < h as i32 {
                let nx = nx as usize;
                let ny = ny as usize;
                if dist[ny][nx] > current + 1 {
                    dist[ny][nx] = current + 1;
                    queue.push_back((nx, ny));
                }
            }
        }
    }
    dist
}

pub fn is_valid_placement( // ok to place 
    board: &[String],
    stars: &[(usize, usize)],
    x: usize,
    y: usize,
    player_char: char,
) -> bool {
    let my_symbols = if player_char == '@' {
        ['@', 'a']
    } else {
        ['$', 's']
    };
    let opp_symbols = if player_char == '@' {
        ['$', 's']
    } else {
        ['@', 'a']
    };
    let mut overlap_count = 0;

    for &(dx, dy) in stars {
        let bx = x + dx;
        let by = y + dy;
        let cell = board[by].as_bytes()[bx] as char;
        
        if my_symbols.contains(&cell) {
            overlap_count += 1;
        } else if opp_symbols.contains(&cell) || cell != '.' {
            return false;
        }
    }
    overlap_count == 1
}

pub fn placement_score( //count score 
    stars: &[(usize, usize)],
    x: usize,
    y: usize,
    board_h: usize,
    board_w: usize,
    dist_map: &[Vec<i32>],
) -> i32 {
    let mut score = 0;
    for &(dx, dy) in stars {
        let bx = x + dx;
        let by = y + dy;
        if by < board_h && bx < board_w {
            let dist = dist_map[by][bx];
            if dist == i32::MAX {
                score += 1_000_000;
            } else {
                score += dist;
            }
        } else {
            score += 1_000_000;
        }
    }
    score
}