use chrono::{Datelike, Duration, Utc};

pub enum StartTime {
    Today,
    StartOfWeek,
}

impl StartTime {
    fn to_string(&self) -> &'static str {
        match self {
            StartTime::Today => "today",
            StartTime::StartOfWeek => "startOfWeek",
        }
    }

    fn from_string(value: &str) -> StartTime {
        match value {
            "today" => StartTime::Today,
            "startOfWeek" => StartTime::StartOfWeek,
            _ => unimplemented!(),
        }
    }

    fn get_date_value(&self) -> String {
        match self {
            StartTime::Today => "idag".to_string(),
            StartTime::StartOfWeek => {
                let current_time = Utc::now();
                let start_of_week = current_time
                    + Duration::days(current_time.weekday().num_days_from_monday().into());
                return start_of_week.format("%Y-%m-%d").to_string();
            }
        }
    }
}
