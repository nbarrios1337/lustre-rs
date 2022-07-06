use indicatif::{ProgressBar, ProgressStyle};

pub fn get_progressbar(len: u64) -> ProgressBar {
    ProgressBar::new((len) as u64).with_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {prefix} {wide_bar} {pos:>7}/{len:7} ({percent}%)"),
    )
}

pub fn set_progressbar_msg(msg: &'static str, bar: ProgressBar) -> ProgressBar {
    bar.with_prefix(msg)
}
