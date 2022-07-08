//! Progress reporting utilities
//!
//! Relies on the [indicatif] crate

use indicatif::{ProgressBar, ProgressStyle};

/// Returns a new [ProgressBar] with a custom template string
///
/// The template is as follows:
///
/// `"[{elapsed_precise}] {prefix} {wide_bar} {pos:>7}/{len:7} ({percent}%)"`.
///
/// See [ProgressStyle] for more information.
pub fn get_progressbar(len: u64) -> ProgressBar {
    ProgressBar::new((len) as u64).with_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {prefix} {wide_bar} {pos:>7}/{len:7} ({percent}%)"),
    )
}

/// Wrapper for [ProgressBar]'s with_prefix function
pub fn set_progressbar_msg(msg: &'static str, bar: ProgressBar) -> ProgressBar {
    bar.with_prefix(msg)
}
