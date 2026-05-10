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

#[derive(Debug, Clone)]
enum PieceType {
    PAWN,
    TOWER,
    HORSE,
    BISHOP,
    QUEEN,
    KING,
}

#[derive(Debug, Clone, PartialEq)]
enum Color {
    WHITE,
    BLACK
}

#[derive(Clone)]
struct Piece {
    piece_type: PieceType,
    color: Color,
}

impl Piece {
    fn new(
        piece_type: PieceType,
        color: Color
    ) -> Self {
        Self {
            piece_type,
            color,
        }
    }
    
    fn get_symbol(
        &self
    ) -> char {
        match self.piece_type {
            PieceType::PAWN => 'P',
            PieceType::TOWER => 'T',
            PieceType::HORSE => 'H',
            PieceType::BISHOP => 'B',
            PieceType::QUEEN => 'Q',
            PieceType::KING => 'K',
        }
    }

    fn generate_moves(
        &self,
        board: &BoardState,
        current_hor: usize,
        current_ver: usize,
    ) -> Vec<BoardState> {
        let mut output: Vec<BoardState> = Vec::new();
        
        let mut new_ver: usize = current_ver + 1;
        if self.color == Color::BLACK {
            new_ver = current_ver - 1;
        }

        match self.piece_type {
            PieceType::PAWN => {
                let mut new_board = board.clone();
                new_board.switch();
                let _ = new_board.move_piece(
                    current_hor,
                    current_ver, 
                    current_hor,
                    new_ver
                );
                output.push(new_board);
            },
            _ => {},
        }

        return output;
    }


    fn get_color(
        &self,
    ) -> Color {
        self.color.clone()
    }
}

#[derive(Clone)]
struct BoardState {
    board: [Option<Piece>; 64],
    next: Vec<BoardState>,
    turn_color: Color,
    bot_color: Color,
}

impl BoardState {
    fn new(
        turn_color: Color,
        bot_color: Color
    ) -> Self {
        Self {
            board: [const { None }; 64],
            next: Vec::new(),
            turn_color,
            bot_color,
        }
    }

    fn add_piece(
        &mut self,
        hor: usize,
        ver: usize,
        piece: Piece
    ) -> () {
        self.board[ver * 8 + hor] = Some(piece);
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

    fn pick_move(
        &mut self,
    ) -> Option<BoardState> {
        self.next.pop()
    }

    fn switch(
        &mut self,
    ) {
        self.turn_color = match self.turn_color {
            Color::WHITE => Color::BLACK,
            Color::BLACK => Color::WHITE,
        };
    }

    fn generate_moves(&mut self) {
        for ver in 0..8 {
            for hor in 0..8 {
                if let Some(piece) = &self.board[ver * 8 + hor] {
                    if piece.get_color() == self.turn_color {
                        let moves = piece.generate_moves(&self, hor, ver);
                        self.next.extend(moves);
                    }
                }
            }
        }
    }
}

impl fmt::Display for BoardState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board = [['.'; 8]; 8];
        writeln!(f, "    0 1 2 3 4 5 6 7")?;

        for ver in (0..8).rev() {
            write!(f, "{} | ", ver)?;

            for hor in 0..8 {
                let mut symbol = match &self.board[ver * 8 + hor] {
                    Some(piece) => piece.get_symbol(),
                    None => ' ',
                };
                write!(f, "{} ", symbol)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

fn main() {
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
        
    let mut board = BoardState::new(Color::BLACK, Color::WHITE);
    for i in 0..8 {
        let piece_type = &piece_types[i as usize];
        board.add_piece(i, 0, Piece::new(piece_type.clone(), Color::WHITE));
        board.add_piece(i, 1, Piece::new(PieceType::PAWN, Color::WHITE));
        board.add_piece(i, 6, Piece::new(PieceType::PAWN, Color::BLACK));
        board.add_piece(i, 7, Piece::new(piece_type.clone(), Color::BLACK));
    }

  
    let mut chessmove: Option<BoardState> = Some(board);
    for i in 0..5 {
        match chessmove {
            Some(mut current_move) => {
                println!("{}", current_move);
                current_move.generate_moves();
                chessmove = current_move.pick_move();
            },
            None => {},
        }
    }
}
