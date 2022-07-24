# Email type

Email type for approach suggested by Alexis King - ["Parse, don't validate"](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/).

## How to use:

```rust
let email = Email::parse("lexi.lambda@gmail.com")?;
```