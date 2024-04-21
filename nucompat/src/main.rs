use binrw::BinRead;
use clap::{command, Parser};
use nucompat::NuFile;
use std::fs::File;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[clap(short, long)]
    path: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();
    let file = File::open(&args.path).expect("Failed to open file");
    let mut reader = std::io::BufReader::new(file);

    let file = NuFile::read(&mut reader).unwrap();
    println!("{:#?}", file);
}
