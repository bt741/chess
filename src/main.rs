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
        } else {
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
        new_board.switch();
        new_board.increment_turn();

        let (stop_now, stop_next) = check_break(&new_board, hor, ver, color);

        if !stop_now
            && new_board
                .move_piece(current_hor, current_ver, hor, ver)
                .is_ok()
        {
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
            PieceType::KING => 100,
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
                let max_ver = match self.is_virgin_status {
                    true => 3,
                    false => 2,
                };

                for ver_extra in 1..max_ver {
                    let new_ver = match self.color {
                        Color::WHITE => current_ver + ver_extra,
                        Color::BLACK => {
                            if current_ver == 0 {
                                return output;
                            }
                            current_ver - 1
                        }
                    };

                    for i in -1isize..2 {
                        let hor: isize = current_hor as isize + i;
                        let mut is_ok: bool = true;
                        if valid_pos(hor, new_ver as isize) {
                            if i != 0 {
                                if !board.is_enemy(hor as usize, new_ver, self.color) {
                                    is_ok = false;
                                }
                            } else {
                                if board.is_occupied(hor as usize, new_ver) {
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
                }
            }
            PieceType::TOWER => {
                let mut stop: [bool; 8] = [false, false, false, false, false, false, false, false];

                for step in 0..8 {
                    let mut count: usize = 0;
                    for i in 1isize..2 {
                        for j in 1isize..2 {
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

                for step in 0..8 {
                    let mut count: usize = 0;
                    for i in 1isize..2 {
                        for j in 1isize..2 {
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

                for step in 0..8 {
                    let mut count: usize = 0;
                    for i in 1isize..2 {
                        for j in 1isize..2 {
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
                let mut new_board = *board;
                new_board.switch();
                new_board.increment_turn();
                output.push(new_board);
            }
            _ => {}
        }

        output
    }
}

#[derive(Clone, Copy)]
struct BoardState {
    board: [Option<Piece>; 64],
    turn_color: Color,
    turn_number: usize,
}

impl BoardState {
    fn new(turn_color: Color) -> Self {
        Self {
            board: [None; 64],
            turn_color,
            turn_number: 0,
        }
    }

    fn increment_turn(&mut self) {
        self.turn_number += 1;
    }

    fn add_piece(&mut self, hor: usize, ver: usize, piece: Piece) {
        self.board[ver * 8 + hor] = Some(piece);
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

    fn evaluate(&self) -> isize {
        let mut score = 0;

        for square in self.board {
            if let Some(piece) = square {
                match piece.color {
                    Color::WHITE => {
                        score += piece.get_score();
                    }
                    Color::BLACK => {
                        score -= piece.get_score();
                    }
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

        if self.board[from_index].is_none() {
            return Err("No piece at source position");
        }

        let mut piece = self.board[from_index].take();
        self.board[to_index] = piece;
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
        writeln!(f, "Color: {:?}", self.turn_color)?;
        writeln!(f, "Turn: {}", self.turn_number)?;

        Ok(())
    }
}

fn main() {
    let mut board = BoardState::new(Color::WHITE);
    let piece_types = [
        PieceType::TOWER,
        // PieceType::HORSE,
        PieceType::BISHOP,
        PieceType::QUEEN,
        PieceType::KING,
        PieceType::BISHOP,
        // PieceType::HORSE,
        PieceType::TOWER,
    ];

    // White pieces
    for i in 0..1 {
        // board.add_piece(i, 0, Piece::new(piece_types[i], Color::WHITE));
        board.add_piece(i, 1, Piece::new(PieceType::PAWN, Color::WHITE));
    }

    // Black pieces
    for i in 0..1 {
        // board.add_piece(i, 7, Piece::new(piece_types[i], Color::BLACK));
        board.add_piece(i, 6, Piece::new(PieceType::PAWN, Color::BLACK));
    }

    let mut current = board;
    for _ in 0..99 {
        let mut moves = current.generate_moves();
        println!("{}", current);

        if moves.is_empty() {
            println!("No moves left.");
            break;
        }

        let maximizing = current.turn_color == Color::WHITE;
        let depth = match maximizing {
            true => 1,
            false => 1,
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

        current = moves.swap_remove(best_index);
    }
}
