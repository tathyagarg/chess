pub mod files {
    pub const A: u8 = 1;
    pub const B: u8 = 2;
    pub const C: u8 = 3;
    pub const D: u8 = 4;
    pub const E: u8 = 5;
    pub const F: u8 = 6;
    pub const G: u8 = 7;
    pub const H: u8 = 8;

    pub fn int_to_file(file: u8) -> char {
        (file + 64) as char
    }

    pub fn file_to_int(file: char) -> u8 {
        (file as u8) - 64
    }
}

pub mod convertors {
    // According to Forsyth-Edwards Notation (FEN):
    // "White pieces are designated using uppercase letters ("PNBRQK"), while black pieces use lowercase letters ("pnbrqk")."
    // https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation
    const PIECES: [char; 12] = ['P', 'N', 'B', 'R', 'Q', 'K', 'p', 'n', 'b', 'r', 'q', 'k'];

    pub fn index_to_piece(idx: usize) -> char {
        PIECES[idx]
    }

    pub fn piece_to_index(piece: char) -> usize {
        if let Some(i) = PIECES.iter().position(|&r| r == piece) {
            i
        } else {
            panic!("Invalid piece")
        }
    }
}
