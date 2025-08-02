use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Assignment {
    pub id: Uuid,
    pub amount: f64,
    pub year_month: YearMonth,

    pub category_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct YearMonth(NaiveDate);

impl YearMonth {
    pub fn new(year: i32, month: u32) -> Self {
        Self(NaiveDate::from_ymd_opt(year, month, 1).unwrap())
    }
}
