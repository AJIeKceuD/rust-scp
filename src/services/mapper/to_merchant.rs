struct MerchantError {
    code: i32,
    info: String,
    repeat: bool,
}

impl From<InnerError> for