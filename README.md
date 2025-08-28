# clap-i18n-richformatter
Clap rich formatter with i18n!

## Usage:

1. Add `clap-i18n-richformatter` to `Cargo.toml`:

```toml
clap-i18n-richformatter = "0.1.0"
```

2. Load:

```rust
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

```

3. run `LANG=zh_CN.UTF-8 cargo run -- -b` and `LANG=C cargo run -- -b`, will see:

```
错误: 未提供下列必要参数:
  --a

用法: arg_requires --a --b

如需了解更多信息，请输入 --help。
```

```
error: the following required arguments were not provided:
  --a

Usage: arg_requires --a --b

For more information, try '--help'.
```
