use chrono::{
    DateTime, Datelike, Local, NaiveDate, Weekday,
};

pub fn get_weekday_arr(month: &String) -> Vec<String> {
    let month = month.parse::<u32>().unwrap();
    let year = get_now_year();

    let days = days_in_month(year, month);
    let dates = (1..=days)
        .map(|d| NaiveDate::from_ymd_opt(year, month, d).unwrap())
        .map(|d| get_weekday(d.weekday()))
        .collect::<Vec<_>>();
    dates
}

pub fn get_year_month() -> (String, String) {
    let local: DateTime<Local> = Local::now();
    let mut year = local.year();
    let mut month = local.month();

    if month == 12 {
        year += 1;
        month = 1;
    } else {
        month += 1;
    }

    (year.to_string(), month.to_string())
}

fn get_weekday(w: Weekday) -> String {
    match w {
        Weekday::Mon => "月".to_owned(),
        Weekday::Tue => "火".to_owned(),
        Weekday::Wed => "水".to_owned(),
        Weekday::Thu => "木".to_owned(),
        Weekday::Fri => "金".to_owned(),
        Weekday::Sat => "土".to_owned(),
        Weekday::Sun => "日".to_owned(),
    }
}

fn get_now_year() -> i32 {
    let local: DateTime<Local> = Local::now();
    return local.year()
}

fn days_in_month(year: i32, month: u32) -> u32 {
    let date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let next_month = if month == 12 {
        NaiveDate::from_ymd_opt(year+1, 1, 1).unwrap()
    } else {
        NaiveDate::from_ymd_opt(year, month+1, 1).unwrap()
    };
    (next_month - date).num_days() as u32
}