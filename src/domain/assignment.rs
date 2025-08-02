use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Assignment {
    id: Uuid,
    amount: f64,
    month: YearMonth,

    category_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct YearMonth(NaiveDate);

impl YearMonth {
    fn new(year: i32, month: u32) -> Self {
        Self(NaiveDate::from_ymd_opt(year, month, 1).unwrap())
    }
}
