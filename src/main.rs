#[macro_use]
extern crate log;

mod analyze;

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let matches = clap::App::new(clap::crate_name!())
        .name(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .subcommand(
            clap::SubCommand::with_name("analyze")
            .arg_from_usage(
                "<DIR> 'Sets the directory to analyze'
                -v...  'Sets the level of verbosity'"))
        .get_matches();

    info!("Henlo.");
}
