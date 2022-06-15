pub use clap::Parser;

#[derive(Parser)]
pub struct Arguments {
    /// The path to the file to write an image into
    #[clap(short, long, parse(from_os_str))]
    pub output: std::path::PathBuf,
}
