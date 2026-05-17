use std::fmt;

/*
        0  1  2  3  4  5  6  7
    7   T  H  B  Q  K  B  H  T
    6   P  P  P  P  P  P  P  P
    5
    4
    3
    2
    1   P  P  P  P  P  P  P  P
    0   T  H  B  Q  K  B  H  T
*/

#[derive(Debug, Clone, Copy, PartialEq)]
enum PieceType {
    PAWN,
    TOWER,
    HORSE,
    BISHOP,
    QUEEN,
    KING,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    WHITE,
    BLACK,
}

fn check_break(board: &BoardState, hor: usize, ver: usize, color: Color) -> (bool, bool) {
    let mut stop_now = false;
    let mut stop_next = false;

    if board.is_occupied(hor, ver) {
        if !board.is_enemy(hor, ver, color) {
            stop_now = true;
            stop_next = true;
        } else {
            stop_now = false;
            stop_next = true;
        }
    }

    (stop_now, stop_next)
}

#[derive(Debug, Clone, Copy)]
struct Piece {
    piece_type: PieceType,
    color: Color,
    is_virgin_status: bool,
}

#[inline(always)]
fn valid_pos(hor: isize, ver: isize) -> bool {
    (0..8).contains(&hor) && (0..8).contains(&ver)
}

fn process_move(
    board: &mut BoardState,
    output: &mut Vec<BoardState>,
    current_hor: usize,
    current_ver: usize,
    hor: isize,
    ver: isize,
    color: Color,
) -> bool {
    if valid_pos(hor, ver) {
        let hor = hor as usize;
        let ver = ver as usize;

        let mut new_board = *board;

        let (stop_now, stop_next) = check_break(&new_board, hor, ver, color);

        if !stop_now
            && new_board
                .move_piece(current_hor, current_ver, hor, ver)
                .is_ok()
        {
            new_board.switch();
            new_board.increment_turn();
            new_board.unvirgin(hor, ver);
            output.push(new_board);
        }

        return stop_next;
    }

    false
}

impl Piece {
    fn new(piece_type: PieceType, color: Color) -> Self {
        Self {
            piece_type,
            color,
            is_virgin_status: true,
        }
    }

    fn get_piece_type(&self) -> &PieceType {
        return &self.piece_type
    }

    fn get_symbol(&self) -> char {
        match self.piece_type {
            PieceType::PAWN => 'P',
            PieceType::TOWER => 'T',
            PieceType::HORSE => 'H',
            PieceType::BISHOP => 'B',
            PieceType::QUEEN => 'Q',
            PieceType::KING => 'K',
        }
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn is_virgin(&self) -> bool {
        self.is_virgin_status
    }

    fn unvirgin(&mut self) {
        self.is_virgin_status = false;
    }

    fn get_score(&self) -> isize {
        match self.piece_type {
            PieceType::PAWN => 1,
            PieceType::TOWER => 5,
            PieceType::HORSE => 3,
            PieceType::BISHOP => 3,
            PieceType::QUEEN => 9,
            PieceType::KING => 1000,
        }
    }

    fn generate_moves(
        &self,
        mut board: &mut BoardState,
        current_hor: usize,
        current_ver: usize,
    ) -> Vec<BoardState> {
        let mut output = Vec::new();

        match self.piece_type {
            PieceType::PAWN => {
                let new_ver = match self.color {
                    Color::WHITE => current_ver as isize + 1,
                    Color::BLACK => current_ver as isize - 1,
                };

                for i in -1isize..2 {
                    let hor: isize = current_hor as isize + i;
                    let mut is_ok: bool = true;
                    if valid_pos(hor, new_ver as isize) {
                        if i != 0 {
                            if !board.is_enemy(hor as usize, new_ver as usize, self.color) {
                                is_ok = false;
                            }
                        } else {
                            if board.is_occupied(hor as usize, new_ver as usize) {
                                is_ok = false;
                            }
                        }
                    }

                    if is_ok {
                        process_move(
                            &mut board,
                            &mut output,
                            current_hor,
                            current_ver,
                            hor,
                            new_ver as isize,
                            self.color,
                        );
                    }
                }

                if self.is_virgin_status {
                    let new_iver: isize = match self.color {
                        Color::WHITE => { current_ver as isize + 1 }
                        Color::BLACK => { current_ver as isize - 1 }
                    };

                    let new_ver: isize = match self.color {
                        Color::WHITE => { current_ver as isize + 2 }
                        Color::BLACK => { current_ver as isize - 2 }
                    };

                    if valid_pos(current_hor as isize, new_ver) && !board.is_occupied(current_hor, new_ver as usize) && !board.is_occupied(current_hor, new_iver as usize) {
                        process_move(
                            &mut board,
                            &mut output,
                            current_hor,
                            current_ver,
                            current_hor as isize,
                            new_ver,
                            self.color,
                        );
                    }
                }
            }
            PieceType::TOWER => {
                let mut stop: [bool; 8] = [false, false, false, false, false, false, false, false];

                for step in 1..8 {
                    let mut count: usize = 0;
                    for i in -1isize..2 {
                        for j in -1isize..2 {
                            if i != 0 && j != 0 {
                                continue;
                            }

                            if !stop[count] {
                                stop[count] = process_move(
                                    &mut board,
                                    &mut output,
                                    current_hor,
                                    current_ver,
                                    current_hor as isize + i * step,
                                    current_ver as isize + j * step,
                                    self.color,
                                );
                            }
                            count += 1;
                        }
                    }
                }
            }
            PieceType::BISHOP => {
                let mut stop: [bool; 8] = [false, false, false, false, false, false, false, false];

                for step in 1..8 {
                    let mut count: usize = 0;
                    for i in -1isize..2 {
                        for j in -1isize..2 {
                            if i == 0 || j == 0 {
                                continue;
                            }

                            if !stop[count] {
                                stop[count] = process_move(
                                    &mut board,
                                    &mut output,
                                    current_hor,
                                    current_ver,
                                    current_hor as isize + i * step,
                                    current_ver as isize + j * step,
                                    self.color,
                                );
                            }
                            count += 1;
                        }
                    }
                }
            }
            PieceType::QUEEN => {
                let mut stop: [bool; 8] = [false, false, false, false, false, false, false, false];

                for step in 1..8 {
                    let mut count: usize = 0;
                    for i in -1isize..2 {
                        for j in -1isize..2 {
                            if i == 0 && j == 0 {
                                continue;
                            }

                            if !stop[count] {
                                stop[count] = process_move(
                                    &mut board,
                                    &mut output,
                                    current_hor,
                                    current_ver,
                                    current_hor as isize + i * step,
                                    current_ver as isize + j * step,
                                    self.color,
                                );
                            }
                            count += 1;
                        }
                    }
                }
            }
            PieceType::KING => {
                for i in -1isize..2 {
                    for j in -1isize..2 {
                        if i == 0 && j == 0 {
                            continue;
                        }

                        process_move(
                            &mut board,
                            &mut output,
                            current_hor,
                            current_ver,
                            current_hor as isize + i,
                            current_ver as isize + j,
                            self.color,
                        );
                    }
                }
            }
            PieceType::HORSE => {
                for i in [-2, 2] {
                    for j in [-1, 1] {
                        process_move(
                            &mut board,
                            &mut output,
                            current_hor,
                            current_ver,
                            current_hor as isize + i,
                            current_ver as isize + j,
                            self.color,
                        );
                        process_move(
                            &mut board,
                            &mut output,
                            current_hor,
                            current_ver,
                            current_hor as isize + j,
                            current_ver as isize + i,
                            self.color,
                        );                         
                    } 
                }
            }
        }

        output
    }

    fn is_checking(&self, board: &BoardState, current_hor: usize, current_ver: usize, target_hor: usize, target_ver: usize) -> bool {
        match self.piece_type {
            PieceType::PAWN => {
                let new_ver: isize = match self.color {
                    Color::WHITE => { current_ver as isize + 1 }
                    Color::BLACK => { current_ver as isize - 1 }
                };

                for i in [-1, 1] {
                    let new_hor = current_hor as isize + i;
                    if valid_pos(new_hor, new_ver) && new_hor as usize == target_hor && new_ver as usize == target_ver {
                        return true;
                    }
                }

                false
            }
            PieceType::TOWER => {
                if current_hor == target_hor {
                    let min = current_ver.min(target_ver);
                    let max = current_ver.max(target_ver);

                    for i in (min+1)..max {
                        if board.is_occupied(current_hor, i) {
                            return false;
                        }
                    }

                    return true;
                } else if current_ver == target_ver {
                    let min = current_hor.min(target_hor);
                    let max = current_hor.max(target_hor);

                    for i in (min+1)..max {
                        if board.is_occupied(i, current_ver) {
                            return false;
                        }
                    }

                    return true;
                }

                false
            }
            PieceType::BISHOP => {
                if (current_ver as isize - target_ver as isize).abs() == (current_hor as isize - target_hor as isize).abs() {
                    let dx: isize = if target_hor > current_hor { 1 } else { -1 };
                    let dy: isize = if target_ver > current_ver { 1 } else { -1 };

                    let mut x = current_hor as isize + dx;
                    let mut y = current_ver as isize + dy;

                    while x != target_hor as isize {
                        if board.is_occupied(x as usize, y as usize) {
                            return false;
                        }

                        x += dx;
                        y += dy;
                    }

                    return true;
                }

                false
            }
            PieceType::QUEEN => {
                if current_hor == target_hor {
                    let min = current_ver.min(target_ver);
                    let max = current_ver.max(target_ver);

                    for i in (min+1)..max {
                        if board.is_occupied(current_hor, i) {
                            return false;
                        }
                    }

                    return true;
                } else if current_ver == target_ver {
                    let min = current_hor.min(target_hor);
                    let max = current_hor.max(target_hor);

                    for i in (min+1)..max {
                        if board.is_occupied(i, current_ver) {
                            return false;
                        }
                    }

                    return true;
                } else if (current_ver as isize - target_ver as isize).abs() as usize == (current_hor as isize - target_hor as isize).abs() as usize {
                    let dx: isize = if target_hor > current_hor { 1 } else { -1 };
                    let dy: isize = if target_ver > current_ver { 1 } else { -1 };

                    let mut x = current_hor as isize + dx;
                    let mut y = current_ver as isize + dy;

                    while x != target_hor as isize {
                        if board.is_occupied(x as usize, y as usize) {
                            return false;
                        }

                        x += dx;
                        y += dy;
                    }

                    return true;
                }
                
                false
            }
            PieceType::HORSE => {
                for i in [-2, 2] {
                    for j in [-1, 1] {
                        let hor = current_hor as isize + i;
                        let ver = current_ver as isize + j;
                        if valid_pos(hor, ver) && target_hor == hor as usize && target_ver == ver as usize {
                            return true;
                        }

                        let hor = current_hor as isize + j;
                        let ver = current_ver as isize + i;
                        if valid_pos(hor, ver) && target_hor == hor as usize && target_ver == ver as usize {
                            return true;
                        }
                    } 
                }

                false
            }
            PieceType::KING => {
                if (current_hor as isize - target_hor as isize).abs() <= 1 && (current_ver as isize - target_ver as isize).abs() <= 1 {
                    return true;
                }

                false
            }
        }
    }
}

#[derive(Clone, Copy)]
struct BoardState {
    board: [Option<Piece>; 64],
    turn_color: Color,
    turn_number: usize,
    white_king_pos: usize,
    black_king_pos: usize,
}

impl BoardState {
    fn new(turn_color: Color) -> Self {
        Self {
            board: [None; 64],
            turn_color,
            turn_number: 0,
            white_king_pos: 65,
            black_king_pos: 65,
        }
    }

    fn is_king_checked(&self) -> bool {
        let king_pos = match self.turn_color {
            Color::WHITE => { self.white_king_pos }
            Color::BLACK => { self.black_king_pos }
        };

        if king_pos > 65 {
            return true;
        }

        let king_hor = king_pos % 8;
        let king_ver = king_pos / 8;

        for hor in 1..8 {
            for ver in 1..8 {
                let index = ver * 8 + hor;
                match self.board[index] {
                    Some(piece) => {
                        if self.is_enemy(hor, ver, self.turn_color) && piece.is_checking(&self, hor, ver, king_hor, king_ver) {
                            return true;
                        }
                    }
                    None => {}
                }
            }
        }

        false
    }

    fn increment_turn(&mut self) {
        self.turn_number += 1;
    }

    fn add_piece(&mut self, hor: usize, ver: usize, piece: Piece) {
        self.board[ver * 8 + hor] = Some(piece);
        if *piece.get_piece_type() == PieceType::KING {
            match piece.get_color() {
                Color::WHITE => { self.white_king_pos = ver * 8 + hor; }
                Color::BLACK => { self.black_king_pos = ver * 8 + hor; }
            }
        }
    }

    fn is_occupied(&self, hor: usize, ver: usize) -> bool {
        self.board[ver * 8 + hor].is_some()
    }

    fn is_enemy(&self, hor: usize, ver: usize, color: Color) -> bool {
        match self.board[ver * 8 + hor] {
            Some(piece) => piece.color != color,
            None => false,
        }
    }

    fn is_virgin(&self, hor: usize, ver: usize) -> bool {
        match self.board[ver * 8 + hor] {
            Some(piece) => piece.is_virgin(),
            None => false,
        }
    }

    fn unvirgin(&mut self, hor: usize, ver: usize) {
        if let Some(piece) = self.board[ver * 8 + hor].as_mut() {
            piece.unvirgin();
        }
    }

    fn get_turn_color(&self) -> Color {
        return self.turn_color.clone();
    }

    fn evaluate(&self) -> isize {
        let mut score = 0;

        for square in self.board {
            if let Some(piece) = square {
                match piece.color {
                    Color::WHITE => { score += piece.get_score(); }
                    Color::BLACK => { score -= piece.get_score(); }
                }
            }
        }

        score
    }

    fn minimax(&mut self, depth: usize, maximizing: bool) -> isize {
        if depth == 0 {
            return self.evaluate();
        }

        let moves = self.generate_moves();

        if moves.is_empty() {
            return self.evaluate();
        }

        let mut best = match maximizing {
            true => isize::MIN,
            false => isize::MAX,
        };

        for mut mv in moves {
            let score = mv.minimax(depth - 1, !maximizing);

            if score < best {
                best = score;
            }
        }

        best
    }

    fn move_piece(
        &mut self,
        from_hor: usize,
        from_ver: usize,
        to_hor: usize,
        to_ver: usize,
    ) -> Result<(), &'static str> {
        let from_index = from_ver * 8 + from_hor;
        let to_index = to_ver * 8 + to_hor;

        let piece = self.board[from_index];
        match piece {
            Some(piece) => {
                let piece_type = piece.get_piece_type();

                if *piece_type == PieceType::KING {
                    let piece_color = piece.get_color();
                    match piece_color {
                        Color::WHITE => { self.white_king_pos = to_index; }
                        Color::BLACK => { self.black_king_pos = to_index; }
                    }
                }
            }
            None => { return Err("No piece here"); }
        }

        let piece = self.board[from_index].take();
        self.board[to_index] = piece;

        if self.is_king_checked() {
            return Err("King is checked");
        }

        Ok(())
    }

    fn switch(&mut self) {
        self.turn_color = match self.turn_color {
            Color::WHITE => Color::BLACK,
            Color::BLACK => Color::WHITE,
        };
    }

    fn generate_moves(&mut self) -> Vec<BoardState> {
        let mut moves = Vec::new();

        for ver in 0..8 {
            for hor in 0..8 {
                if let Some(piece) = self.board[ver * 8 + hor] {
                    if piece.color == self.turn_color {
                        moves.extend(piece.generate_moves(self, hor, ver));
                    }
                }
            }
        }

        moves
    }
}

impl fmt::Display for BoardState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "    0 1 2 3 4 5 6 7")?;

        for ver in (0..8).rev() {
            write!(f, "{} | ", ver)?;

            for hor in 0..8 {
                let symbol = match self.board[ver * 8 + hor] {
                    Some(piece) => {
                        let c = piece.get_symbol();

                        match piece.color {
                            Color::WHITE => c,
                            Color::BLACK => c.to_ascii_lowercase(),
                        }
                    }
                    None => '.',
                };

                write!(f, "{} ", symbol)?;
            }

            writeln!(f)?;
        }

        writeln!(f)?;
        writeln!(f, "Evaluation: {}", self.evaluate())?;
        match self.turn_color {
            Color::WHITE => writeln!(f, "Move by: BLACK")?,
            Color::BLACK => writeln!(f, "Move by: WHITE")?,
        }
        writeln!(f, "Turn: {}", self.turn_number)?;
        // writeln!(f, "White king: {} {}", self.white_king_pos % 8, self.white_king_pos / 8);
        // writeln!(f, "Black king: {} {}", self.black_king_pos % 8, self.black_king_pos / 8);
        writeln!(f, "-------------------");

        Ok(())
    }
}

fn main() {
    let mut board = BoardState::new(Color::WHITE);
    let piece_types = [
        PieceType::TOWER,
        PieceType::HORSE,
        PieceType::BISHOP,
        PieceType::QUEEN,
        PieceType::KING,
        PieceType::BISHOP,
        PieceType::HORSE,
        PieceType::TOWER,
    ];

    // White pieces
    for i in 0..8 {
        board.add_piece(i, 0, Piece::new(piece_types[i], Color::WHITE));
        board.add_piece(i, 1, Piece::new(PieceType::PAWN, Color::WHITE));
    }

    // Black pieces
    for i in 0..8 {
        board.add_piece(i, 7, Piece::new(piece_types[i], Color::BLACK));
        board.add_piece(i, 6, Piece::new(PieceType::PAWN, Color::BLACK));
    }
   
    println!("{}", board);
    
    let mut winner: Color = Color::WHITE;
    let mut current = board;
    for _ in 0..999 {
        let mut moves = current.generate_moves();

        if moves.is_empty() {
            println!("{:?} WINS", winner);
            break;
        }

        let maximizing = current.turn_color == Color::WHITE;
        let depth = match maximizing {
            true => 1,
            false => 3,
        };

        let mut best_index = 0;
        let mut best_score = isize::MIN;

        for (index, board) in moves.iter_mut().enumerate() {
            let score = board.minimax(depth, !maximizing);

            if score > best_score {
                best_score = score;
                best_index = index;
            }
        }

        winner = current.get_turn_color();
        current = moves.swap_remove(best_index);
        println!("{}", current);
    }
}
