use clap::{command, Parser, Subcommand};
use enmacompat::{area::read_area, meta::EnmaMeta, windows_pe::WindowsPEFile};
use std::fs::File;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[clap(subcommand)]
    command: Commands,

    #[clap(short, long)]
    path: std::path::PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "Analyze the file and generate a metadata file")]
    Analyze,
    #[clap(about = "Extract data from the file")]
    Extract,
}

fn main() {
    let args = Args::parse();
    let file = File::open(&args.path).expect("Failed to open file");
    let mut windows_pe_file = WindowsPEFile::from(file);

    match args.command {
        Commands::Analyze => {
            let file = File::create(args.path.with_extension("json")).unwrap();
            let mut writer = std::io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &EnmaMeta::from_analysis(&mut windows_pe_file).unwrap(),
            )
            .unwrap();
        }
        Commands::Extract => {
            let meta_file = File::open(args.path.with_extension("json"))
                .expect("Missing meta file, did you forget to run the analyze command?");
            let meta: EnmaMeta = serde_json::from_reader(meta_file)
                .expect("Failed to read meta file, try running the analyze command again?");

            for i in 1..meta.area_array_count {
                let address = windows_pe_file
                    .read_addr::<u64>(meta.area_array_addr + i as u64 * 8)
                    .unwrap();

                let area = read_area(&windows_pe_file, address).unwrap();
                let file = File::create(format!("out/{}.json", area.name)).unwrap();
                let mut writer = std::io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &area).unwrap();
            }
        }
    }
}
