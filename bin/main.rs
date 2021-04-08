/// This is main.
///
/// See [`libsuccotash`] for implementation details.
/// See [`libsuccotash::bin_util`] for things related to the executable.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = libsuccotash::bin_util::get_args();
    let verbosity = matches.args.get("v").map(|v| v.occurs).unwrap_or(0);
    libsuccotash::bin_util::init_logging(verbosity)?;

    match matches.subcommand_name() {
        Some("analyze") => {
            let dir = matches
                .subcommand_matches("analyze")
                .unwrap()
                .value_of("DIR")
                .unwrap();
            async_std::task::block_on(libsuccotash::analyze::run(dir.into()));
        }
        Some(sub) => log::error!("Unknown subcommand '{}'", sub),
        None => log::error!("You haven't specified a subcommand; see help"),
    };

    Ok(())
}
