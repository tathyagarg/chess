use core::fmt;

use crate::consts::{convertors, files};

pub mod castling {
    pub const WHITE_KSIDE: u8 = 0b1000;
    pub const WHITE_QSIDE: u8 = 0b0100;
    pub const BLACK_KSIDE: u8 = 0b0010;
    pub const BLACK_QSIDE: u8 = 0b0001;
}

pub struct EnPassantSquare {
    pub file: u8,
    pub rank: u8,
}

pub enum EnPassantState {
    Cannot,
    Can(EnPassantSquare),
}

impl fmt::Display for EnPassantState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match self {
            EnPassantState::Cannot => String::from("-"),
            EnPassantState::Can(square) => {
                format!("{}{}", files::int_to_file(square.file), square.rank)
            }
        };

        write!(f, "{}", printable)
    }
}

pub struct BoardState {
    pub side_to_move: char,
    pub castling_ability: u8,
    pub en_passant: EnPassantState,
    pub half_move: u8,
    pub full_move: u16,
}

impl Default for BoardState {
    fn default() -> Self {
        Self::new()
    }
}

impl BoardState {
    pub fn new() -> BoardState {
        BoardState {
            side_to_move: 'w',
            castling_ability: castling::WHITE_KSIDE
                | castling::WHITE_QSIDE
                | castling::BLACK_KSIDE
                | castling::BLACK_QSIDE,
            en_passant: EnPassantState::Cannot,
            half_move: 0,
            full_move: 1,
        }
    }

    pub fn from_fen(fen: String) -> BoardState {
        let mut iter = fen.split(' ');
        let side_to_move = iter.next().unwrap().chars().next().unwrap();
        let castling_ability_raw = iter.next().unwrap();
        let en_passant_raw = iter.next().unwrap();
        let half_move_raw = iter.next().unwrap();
        let full_move_raw = iter.next().unwrap();

        let mut castling_ability = 0;
        println!("ca: {castling_ability_raw}");
        if castling_ability_raw.contains('K') {
            castling_ability |= castling::WHITE_KSIDE;
        }
        if castling_ability_raw.contains('Q') {
            castling_ability |= castling::WHITE_QSIDE;
        }
        if castling_ability_raw.contains('k') {
            castling_ability |= castling::BLACK_KSIDE;
        }
        if castling_ability_raw.contains('q') {
            castling_ability |= castling::BLACK_QSIDE;
        }

        let en_passant = if en_passant_raw == "-" {
            EnPassantState::Cannot
        } else {
            let (file, rank) = {
                let mut chars = en_passant_raw.chars();
                (chars.next().unwrap(), chars.next().unwrap())
            };
            EnPassantState::Can(EnPassantSquare {
                file: files::file_to_int(file),
                rank: rank.to_string().parse::<u8>().unwrap(),
            })
        };

        BoardState {
            side_to_move,
            castling_ability,
            en_passant,
            half_move: half_move_raw.to_string().parse::<u8>().unwrap(),
            full_move: full_move_raw.to_string().parse::<u16>().unwrap(),
        }
    }

    pub fn to_fen(&self) -> String {
        let mut castling_ability = String::new();
        if self.castling_ability & castling::WHITE_KSIDE == castling::WHITE_KSIDE {
            castling_ability.push('K');
        }
        if self.castling_ability & castling::WHITE_QSIDE == castling::WHITE_QSIDE {
            castling_ability.push('Q');
        }
        if self.castling_ability & castling::BLACK_KSIDE == castling::BLACK_KSIDE {
            castling_ability.push('k');
        }
        if self.castling_ability & castling::BLACK_QSIDE == castling::BLACK_QSIDE {
            castling_ability.push('q');
        }

        let en_passant = match &self.en_passant {
            EnPassantState::Cannot => String::from("-"),
            EnPassantState::Can(square) => {
                format!("{}{}", files::int_to_file(square.file), square.rank)
            }
        };

        format!(
            "{} {} {} {} {}",
            self.side_to_move, castling_ability, en_passant, self.half_move, self.full_move
        )
    }

    pub fn display(&self) -> String {
        let mut castling_ability = String::new();
        if self.castling_ability & castling::WHITE_KSIDE == castling::WHITE_KSIDE {
            castling_ability.push('K');
        }
        if self.castling_ability & castling::WHITE_QSIDE == castling::WHITE_QSIDE {
            castling_ability.push('Q');
        }
        if self.castling_ability & castling::BLACK_KSIDE == castling::BLACK_KSIDE {
            castling_ability.push('k');
        }
        if self.castling_ability & castling::BLACK_QSIDE == castling::BLACK_QSIDE {
            castling_ability.push('q');
        }

        format!("Side to Move: {}\nCastling Ability: {}\nEn Passant: {}\nHalf Move Counter: {}\nFull Move Counter: {}", self.side_to_move, castling_ability, self.en_passant, self.half_move, self.full_move)
    }
}
