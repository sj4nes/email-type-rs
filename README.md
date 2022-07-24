# Email type

Email type for approach suggested by Alexis King - ["Parse, don't validate"](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/).

## How to use:

Add dependency:

```toml
email-type-rs = { git = "https://github.com/lebe-dev/email-type-rs", version = "0.1.0", features = ["utils"] }
```

Use:

```rust
let email = Email::parse("lexi.lambda@gmail.com")?;
```

## Util functions

Add to `Cargo.toml`:

```toml
[dev-dependencies]
email-type-rs = { git = "https://github.com/lebe-dev/email-type-rs", version = "0.1.0", features = ["utils"] }
```

Functions:

- `get_random_email()` - return random `Email`. Useful for tests.