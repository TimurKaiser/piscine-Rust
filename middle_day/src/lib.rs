pub use chrono::*;
pub use chrono::Weekday as wd;


pub fn middle_day(y: i32) -> Option<Weekday> {

    let start_year = chrono::NaiveDate::from_ymd_opt(y, 01, 01).unwrap();
    let start_next_year = chrono::NaiveDate::from_ymd_opt(y+1, 01, 01).unwrap();
    let duration = start_year - start_next_year;
    let total_seconds = duration.num_seconds().abs();
    let total_days = total_seconds / 86400; 

    if total_days % 2 == 0 {
        println!("Année à 366 jours : CIAO");
        return None
    } else {
        println!("On continue les investigations");

        let day_to_reach = chrono::NaiveDate::from_ymd_opt(y, 07, 02).unwrap();
        println!("{}", day_to_reach.weekday());
        Some(day_to_reach.weekday())
    }
}