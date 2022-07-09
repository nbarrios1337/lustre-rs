//! Definition of command line arguments

// pub is neeeded for the program to called Arguments::parse()
pub use clap::Parser;

use crate::scenes::SceneType;

/// Toy RT Renderer
#[derive(Parser, Debug)]
pub struct Arguments {
    /// The path to the file to write an image into
    #[clap(short, long, parse(from_os_str))]
    pub output: std::path::PathBuf,

    /// The scene to use
    #[clap(short, long, arg_enum)]
    pub scene: SceneType,

    /// samples per pixel
    #[clap(
        short = 'n',
        long,
        value_parser,
        default_value_t = 100,
        value_name = "NUM"
    )]
    pub samples_per_pixel: u32,
}
