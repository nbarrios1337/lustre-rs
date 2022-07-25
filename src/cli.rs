//! Definition of command line arguments

// pub is neeeded for the program to called Arguments::parse()
pub use clap::Parser;

use crate::scenes::SceneType;

/// Argument defintions for clap-derive
#[derive(Parser, Debug)]
#[clap(version, about, global_setting(clap::AppSettings::DeriveDisplayOrder))]
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

    /// samples per pixel
    ///
    /// A higher count of samples leads to higher visual fidelity due to more rays sent for a pixel
    #[clap(
        short = 'n',
        long = "samples",
        value_parser = valid_count,
        default_value_t = 100,
        value_name = "NUM"
    )]
    pub samples_per_pixel: u32,

    /// number of light contribution bounces
    ///
    /// A higher number of bounces leads to higher visual fidelity due to more accurate gathered light
    #[clap(
        short,
        long = "bounces",
        value_parser = valid_count,
        default_value_t = 50,
        value_name = "NUM"
    )]
    pub bounce_depth: u16,

    /// The hardcoded scene to use
    #[clap(short, long, value_enum, default_value_t = SceneType::CoverPhoto)]
    pub scene: SceneType,
}

fn valid_count(s: &str) -> Result<u32, String> {
    match s.parse() {
        Ok(count) => {
            if count > 0 {
                Ok(count)
            } else {
                Err("count must be greater than 0".to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
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
