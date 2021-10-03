// TODO remake unwrap?
#[macro_export]
macro_rules! response_json {
    ($response_obj:expr) => {
        {
            let response_data = &$response_obj.data;
            let response_headers = &$response_obj.headers;
            let response_status = &$response_obj.status.unwrap_or(StatusCode::OK);

            Response::builder()
            .header("Foo", "Bar")
            .header("Content-Type", "application/json")
            .status(response_status)
            .body::<Body>(
                serde_json::to_string(response_data).unwrap().to_string().into()
            )
            .unwrap()
        }
    }
}
