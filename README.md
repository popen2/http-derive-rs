# 🦀 http-derive

Derive macro for implementing `Into<http::StatusCode>` for your enums using the standard http types.

## Why?

* It uses the commonly-used `http::StatusCode`.
* Selects `http` 0.2 and/or 1.0 using feature flags.
* Works well with other libraries like `thiserror`.
* Easier to define as part of an error `enum` instead of manually `impl From<X> for http::StatusCode`.

## Inspiration

* [as_status_code](https://crates.io/crates/as_http_status_code)
* [actix-web-error](https://crates.io/crates/actix-web-error)

## Features

**By default this crate enables no features**. This is to allow selecting which `http` crate/s are selected.

If your crate has to enable `http` support conditionally you can leave the `[#derive(HttpStatus)]` in place and turn on one or more of the features below depending on your crate's feature flags.

* `http-1` enables `http = "1"`
* `http-02` enables `http = "0.2"`

For example, in `Cargo.toml` add:

```toml
http-derive = { version = "0.1.0", features = ["http-1"] }
```

## Examples

```rust
use http_derive::HttpStatus;

#[derive(Debug, HttpStatus, thiserror::Error)]
pub enum MyError {
    #[error("This is not supported")]
    #[http(status = "BAD_REQUEST")]
    Unsupported,

    #[error("Try again")]
    #[http(status = "SERVICE_UNAVAILABLE")]
    TryAgain,

    #[error(transparent)]
    #[http(transparent)]
    Sub(MySubError),
}

#[derive(Debug, HttpStatus, thiserror::Error)]
pub enum MySubError {
    #[error("Unauthorized")]
    #[http(status = "UNAUTHORIZED")]
    Unauthorized,

    #[error("Unauthorized")]
    #[http(status = "UNAUTHORIZED")]
    EvenMoreUnauthorized,
}
```
