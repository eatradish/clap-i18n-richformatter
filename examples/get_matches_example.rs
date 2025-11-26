use clap::{CommandFactory, Parser};
use clap_i18n_derive::clap_i18n;
use clap_i18n_richformatter::CommandI18nExt;

#[derive(Debug, Parser)]
#[clap_i18n]
#[command(version, about = "An example program with i18n support")]
struct Args {
    #[arg(long, short)]
    a: bool,
    #[arg(long, short, requires = "a")]
    b: bool,
}

fn main() {
    let _matches = Args::command().get_matches_i18n();
}
