// TODO remake unwrap?
#[macro_export]
macro_rules! response_json {
    ($response_obj:expr) => {
        {
            Response::builder()
            .header("Foo", "Bar")
            .header("Content-Type", "application/json")
            .status(StatusCode::OK)
            .body::<Body>(
                serde_json::to_string($response_obj).unwrap().to_string().into()
            )
            .unwrap()
        }
    }
}