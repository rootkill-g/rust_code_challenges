use chrono::{Utc, TimeZone, NaiveDate};

trait Deadline {
    fn is_passed(&self) -> bool;
}

#[derive(Debug)]
struct Event {
    when: NaiveDate,
}

impl Deadline for Event {
    fn is_passed(&self) -> bool {
        let now = Utc::now().date_naive();
        now > self.when
    }
}

fn main() {
    let date = Utc.with_ymd_and_hms(2023, 2, 2, 11, 5, 0).unwrap();
    let event = Event {
        when: date.date_naive(),
    };

    // println!("{event:#?}");

    println!("Is the event passed? : {}", event.is_passed());
}

#[test]
fn test_past_event() {
    let date = Utc.with_ymd_and_hms(2022, 12, 1, 0, 0, 0).unwrap();
    let event = Event {
        when: date.date_naive(),
    };
        assert_eq!(event.when, NaiveDate::from_ymd_opt(2022, 12, 1).unwrap());
}

#[test]
fn test_future_event() {
    let date = Utc.with_ymd_and_hms(2023, 2, 2, 11, 5, 0).unwrap();
    let event  = Event {
        when: date.date_naive(),
    };
    assert_eq!(event.when, NaiveDate::from_ymd_opt(2023, 2, 2).unwrap());
}
