use anyhow::Result;
use crate::err::PositionParseError;

#[derive(Debug, PartialEq)]
pub struct Position {
    pub row: u8,
    pub col: u8
}

impl Position {
    const VALID_ROWS: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];
    const VALID_COLS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

    pub fn parse(position: &str) -> Result<Self> {
        let mut chars = position.chars();
        let col = Self::parse_col(&chars.next().expect("Not enough input for column!"))?;
        let row = Self::parse_row(&chars.next().expect("Not enough input for row!"))?;

        Ok(Self{row, col})
    }

    fn parse_row(char: &char) -> Result<u8, PositionParseError> {
        if !Position::VALID_ROWS.contains(char) {Err(PositionParseError::InvalidRow(char.clone()))}
        else {Ok(char.to_digit(10).unwrap() as u8 - 1)}
    }

    fn parse_col(char: &char) -> Result<u8, PositionParseError> {
        let lower = char.to_lowercase().next().unwrap();
        if !Position::VALID_COLS.contains(&lower) {Err(PositionParseError::InvalidCol(char.clone()))}
        else {Ok(Position::VALID_COLS.iter().position(|c| *c == lower).unwrap() as u8)}
    }

    pub fn to_string(&self) -> String {
        String::from(format!("{}{}", Self::VALID_COLS[self.col as usize], Self::VALID_ROWS[self.row as usize]))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_position() {
        // Arrange
        let expected = Position{row: 4, col: 1};

        // Act
        let actual = Position::parse("b5");
        assert!(actual.is_ok());
        let actual = actual.unwrap();

        // Assert
        assert_eq!(expected, actual);
        assert_eq!("b5".to_string(), actual.to_string());
    }
}