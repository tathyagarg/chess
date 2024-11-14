use crate::consts::convertors;
use core::fmt;
use display::DisplayMode;
pub mod config;
pub mod display;
pub mod state;

pub struct Board {
    pub white_pawns: u64,
    pub white_knights: u64,
    pub white_bishops: u64,
    pub white_rooks: u64,
    pub white_queens: u64,
    pub white_king: u64,
    pub black_pawns: u64,
    pub black_knights: u64,
    pub black_bishops: u64,
    pub black_rooks: u64,
    pub black_queens: u64,
    pub black_king: u64,

    pub display_mode: display::DisplayMode,
    pub state: state::BoardState,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            white_pawns: 65280,                 // 0b11111111 << 8
            white_knights: 66,                  // 0b01000010
            white_bishops: 36,                  // 0b00100100
            white_rooks: 129,                   // 0b10000001
            white_queens: 16,                   // 0b00010000
            white_king: 8,                      // 0b00001000
            black_pawns: 71776119061217280,     // 0b11111111 << 48
            black_knights: 4755801206503243776, // 0b01000010 << 56
            black_bishops: 2594073385365405696, // 0b00100100 << 56
            black_rooks: 9295429630892703744,   // 0b10000001 << 56
            black_queens: 1152921504606846976,  // 0b00010000 << 56
            black_king: 576460752303423488,     // 0b00001000 << 56

            display_mode: config::DISPLAY_MODE,
            state: state::BoardState::new(),
        }
    }

    pub fn from_fen(fen: String) {
        let mut pieces: [u64; 12] = [0; 12];

        let (position, state) = fen.split_once(' ').unwrap();

        let mut spaces = 0;
        for curr in position.chars() {
            if curr.is_ascii_digit() {
                spaces += curr.to_string().parse::<u8>().unwrap();
            } else if curr.is_alphabetic() {
                let index = convertors::piece_to_index(curr);
                for (i, piece_state) in pieces.iter_mut().enumerate() {
                    *piece_state <<= spaces + (i != index) as u8;
                }

                pieces[index] <<= 1;
                pieces[index] |= 1;
                spaces = 0;
            }
        }
        if spaces != 0 {
            for piece_state in pieces.iter_mut() {
                *piece_state <<= spaces;
            }
        }

        let b = Board {
            white_pawns: pieces[0],
            white_knights: pieces[1],
            white_bishops: pieces[2],
            white_rooks: pieces[3],
            white_queens: pieces[4],
            white_king: pieces[5],
            black_pawns: pieces[6],
            black_knights: pieces[7],
            black_bishops: pieces[8],
            black_rooks: pieces[9],
            black_queens: pieces[10],
            black_king: pieces[11],

            display_mode: config::DISPLAY_MODE,
            state: state::BoardState::from_fen(state.to_string()),
        };

        println!("{}", b);
    }

    pub fn pieces(&self) -> [u64; 12] {
        [
            self.white_pawns,
            self.white_knights,
            self.white_bishops,
            self.white_rooks,
            self.white_queens,
            self.white_king,
            self.black_pawns,
            self.black_knights,
            self.black_bishops,
            self.black_rooks,
            self.black_queens,
            self.black_king,
        ]
    }

    fn make_piece_array(&self) -> [char; 64] {
        let mut pieces = self.pieces();
        let mut base = [' '; 64];
        for (i, bitboard) in pieces.iter_mut().enumerate() {
            let mut j = 63;
            while *bitboard > 0 {
                if *bitboard & 1 == 1 {
                    base[j] = convertors::index_to_piece(i);
                }
                *bitboard >>= 1;
                j = j.saturating_sub(1);
            }
        }

        base
    }

    pub fn display(&self) -> String {
        format!(
            "{}\n{}",
            (match self.display_mode {
                DisplayMode::Basic => display::basic_display,
                DisplayMode::BasicGrid => display::basic_grid_display,
                DisplayMode::BasicBox => display::basic_box_display,
                DisplayMode::ColoredBox => display::colored_box_display,
            })(&self.make_piece_array()),
            self.state.display()
        )
    }

    pub fn to_fen(&self) -> String {
        let pieces = self.make_piece_array();
        let mut rows: [[char; 8]; 8] = [[' '; 8]; 8];
        for (i, piece) in pieces.iter().enumerate() {
            rows[i / 8][i % 8] = *piece
        }
        let mut res: Vec<String> = Vec::new();
        let mut curr = String::new();

        for row in rows {
            let mut spaces = 0;
            for piece in row {
                if piece != ' ' && spaces == 0 {
                    curr.push(piece);
                } else if piece != ' ' {
                    curr.push(spaces.to_string().chars().next().unwrap());
                    curr.push(piece);

                    spaces = 0;
                } else {
                    spaces += 1
                }
            }
            if spaces != 0 {
                curr.push(spaces.to_string().chars().next().unwrap());
            }

            res.push(curr);
            curr = String::new();
        }

        let position_fen = res.join("/");
        let state_fen = self.state.to_fen();

        format!("{} {}", position_fen, state_fen)
    }
}

pub fn popcount(bitboard: &mut u64) -> u8 {
    let mut count: u8 = 0;
    let mut bb = *bitboard;
    while bb != 0 {
        count += 1;
        bb &= bb - 1;
    }

    count
}
