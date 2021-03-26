#[macro_use]
extern crate log;

use async_std::task;

mod analyze;

fn init_logging(verbosity: u64) -> Result<(), Box<dyn std::error::Error>> {
    let level = match verbosity {
        0 => log::LevelFilter::Info,
        1 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    let colors = fern::colors::ColoredLevelConfig::new()
        .info(fern::colors::Color::Green);

    fern::Dispatch::new()
        // Based on fern's usage example
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} {} [{}] {}",
                chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .level_for(clap::crate_name!(), level)
        .chain(std::io::stdout())
        .apply()?;

    Ok(())
}

fn get_args() -> clap::ArgMatches<'static> {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = get_args();
    let verbosity = args.args.get("v").map(|v| v.occurs).unwrap_or(0);
    init_logging(verbosity)?;

    match args.subcommand_name() {
        Some("analyze") => {
            let dir = args
                .subcommand_matches("analyze")
                .unwrap()
                .value_of("DIR")
                .unwrap();
            task::block_on(analyze::run(dir.into()));
        }
        Some(sub) => error!("Unknown subcommand '{}'", sub),
        None => error!("You haven't specified a subcommand; see help"),
    };

    Ok(())
}
