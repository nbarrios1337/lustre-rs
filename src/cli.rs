//! Definition of command line arguments

// pub is neeeded for the program to called Arguments::parse()
pub use clap::Parser;

/// Toy RT Renderer
#[derive(Parser)]
pub struct Arguments {
    /// The path to the file to write an image into
    #[clap(short, long, parse(from_os_str))]
    pub output: std::path::PathBuf,
}
