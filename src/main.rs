pub mod board;
pub mod consts;
pub mod moves;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value_t = String::from("abc"))]
    from: String,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);

    board::Board::from_fen(args.from);

    //   let b = board::Board::new();

    // println!("{}", b);
    //    println!("{}", b.to_fen());
}
