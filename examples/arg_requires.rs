use clap::Parser;
use clap_i18n_richformatter::{ClapI18nRichFormatter, init_clap_rich_formatter_localizer};

#[derive(Debug, Parser)]
struct ExampleArg {
    #[arg(long, short)]
    a: bool,
    #[arg(long, short, requires = "a")]
    b: bool,
}

fn main() {
    init_clap_rich_formatter_localizer();
    let args = ExampleArg::try_parse()
        .map_err(|e| {
            let e = e.apply::<ClapI18nRichFormatter>();
            e.exit();
        })
        .unwrap();

    dbg!(args);
}
