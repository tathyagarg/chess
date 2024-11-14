pub enum DisplayMode {
    Basic,
    BasicGrid,
    BasicBox,
    ColoredBox,
}

const BG_BLACK: &str = "\x1b[40m";
const BG_WHITE: &str = "\x1b[47m";
const FG_BLACK: &str = "\x1b[30m";
const FG_WHITE: &str = "\x1b[37m";
const RESET: &str = "\x1b[0m";

pub fn basic_display(pieces: &[char; 64]) -> String {
    let mut result = String::new();

    for (i, curr) in pieces.iter().enumerate() {
        if i % 8 == 0 {
            result.push_str("\n|");
        }
        result.push(*curr);
        result.push('|');
    }

    let mut chars = result.chars();
    chars.next();
    chars.as_str().to_string()
}

pub fn basic_grid_display(pieces: &[char; 64]) -> String {
    let basic = basic_display(pieces);
    let rows = basic.split('\n');

    let mut res = String::new();

    for (i, row) in rows.into_iter().enumerate() {
        res.push_str(format!("{} {}\n", 8 - i, row).as_str());
    }
    res.push_str("   A B C D E F G H\n");

    res
}

pub fn basic_box_display(pieces: &[char; 64]) -> String {
    let mut res = String::new();
    let mut base = String::new();

    for (i, curr) in pieces.iter().enumerate() {
        if i % 8 == 0 {
            base.push_str("\n ");
        }
        base.push(*curr);
        base.push(' ');
    }

    let mut chars = base.chars();
    chars.next();
    let base = chars.as_str().to_string();

    res.push_str("  +-----------------+\n");
    for (i, row) in base.split('\n').enumerate() {
        res.push_str(format!("{} |{}|\n", 8 - i, row).as_str());
    }
    res.push_str("  +-----------------+\n");
    res.push_str("    A B C D E F G H  ");

    res
}

pub fn colored_box_display(pieces: &[char; 64]) -> String {
    let mut res = String::new();
    let mut base = String::new();

    for (i, curr) in pieces.iter().enumerate() {
        if i % 8 == 0 {
            base.push('\n');
        }
        if i % 2 != i / 8 % 2 {
            base.push_str(BG_BLACK);
            base.push_str(FG_WHITE);
            if *curr == ' ' {
                base.push_str("   ");
            } else {
                base.push(' ');
                base.push(*curr);
                base.push(' ');
            }
            base.push_str(RESET);
        } else {
            base.push_str(BG_WHITE);
            base.push_str(FG_BLACK);
            if *curr == ' ' {
                base.push_str("   ");
            } else {
                base.push(' ');
                base.push(*curr);
                base.push(' ');
            }
            base.push_str(RESET);
        }
    }

    let mut chars = base.chars();
    chars.next();
    let base = chars.as_str().to_string();

    res.push_str("  +------------------------+\n");
    for (i, row) in base.split('\n').enumerate() {
        res.push_str(format!("{} |{}|\n", 8 - i, row).as_str());
    }
    res.push_str("  +------------------------+\n");
    res.push_str("    A  B  C  D  E  F  G  H  ");

    res
}
