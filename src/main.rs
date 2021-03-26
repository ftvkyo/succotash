#[macro_use]
extern crate log;

use async_std::task;

mod analyze;

fn init_logging(verbosity: usize) -> Result<(), Box<dyn std::error::Error>> {
    let level = match verbosity {
        0 => log::LogLevelFilter::Info,
        1 => log::LogLevelFilter::Debug,
        _ => log::LogLevelFilter::Trace,
    };

    Ok(pretty_logger::init_level(level)?)
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

    let verbosity = args.values_of("v").map(|v| v.count()).unwrap_or(0);
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
        Some(sub) => error!("Unknown subcommand {}.", sub),
        None => error!("You haven't specified a subcommand. See help."),
    };

    Ok(())
}
