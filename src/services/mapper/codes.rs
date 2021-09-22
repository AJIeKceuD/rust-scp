// enum InnerCode {
//     Ok = 0,
//     ErrorUnknown = 1,
//     ErrorIncomeDataParse = 10,
// }
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InnerResultCode(i32);
#[derive(Debug, Serialize, Deserialize)]
pub struct InnerResultInfo(String);
#[derive(Debug, Serialize, Deserialize)]
pub struct InnerResultRepeat(bool);

pub struct InnerResult {
    pub code: InnerResultCode,
    pub info: InnerResultInfo,
    pub repeat: InnerResultRepeat,
}

pub struct InnerCode;

impl InnerCode {
    pub const OK: InnerResult = InnerResult { code: InnerResultCode(0), info: InnerResultInfo(String::new()), repeat: InnerResultRepeat(false)};
    pub const ERROR_UNKNOWN: InnerResult = InnerResult { code: InnerResultCode(1), info: InnerResultInfo(String::new()), repeat: InnerResultRepeat(false)};
    pub const ERROR_INCOME_DATA_PARSE: InnerResult = InnerResult { code: InnerResultCode(10), info: InnerResultInfo(String::new()), repeat: InnerResultRepeat(false)};
}
