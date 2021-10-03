use serde::{Deserialize, Serialize};

// pub struct InnerResults;
//
// impl InnerResults {
//     pub const OK: InnerResult = InnerResult { code: InnerResultCode(0), info: InnerResultInfo(String::new()), repeat: InnerResultRepeat(false)};
//     pub const ERROR_UNKNOWN: InnerResult = InnerResult { code: InnerResultCode(1), info: InnerResultInfo(String::new()), repeat: InnerResultRepeat(false)};
//     pub const ERROR_INCOME_DATA_PARSE: InnerResult = InnerResult { code: InnerResultCode(10), info: InnerResultInfo(String::new()), repeat: InnerResultRepeat(false)};
// }
#[derive(Debug, Serialize, Deserialize)]
pub enum InnerResult {
    Ok(InnerResultElement),
    ErrorUnknown(InnerResultElement),
    ErrorIncomeData(InnerResultElement), // default error for bad income data
    ErrorIncomeDataParse(InnerResultElement),
    ErrorActionUnknown(InnerResultElement),
}

impl InnerResult {
    pub const fn is_repeatable(&self) -> InnerResultRepeat {
        match self {
            Self::ErrorIncomeDataParse(_) => InnerResultRepeat(true),
            _ => InnerResultRepeat(false),
        }
    }

    // Bad copypaste, dont know better way
    pub fn get_info(&self) -> String {
        match self {
            Self::Ok(item)
            | Self::ErrorUnknown(item)
            | Self::ErrorIncomeData(item)
            | Self::ErrorIncomeDataParse(item)
            | Self::ErrorActionUnknown(item)
            => item.info.0.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InnerResultElement {
    // pub code: InnerResultCode,
    pub info: InnerResultInfo,
    // pub repeat: InnerResultRepeat, use is_repeatable
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct InnerResultCode(i32);
#[derive(Debug, Serialize, Deserialize)]
pub struct InnerResultInfo(pub String);
impl InnerResultInfo {
    pub const OK: &'static str = "Success";
    pub const ERROR_UNKNOWN: &'static str = "Unknown error";
    pub const ERROR_INCOME_DATA: &'static str = "Bad income data";
    pub const ERROR_INCOME_DATA_BAD_JSON: &'static str = "Bad income data - cant parse JSON";
    pub const ERROR_INCOME_DATA_BAD_VERSION: &'static str = "Bad income data - version undefined or unsupported";
    pub const ERROR_ACTION_UNKNOWN: &'static str = "Action undefined";

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InnerResultRepeat(pub bool);

// impl From<i32> for InnerResult {
//     fn from(item: i32) -> Self {
//         match item {
//             _ => InnerResult::Ok(InnerResultElement {info: InnerResultInfo(String::new())})
//         }
//     }
// }
