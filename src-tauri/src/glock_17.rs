use chrono::{NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct EventLine {
    pub id: i32,
    pub event: String,
    pub start_time: String,
    pub end_time: String,
    pub person_1: String,
    pub person_2: String,
    pub person_3: String,
}

#[derive(Deserialize, Serialize)]
pub struct EventData {
    pub data: Vec<EventLine>,
}

struct PersonData {
    id: i32,
    event: String,
    recorded: bool,
    periods: Vec<[i32; 2]>,
}

impl EventData {
    pub fn new() -> Self {
        EventData { data: Vec::new() }
    }

    pub fn check_data_conflict(&self) -> EventData {
        let mut person_map: HashMap<String, PersonData> = HashMap::new();

        let event_lines: Vec<EventLine> = vec![];

        for line in self.data.iter() {
            let start = format_date_into_timestamp(&line.start_time);
            let end = format_date_into_timestamp(&line.end_time);
            match person_map.get_mut(&line.person_1) {
                Some(person_data) => {
                    for period in person_data.periods.clone().into_iter() {
                        // check if period confict here
                        if check_overlap(&[start, end], &period) {
                            person_data.recorded = true
                        }
                    }

                    let new_period = [start, end];
                    person_data.periods.push(new_period);
                }
                _ => {
                    let period = [start, end];
                    person_map.insert(
                        line.person_1.clone(),
                        PersonData {
                            id: line.id,
                            event: line.event.clone(),
                            recorded: false,
                            periods: vec![period],
                        },
                    );
                }
            }
        }
        EventData { data: event_lines }
    }
}

fn format_date_into_timestamp(date: &str) -> i32 {
    let fmt = "%Y-%m-%d %H:%M:%S";

    let date_time = NaiveDateTime::parse_from_str(&date, fmt).unwrap();
    let timestamp = Utc.from_utc_datetime(&date_time).timestamp() as i32;
    timestamp
}

fn check_overlap(date_1: &[i32; 2], date_2: &[i32; 2]) -> bool {
    if (date_1[0] < date_2[0] && date_1[1] < date_2[0])
        || (date_1[0] > date_2[1] && date_1[1] > date_2[1])
    {
        return false;
    }
    true
}
