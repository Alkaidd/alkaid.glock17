use chrono::{NaiveDateTime, ParseError, TimeZone, Utc};
use office::{DataType, Excel};
use rust_xlsxwriter::{Workbook, XlsxError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone)]
pub struct EventLine {
    pub id: i32,
    pub event: String,
    pub start_time: String,
    pub end_time: String,
    pub person_1: String,
    pub person_2: String,
    pub person_3: String,
}
impl EventLine {
    fn get_field_val(&self, field: &str) -> Option<String> {
        match field {
            "id" => Some(self.id.to_string()),
            "event" => Some(self.event.clone()),
            "start_time" => Some(self.start_time.clone()),
            "end_time" => Some(self.end_time.clone()),
            "person_1" => Some(self.person_1.clone()),
            "person_2" => Some(self.person_2.clone()),
            "person_3" => Some(self.person_3.clone()),
            _ => None,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct EventOverlap {
    pub id: i32,
    pub event: String,
    pub start_time: String,
    pub end_time: String,
    pub person_1: String,
    pub person_2: String,
    pub person_3: String,
    pub overlap_fields: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct EventData {
    pub data: Vec<EventLine>,
}

#[derive(Deserialize, Serialize)]
pub struct EventOverlapData {
    pub data: Vec<EventOverlap>,
}

#[derive(Deserialize, Serialize)]
pub struct ResData<T> {
    pub status: String,
    pub data: T,
}

struct PersonData {
    events: Vec<EventLine>,
    periods: Vec<[i32; 2]>,
}

impl EventData {
    pub fn new(data: Vec<EventLine>) -> Self {
        EventData { data }
    }

    pub fn check_data_conflict(&self) -> EventOverlapData {
        let mut person_map: HashMap<String, PersonData> = HashMap::new();

        let mut event_map: HashMap<i32, EventOverlap> = HashMap::new();

        // 遍历所有数据
        for line in &self.data {
            let start = match format_date_into_timestamp(&line.start_time) {
                Ok(val) => val,
                Err(_) => continue,
            };
            let end = match format_date_into_timestamp(&line.end_time) {
                Ok(val) => val,
                Err(_) => continue,
            };
            let fields = ["person_1", "person_2", "person_3"];
            // 遍历每条数据的人员
            for field in fields {
                let temp_val = line.get_field_val(field).unwrap();
                if temp_val == "" {
                    continue;
                }
                // 查看人员表是否已经进入排期
                match person_map.get_mut(&temp_val) {
                    // 如果已经进入排期,查看排期是否有冲突
                    Some(person_data) => {
                        for (index, period) in person_data.periods.iter().enumerate() {
                            // check if period confict here
                            if check_overlap(&[start, end], &period) {
                                // 检查前一个冲突事件是否已经在表内了,在的话检查冲突人员是否已经在里面了
                                let front_event = &person_data.events[index];
                                match event_map.get_mut(&front_event.id) {
                                    Some(event_data) => {
                                        let mut has_person: bool = false;
                                        let mut front_field: String = "".to_string();

                                        for temp_field in fields {
                                            let temp_field_val =
                                                front_event.get_field_val(temp_field).unwrap();
                                            if temp_field_val == temp_val {
                                                front_field = temp_field.to_string();
                                            }
                                        }

                                        if front_field == "" {
                                            break;
                                        }

                                        for person in event_data.overlap_fields.iter() {
                                            if *person == front_field {
                                                has_person = true
                                            }
                                        }
                                        if !has_person {
                                            event_data.overlap_fields.push(front_field.to_string())
                                        }
                                    }
                                    _ => {
                                        let mut front_field = "".to_string();

                                        for temp_field in fields {
                                            let temp_field_val =
                                                front_event.get_field_val(temp_field).unwrap();
                                            if temp_field_val == temp_val {
                                                front_field = temp_field.to_string();
                                            }
                                        }

                                        if front_field == "" {
                                            break;
                                        }

                                        let event_overlap_data = EventOverlap {
                                            id: front_event.id,
                                            event: front_event.event.clone(),
                                            start_time: front_event.start_time.clone(),
                                            end_time: front_event.end_time.clone(),
                                            person_1: front_event.person_1.clone(),
                                            person_2: front_event.person_2.clone(),
                                            person_3: front_event.person_3.clone(),
                                            overlap_fields: vec![front_field],
                                        };
                                        event_map.insert(front_event.id, event_overlap_data);
                                    }
                                }

                                // 随后检查第二个冲突的事件是否已经在表中了
                                match event_map.get_mut(&line.id) {
                                    Some(event_data) => {
                                        let mut has_person: bool = false;
                                        for person in event_data.overlap_fields.iter() {
                                            if person == field {
                                                has_person = true
                                            }
                                        }
                                        if !has_person {
                                            event_data.overlap_fields.push(field.to_string())
                                        }
                                    }
                                    _ => {
                                        let event_overlap_data = EventOverlap {
                                            id: line.id,
                                            event: line.event.clone(),
                                            start_time: line.start_time.clone(),
                                            end_time: line.end_time.clone(),
                                            person_1: line.person_1.clone(),
                                            person_2: line.person_2.clone(),
                                            person_3: line.person_3.clone(),
                                            overlap_fields: vec![field.to_string()],
                                        };
                                        event_map.insert(line.id, event_overlap_data);
                                    }
                                }
                            }
                        }

                        let new_period = [start, end];
                        person_data.periods.push(new_period);
                        person_data.events.push(line.clone());
                    }
                    _ => {
                        let period = [start, end];
                        person_map.insert(
                            temp_val.clone(),
                            PersonData {
                                events: vec![line.clone()],
                                periods: vec![period],
                            },
                        );
                    }
                }
            }
        }

        let overlap_events = event_map.values().cloned().collect();

        EventOverlapData {
            data: overlap_events,
        }
    }

    pub fn parse_from_xlsx(path: &str) -> Result<EventData, office::Error> {
        let mut event_data = EventData { data: vec![] };
        let mut workbook = match Excel::open(path) {
            Ok(val) => val,
            Err(error) => return Err(error),
        };
        let range = workbook.worksheet_range("Sheet1").unwrap();
        println!("started to parse row...");
        for row in range.rows().skip(1) {
            if let (
                Some(DataType::String(id)),
                Some(DataType::String(event)),
                Some(DataType::String(start_time)),
                Some(DataType::String(end_time)),
                Some(DataType::String(person_1)),
                Some(DataType::String(person_2)),
            ) = (
                row.get(0),
                row.get(1),
                row.get(2),
                row.get(3),
                row.get(4),
                row.get(5),
            ) {
                let int_id = match id.parse::<i32>() {
                    Ok(val) => val,
                    _ => -1,
                };
                let person_3 = match row.get(6) {
                    Some(DataType::String(val)) => val,
                    _ => "",
                };
                let event_line = EventLine {
                    id: int_id,
                    event: event.clone(),
                    start_time: start_time.clone(),
                    end_time: end_time.clone(),
                    person_1: person_1.clone(),
                    person_2: person_2.clone(),
                    person_3: person_3.to_string(),
                };

                event_data.data.push(event_line);
            }
        }

        Ok(event_data)
    }

    pub fn xlsx_from_data(&self, path: &str) -> Result<(), XlsxError> {
        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();
        let header = [
            "id",
            "event",
            "start_time",
            "end_time",
            "person_1",
            "person_2",
            "person_3",
        ];
        for (index, &field) in header.iter().enumerate() {
            worksheet.write(0, index as u16, field.to_string())?;
        }

        for (index, event_line) in self.data.iter().enumerate() {
            let row_id = index + 1;
            for column_id in 0..header.len() {
                worksheet.write(
                    row_id as u32,
                    column_id as u16,
                    event_line.get_field_val(header[column_id]),
                )?;
            }
        }

        workbook.save(path)?;
        Ok(())
    }
}

fn format_date_into_timestamp(date: &str) -> Result<i32, ParseError> {
    let fmt = "%Y-%m-%d %H:%M:%S";

    NaiveDateTime::parse_from_str(&date, fmt).map(|date_time| {
        let timestamp = Utc.from_utc_datetime(&date_time).timestamp() as i32;
        timestamp
    })
}

fn check_overlap(date_1: &[i32; 2], date_2: &[i32; 2]) -> bool {
    if (date_1[0] < date_2[0] && date_1[1] < date_2[0])
        || (date_1[0] > date_2[1] && date_1[1] > date_2[1])
    {
        return false;
    }
    true
}
