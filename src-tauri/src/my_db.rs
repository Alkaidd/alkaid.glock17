use crate::utils::{check_db_file, get_db_file_path};

pub struct Record {
    pub id: i32,
    pub content: String,
}

use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Seek, Write};

pub struct Database {
    pub file: File,
}

pub trait TodoOp {
    fn open() -> Database;
    fn add_record(&mut self, _: &Record) -> Result<(), std::io::Error>;
    fn parse_record_line(_: &str) -> Record;
    fn read_records(&mut self) -> Vec<Record>;
    fn remove_record(&mut self, _: i32) -> Result<(), std::io::Error>;
}

impl TodoOp for Database {
    fn open() -> Database {
        check_db_file().unwrap();

        let db_file = get_db_file_path();

        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(db_file)
            .unwrap();
        Database { file }
    }

    fn add_record(&mut self, record: &Record) -> Result<(), std::io::Error> {
        let line = format!("{},{}\n", record.id, record.content);
        // writeln! 宏返回一个 Result，我们直接返回它
        writeln!(self.file, "{}", line)
    }

    fn parse_record_line(line: &str) -> Record {
        let fields: Vec<&str> = line.split(',').collect();

        if fields.len() == 1 {
            return Record {
                id: 0,
                content: "".to_string(),
            };
        }
        let content = fields[1..].join(",");
        Record {
            id: fields[0].parse::<i32>().unwrap(),
            content,
        }
    }

    fn read_records(&mut self) -> Vec<Record> {
        let reader = BufReader::new(&self.file);
        reader
            .lines()
            .map_while(Result::ok)
            .filter(|line| !line.is_empty())
            .map(|line| Self::parse_record_line(&line))
            .collect()
    }

    fn remove_record(&mut self, id: i32) -> Result<(), std::io::Error> {
        let reader = BufReader::new(&self.file);
        let mut lines = reader.lines().enumerate();
        let line = lines.find(|(_, line)| {
            let record = Self::parse_record_line(line.as_ref().unwrap());
            record.id == id
        });
        match line {
            Some((i, _)) => {
                let contents = fs::read_to_string(get_db_file_path()).unwrap();
                let new_contents = contents
                    .lines()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, line)| line)
                    .collect::<Vec<_>>()
                    .join("\n");
                self.file.seek(std::io::SeekFrom::Start(0)).unwrap();
                self.file.write_all(new_contents.as_bytes()).unwrap();
                self.file.set_len(new_contents.len() as u64).unwrap();
                Ok(())
            }
            // 未找到 id 对应行时返回一个错误
            None => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("No such record: {}", id),
            )),
        }
    }
}
