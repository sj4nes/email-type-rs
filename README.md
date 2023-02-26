# Email type

Email type for approach suggested by Alexis King - ["Parse, don't validate"](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/).

## How to use:

Add dependency:

```toml
email-type-rs = { git = "https://github.com/lebe-dev/email-type-rs", version = "1.0.0", features = ["utils"] }
```

Use:

```rust
let email = Email::from_str("lexi.lambda@gmail.com")?;
let email = "lexi.lambda@gmail.com".parse()?;

// fn some_func(value: &str)
some_func(email.as_str());
some_func(&email);
```

## Util functions

Add to `Cargo.toml`:

```toml
[dev-dependencies]
email-type-rs = { git = "https://github.com/lebe-dev/email-type-rs", version = "1.0.0", features = ["utils"] }
```

Functions:

- `get_random_email()` - return random `Email`. Useful for tests.

## Thanks

- Alexis King, article - [Parse, don't validate](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/)
- Justin Wernick, article - [The Newtype Pattern In Rust](https://www.worthe-it.co.za/blog/2020-10-31-newtype-pattern-in-rust.html)
