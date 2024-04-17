use clap::{command, Parser};
use enmacompat::{
    area::{find_area_array_location, read_area, EnmaAreaRaw},
    windows_pe::WindowsPEFile,
};
use std::fs::File;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[clap(short, long)]
    path: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();
    let file = File::open(&args.path).expect("Failed to open file");
    let mut windows_pe_file = WindowsPEFile::from(file);

    println!("\x1b[34m\x1b[1mSearching for Area Array Location\x1b[0m");
    let location = Some(0x1417cc980); //find_area_array_location(&mut windows_pe_file).unwrap();
    if let Some(location) = location {
        println!(
            "\x1b[32m\x1b[1mArea Array Location\x1b[0m: \x1b[4m{:#x}\x1b[0m (Memory), \x1b[4m{:#x}\x1b[0m (File)",
            location,
            windows_pe_file.get_file_address(location).unwrap()
        );

        for i in 1..2 {
            let address = windows_pe_file.read_addr::<u64>(location + i * 8).unwrap();
            let area = read_area(&windows_pe_file, address).unwrap();

            let file = File::create(format!("{}.json", area.name)).unwrap();
            let mut writer = std::io::BufWriter::new(file);
            serde_json::to_writer_pretty(&mut writer, &area).unwrap();
        }
    } else {
        println!("\x1b[31mArea array location could not found\x1b[0m");
    }
}
