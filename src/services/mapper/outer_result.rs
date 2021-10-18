use serde::{Deserialize, Serialize};
use super::inner_result::{InnerResult, InnerResultInfo, InnerResultElement, InnerResultRepeat};

pub struct OuterResult {}

impl OuterResult {
    pub fn get_code(inner_result: &InnerResult) -> OuterResultCode {
        OuterResultCode::from(inner_result)
    }

    pub fn get_info(inner_result: &InnerResult) -> OuterResultInfo {
        OuterResultInfo::from(inner_result)
    }

    pub fn is_repeatable(inner_result: &InnerResult) -> Option<OuterResultRepeat> {
        match inner_result {
            InnerResult::Ok(_) => Option::None,
            _ => Some(OuterResultRepeat::from(inner_result)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OuterResultCode(pub i32);

impl From<&InnerResult> for OuterResultCode {
    fn from(item: &InnerResult) -> Self {
        match item {
            InnerResult::Ok(_) => OuterResultCode(0),
            InnerResult::ErrorUnknown(_) => OuterResultCode(1),
            InnerResult::ErrorIncomeData(_) => OuterResultCode(10),
            InnerResult::ErrorIncomeDataParse(_) => OuterResultCode(11),
            InnerResult::ErrorActionUnknown(_) => OuterResultCode(20),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OuterResultInfo(String);

impl From<&InnerResult> for OuterResultInfo {
    fn from(item: &InnerResult) -> Self {
        match item {
            _ => OuterResultInfo(item.get_info()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OuterResultRepeat(bool);

impl OuterResultRepeat {
    pub fn is_repeatable(inner_result: &InnerResult) -> Option<Self> {
        match inner_result {
            InnerResult::Ok(_) => Option::None,
            _ => Some(Self::from(inner_result)),
        }
    }
}

impl From<&InnerResult> for OuterResultRepeat {
    fn from(item: &InnerResult) -> Self {
        match item.is_repeatable().0 {
            false => OuterResultRepeat(false),
            true => OuterResultRepeat(true),
        }
    }
}
