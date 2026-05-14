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

#[derive(Debug, Clone, Copy)]
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

fn check_break(
    board: &BoardState,
    hor: usize,
    ver: usize,
    color: Color,
) -> (bool, bool) {
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
}

#[inline(always)]
fn valid_pos(hor: isize, ver: isize) -> bool {
    (0..8).contains(&hor) && (0..8).contains(&ver)
}

fn process_move(board: &BoardState, output: &mut Vec<BoardState>, current_hor: usize, current_ver: usize, hor: isize, ver: isize, color: Color) -> bool {
    if valid_pos(hor, ver) {
        let hor = hor as usize;
        let ver = ver as usize;

        let mut new_board = *board;
        new_board.switch();
        let (stop_now, stop_next) = check_break(&new_board, hor, ver, color);

        if !stop_now && new_board.move_piece(current_hor, current_ver, hor, ver).is_ok() {
            output.push(new_board);
        }
        
        return stop_next;
    }
    false
}
               
impl Piece {
    fn new(piece_type: PieceType, color: Color) -> Self {
        Self { piece_type, color }
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
        board: &BoardState,
        current_hor: usize,
        current_ver: usize,
    ) -> Vec<BoardState> {
        let mut output = Vec::new();

        match self.piece_type {
            PieceType::PAWN => {
                let new_ver = match self.color {
                    Color::WHITE => current_ver + 1,
                    Color::BLACK => {
                        if current_ver == 0 {
                            return output;
                        }
                        current_ver - 1
                    }
                };

                if new_ver < 8 {
                    // forward
                    let mut new_board = *board;
                    if !new_board.is_occupied(current_hor, new_ver)
                        && new_board
                            .move_piece(current_hor, current_ver, current_hor, new_ver)
                            .is_ok()
                    {
                        new_board.switch();
                        output.push(new_board);
                    }

                    // capture left
                    if current_hor > 0 {
                        let mut new_board = *board;
                        if new_board.is_enemy(current_hor - 1, new_ver, self.color)
                            && new_board
                                .move_piece(current_hor, current_ver, current_hor - 1, new_ver)
                                .is_ok()
                        {
                            new_board.switch();
                            output.push(new_board);
                        }
                    }

                    // capture right
                    if current_hor < 7 {
                        let mut new_board = *board;
                        if new_board.is_enemy(current_hor + 1, new_ver, self.color)
                            && new_board
                                .move_piece(current_hor, current_ver, current_hor + 1, new_ver)
                                .is_ok()
                        {
                            new_board.switch();
                            output.push(new_board);
                        }
                    }
                }
            }
            PieceType::TOWER => {
                let mut stop: bool = false;
                for i in 1..8 - current_ver {
                    let mut new_board = *board;
                    new_board.switch();
                    let new_ver = current_ver + i;
                    if new_board.is_occupied(current_hor, new_ver) {
                        if !new_board.is_enemy(current_hor, new_ver, self.color) {
                            break;
                        } else {
                            stop = true;
                        }
                    }
                    if new_board
                        .move_piece(current_hor, current_ver, current_hor, new_ver)
                        .is_ok()
                    {
                        output.push(new_board);
                    }
                    if stop {
                        break;
                    }
                }
                stop = false;
                for i in 0..current_ver {
                    let mut new_board = *board;
                    new_board.switch();
                    let new_ver = current_ver - i;
                    if new_board.is_occupied(current_hor, new_ver) {
                        if !new_board.is_enemy(current_hor, new_ver, self.color) {
                            break;
                        } else {
                            stop = true;
                        }
                    }
                    if new_board
                        .move_piece(current_hor, current_ver, current_hor, new_ver)
                        .is_ok()
                    {
                        output.push(new_board);
                    }
                    if stop {
                        break;
                    }
                }
                stop = false;
                for i in 1..8 - current_hor {
                    let mut new_board = *board;
                    new_board.switch();
                    let new_hor = current_hor + i;
                    if new_board.is_occupied(new_hor, current_ver) {
                        if !new_board.is_enemy(new_hor, current_ver, self.color) {
                            break;
                        } else {
                            stop = true;
                        }
                    }
                    if new_board
                        .move_piece(current_hor, current_ver, new_hor, current_ver)
                        .is_ok()
                    {
                        output.push(new_board);
                    }
                    if stop {
                        break;
                    }
                }
                stop = false;
                for i in 0..current_hor {
                    let mut new_board = *board;
                    new_board.switch();
                    let new_hor = current_hor - i;
                    if new_board.is_occupied(new_hor, current_ver) {
                        if !new_board.is_enemy(new_hor, current_ver, self.color) {
                            break;
                        } else {
                            stop = true;
                        }
                    }
                    if new_board
                        .move_piece(current_hor, current_ver, new_hor, current_ver)
                        .is_ok()
                    {
                        output.push(new_board);
                    }
                    if stop {
                        break;
                    }
                }
            }
            PieceType::BISHOP => {
                let mut stop_lu: bool = false;
                let mut stop_ld: bool = false;
                let mut stop_ru: bool = false;
                let mut stop_rd: bool = false;

                for i in 1..8 {
                    let ver_u = current_ver as isize + i as isize;
                    let hor_l = current_hor as isize - i as isize;
                    let ver_d = current_ver as isize - i as isize;
                    let hor_r = current_hor as isize + i as isize;

                    if !stop_lu {
                        stop_lu = process_move(board, &mut output, current_hor, current_ver, hor_l, ver_u, self.color);
                    }

                    if !stop_ru {
                        stop_ru = process_move(board, &mut output, current_hor, current_ver, hor_r, ver_u, self.color);
                    }

                    if !stop_ld {
                        stop_ld = process_move(board, &mut output, current_hor, current_ver, hor_l, ver_d, self.color);
                    }

                    if !stop_rd {
                        stop_rd = process_move(board, &mut output, current_hor, current_ver, hor_r, ver_d, self.color);
                    }
                }
            }
            PieceType::HORSE => {
                let mut new_board = *board;
                new_board.switch();
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
}

impl BoardState {
    fn new(turn_color: Color) -> Self {
        Self {
            board: [None; 64],
            turn_color,
        }
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

    fn minimax(&self, depth: usize, maximizing: bool) -> isize {
        if depth == 0 {
            return self.evaluate();
        }

        let moves = self.generate_moves();

        if moves.is_empty() {
            return self.evaluate();
        }

        if maximizing {
            let mut best = isize::MIN;

            for mv in moves {
                let score = mv.minimax(depth - 1, false);

                if score > best {
                    best = score;
                }
            }

            best
        } else {
            let mut best = isize::MAX;

            for mv in moves {
                let score = mv.minimax(depth - 1, true);

                if score < best {
                    best = score;
                }
            }

            best
        }
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

        let piece = self.board[from_index].take();
        self.board[to_index] = piece;

        Ok(())
    }

    fn switch(&mut self) {
        self.turn_color = match self.turn_color {
            Color::WHITE => Color::BLACK,
            Color::BLACK => Color::WHITE,
        };
    }

    fn generate_moves(&self) -> Vec<BoardState> {
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

    fn get_color(&self, hor: usize, ver: usize) -> Option<Color> {
        let piece = self.board[ver * 8 + hor];
        match piece {
            Some(piece) => {
                return Some(piece.get_color());
            }
            None => {
                return None;
            }
        }
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

        Ok(())
    }
}

fn main() {
    let mut board = BoardState::new(Color::WHITE);
    let piece_types = [
        PieceType::TOWER,
        // PieceType::HORSE,
        PieceType::BISHOP,
        // PieceType::QUEEN,
        // PieceType::KING,
        PieceType::BISHOP,
        // PieceType::HORSE,
        PieceType::TOWER,
    ];

    // White pieces
    for i in 0..4 {
        board.add_piece(i, 0, Piece::new(piece_types[i], Color::WHITE));

        // board.add_piece(
        //     i,
        //     1,
        //     Piece::new(PieceType::PAWN, Color::WHITE),
        // );
    }

    // Black pieces
    for i in 0..4 {
        board.add_piece(i, 7, Piece::new(piece_types[i], Color::BLACK));

        // board.add_piece(
        //     i,
        //     6,
        //     Piece::new(PieceType::PAWN, Color::BLACK),
        // );
    }

    let mut current = board;

    for _ in 0..9 {
        let mut moves = current.generate_moves();
        println!("{}", current);

        if moves.is_empty() {
            println!("No moves left.");
            break;
        }

        let maximizing = current.turn_color == Color::WHITE;

        let best_index = moves
            .iter()
            .enumerate()
            .max_by_key(|(_, board)| board.minimax(1, !maximizing))
            .map(|(index, _)| index)
            .unwrap();

        current = moves.swap_remove(best_index);
    }
}
