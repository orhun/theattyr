pub use clap::Parser;

/// Argument parser powered by [`clap`].
#[derive(Clone, Debug, Default, Parser)]
#[clap(
    version,
    author = clap::crate_authors!("\n"),
    about,
    rename_all_env = "screaming-snake",
    help_template = "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading}
  {usage}

{all-args}{after-help}
",
)]
pub struct Args {
    /// Play a specific file.
    #[arg(env, long)]
    pub file: Option<String>,

    /// Terminal tick rate.
    #[arg(env, short, long, value_name = "MS", default_value = "100")]
    pub tick_rate: u64,

    /// Target FPS value.
    #[arg(env, short, long, default_value = "60.0")]
    pub fps: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    #[test]
    fn test_args() {
        Args::command().debug_assert();
    }
}
