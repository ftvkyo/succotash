//! Utilities for the executable shipped with the succotash library.
//!
//! This file is in the library part to allow rustdoc example testing.

/// Initialize fern logger with specified verbosity
///
/// # Arguments
///
/// * `verbosity` - Level of verbosity to set. Higher value = more verbosity.
///
/// # Examples
///
/// ```
/// # use libsuccotash::bin_util;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// bin_util::init_logging(0)?; // Initialize logging with minimal verbosity.
/// # Ok(())
/// # }
/// ```
pub fn init_logging(verbosity: u64) -> Result<(), Box<dyn std::error::Error>> {
    let level = match verbosity {
        0 => log::LevelFilter::Info,
        1 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    let cute = matches!(verbosity, 0);

    let colors = fern::colors::ColoredLevelConfig::new().info(fern::colors::Color::Green);

    fern::Dispatch::new()
        // Based on fern's usage example
        .format(move |out, message, record| {
            if cute {
                out.finish(format_args!(
                    "[{}] {}",
                    colors.color(record.level()),
                    message
                ));
            } else {
                out.finish(format_args!(
                    "{} {} [{}] {}",
                    chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                    record.target(),
                    colors.color(record.level()),
                    message
                ));
            }
        })
        .level(log::LevelFilter::Info)
        .level_for(clap::crate_name!(), level)
        .level_for(format!("lib{}", clap::crate_name!()), level)
        .chain(std::io::stdout())
        .apply()?;

    info!("Using log level {}", level);
    if cute {
        info!("Using cute output mode");
    }

    Ok(())
}

/// Generate argument parser and parse command line arguments with it.
///
/// Returns a struct with the args.
///
/// # Examples
///
/// Let's say cmdline is `binary_name -vv`
/// ```
/// # let matches = clap::App::new(clap::crate_name!())
/// #     .arg_from_usage("-v... 'Verbosity'")
/// #     .get_matches_from(["binary_name", "-vv"].iter());
/// // let matches = bin_util::get_args();
/// let verbosity = matches.args.get("v").map(|v| v.occurs).unwrap_or(0);
/// assert_eq!(verbosity, 2);
/// ```
pub fn get_args() -> clap::ArgMatches<'static> {
    clap::App::new(clap::crate_name!())
        .name(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg_from_usage("-v... 'Sets the level of verbosity'")
        .subcommand(
            clap::SubCommand::with_name("analyze")
                .arg_from_usage("<DIR> 'Sets the directory to analyze'"),
        )
        .get_matches()
}
