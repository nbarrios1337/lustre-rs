//! Progress reporting utilities
//!
//! Relies on the [indicatif] crate

use indicatif::{ProgressBar, ProgressFinish, ProgressStyle};

/// Returns a new [ProgressBar] with a custom template string
///
/// The template is as follows:
///
/// `"[{elapsed_precise}] {prefix} {wide_bar} {pos:>7}/{len:7} ({percent}%)"`.
///
/// See [ProgressStyle] for more information.
pub fn get_progressbar(len: u64) -> ProgressBar {
    ProgressBar::new((len) as u64).with_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {spinner} {prefix} {human_pos:>7}/{human_len:7} ({percent}%) {msg}",
        )
        .unwrap(),
    ).with_finish(ProgressFinish::WithMessage(std::borrow::Cow::Owned("Done!".to_string())))
}
