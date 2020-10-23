use crate::piece::Piece;
use crate::Position;

struct SquareBoard8x8 {
    /// squares are stored in an array of length 64.
    /// They are row-centric, meaning the array can be partitioned by 8 receiving the rows 1 to 8.
    /// Ordering is a1=0, b1=1, c1=2, ..., h1=7, a2=8, b2=9, ..., g8=62, h8=63
    squares: [Option<Piece>;64]
}

impl SquareBoard8x8 {
    /// Initializes the standard 8x8 squares chess board
    pub const fn new() -> Self {
        Self {
            squares: [
                Some(Piece::WhiteRook), Some(Piece::WhiteKnight), Some(Piece::WhiteBishop), Some(Piece::WhiteQueen), Some(Piece::WhiteKing), Some(Piece::WhiteBishop), Some(Piece::WhiteKnight), Some(Piece::WhiteRook),
                Some(Piece::WhitePawn), Some(Piece::WhitePawn), Some(Piece::WhitePawn), Some(Piece::WhitePawn), Some(Piece::WhitePawn), Some(Piece::WhitePawn), Some(Piece::WhitePawn), Some(Piece::WhitePawn),
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                Some(Piece::BlackPawn), Some(Piece::BlackPawn), Some(Piece::BlackPawn), Some(Piece::BlackPawn), Some(Piece::BlackPawn), Some(Piece::BlackPawn), Some(Piece::BlackPawn), Some(Piece::BlackPawn),
                Some(Piece::BlackRook), Some(Piece::BlackKnight), Some(Piece::BlackBishop), Some(Piece::BlackQueen), Some(Piece::BlackKing), Some(Piece::BlackBishop), Some(Piece::BlackKnight), Some(Piece::BlackRook)
            ]
        }
    }

    fn position_to_index(position: &Position) -> usize {
        position.row as usize * 8 + position.col as usize
    }

    fn square(&self, position: impl Into<Position>) -> &Option<Piece> {
        &self.squares[Self::position_to_index(&position.into())]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn query_initial_squares() {
        // Arrange
        let board = SquareBoard8x8::new();

        // Assert
        assert_eq!(&Some(Piece::WhiteKnight), board.square(Position::parse("b1").unwrap()));
        assert_eq!(&Some(Piece::BlackKing), board.square(Position::parse("E8").unwrap()));
        assert_eq!(&Some(Piece::WhiteQueen), board.square(Position::parse("d1").unwrap()));
        assert_eq!(&Some(Piece::WhitePawn), board.square(Position::parse("G2").unwrap()));
        assert_eq!(&None, board.square(Position::parse("c4").unwrap()));
    }
}