struct OuterBeelineCode(i32);

impl From<&OuterBeelineCode> for InnerResult {
    fn from(item: OuterBeelineCode) -> Self {
        match item {
            OuterBeelineCode(0) => InnerResult::Ok(InnerResultElement {info: String::new()}),
            _ => InnerResult::UnknownError(InnerResultElement {info: String::new()}),
        }
    }
}
