# human-chrono-parser

Parse human-written relative dates like "today" ,"tomorrow", "in 3 days",
"next monday" and other variants.

## Installation

To use the `human-chrono-parser` in your project, include it in your
`Cargo.toml`:

```toml
[dependencies]
human-chrono-parser = "0.0.1"
```

For Python, you can install the bindings using pip:

```bash
pip install human-chrono-parser
```

## Usage

### Rust Exemple

Here is a basic example of how to use the `human_chrono_parser::parse` functino in Rust:

```rust
use chrono::{Days, NaiveDate};
use human_chrono_parser::locales::Locale;

fn main() {
    let now = NaiveDate::from_ymd_opt(2024, 8, 13).unwrap(); // Example: Tuesday, August 13, 2024

    let expr = human_chrono_parser::parse(&mut "amanhã", &Locale::BrazilianPortuguese).unwrap();
    let tommorow = expr.relative_to(&now);
    println!("{:?}", tommorow);
    // outputs: Some(2024-08-14)

    assert_eq!(tommorow, now.checked_add_days(Days::new(1)));
}
```

### Python Example

Here is a basic example of how to use the `human-chrono-parser` in Python:

```python
from datetime import date, timedelta
from human_chrono_parser import parse, Locale

if __name__ == "__main__":
    now = date(2024, 8, 13)  # Example: Tuesday, August 13, 2024

    expr = parse("amanhã", locale=Locale.BRAZILIAN_PORTUGUESE)
    tomorrow = expr.relative_to(now)
    print(tomorrow)
    # outputs: datetime.datetime(2024, 8, 14)

    assert tomorrow == now + timedelta(days=1)
```

## Locales

Currently only BrazilianPortuguese (pt-BR) locale is supported. **We welcome  contributions to
support other locales!**


## Contributing

Contributions are welcome! If you'd like to improve the library or add more features, please open an
issue, fork the repository and create a pull request.
