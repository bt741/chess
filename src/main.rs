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


#[derive(Debug)]
struct Position {
    hor: u8,
    ver: u8,
}

impl Position {
    fn new(hor: u8, ver: u8) -> Self {
        Self { hor, ver }
    }
}

#[derive(Debug)]
struct Piece {
    position: Position,
    // next_moves: Vec<Position>,
    piece_type: PieceType,
}

impl Piece {
    fn new(hor: u8, ver: u8, piece_type: PieceType) -> Self {
        let position = Position::new(hor, ver);
        Self {
            position,
            // next_moves: Position::get_next_moves(postition, type),
            piece_type,
        }
    }
    
    fn symbol(&self) -> char {
        match self.piece_type {
            PieceType::PAWN => 'P',
            PieceType::TOWER => 'T',
            PieceType::HORSE => 'H',
            PieceType::BISHOP => 'B',
            PieceType::QUEEN => 'Q',
            PieceType::KING => 'K',
        }
    }
}

struct BoardState {
    pieces: Vec<Piece>,
}

impl BoardState {
    fn new(pieces: Vec<Piece>) -> Self {
        Self {
            pieces
        }
    }
}

impl fmt::Debug for BoardState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board = [['.'; 8]; 8];

        for piece in &self.pieces {
            let x = piece.position.hor as usize;
            let y = piece.position.ver as usize;

            board[y][x] = piece.symbol();
        }

        writeln!(f, "    0 1 2 3 4 5 6 7")?;

        for y in (0..8).rev() {
            write!(f, "{} | ", y)?;

            for x in 0..8 {
                write!(f, "{} ", board[y][x])?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

fn main() {
    let mut pieces: Vec<Piece> = Vec::new();
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
        
    for i in 0..8 {
        let piece_type = &piece_types[i as usize];
        pieces.push(Piece::new(i, 0, piece_type.clone()));
        pieces.push(Piece::new(i, 1, PieceType::PAWN));
        pieces.push(Piece::new(i, 6, PieceType::PAWN));
        pieces.push(Piece::new(i, 7, piece_type.clone()));
    }

    let board = BoardState::new(pieces);
    println!("{:?}", board);
}
