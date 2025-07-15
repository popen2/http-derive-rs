use http_derive::HttpStatus;

#[derive(HttpStatus)]
pub enum MyError {
    #[http(status = BAD_REQUEST)]
    Unsupported,

    #[http(status = SERVICE_UNAVAILABLE)]
    TryAgain,

    #[http(transparent)]
    Sub(MySubError),
}

#[derive(HttpStatus)]
pub enum MySubError {
    #[http(status = UNAUTHORIZED)]
    Unauthorized,

    #[http(status = UNAUTHORIZED)]
    EvenMoreUnauthorized,
}

#[cfg(feature = "http-02")]
#[test]
fn test_http_02() {
    use http_derive::http_02::status::StatusCode;

    for (err, status) in [
        (MyError::Unsupported, StatusCode::BAD_REQUEST),
        (
            MyError::Sub(MySubError::EvenMoreUnauthorized),
            StatusCode::UNAUTHORIZED,
        ),
    ] {
        assert_eq!(StatusCode::from(&err), status);
    }
}

#[cfg(feature = "http-1")]
#[test]
fn test_http_1() {
    use http_derive::http_1::status::StatusCode;

    for (err, status) in [
        (MyError::Unsupported, StatusCode::BAD_REQUEST),
        (
            MyError::Sub(MySubError::EvenMoreUnauthorized),
            StatusCode::UNAUTHORIZED,
        ),
    ] {
        assert_eq!(StatusCode::from(&err), status);
    }
}
