//! Definition of command line arguments

// pub is neeeded for the program to called Arguments::parse()
pub use clap::Parser;

use crate::scenes::SceneType;

/// Argument defintions for clap-derive
#[derive(Parser, Debug)]
#[clap(about = "Toy RT Renderer")]
pub struct Arguments {
    /// The path to the file to write the resulting image into
    #[clap(
        short,
        long,
        value_parser,
        default_value = "output.png",
        value_name = "FILE"
    )]
    pub output: std::path::PathBuf,

    /// The hardcoded scene to use
    #[clap(short, long, value_enum, default_value_t = SceneType::CoverPhoto)]
    pub scene: SceneType,

    /// samples per pixel
    ///
    /// A higher count of samples leads to higher visual fidelity
    #[clap(
        short = 'n',
        long = "samples",
        value_parser,
        default_value_t = 100,
        value_name = "NUM"
    )]
    pub samples_per_pixel: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_cli() {
        use clap::CommandFactory;
        Arguments::command().debug_assert()
    }
}
