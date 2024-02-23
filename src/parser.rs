use calamine::{open_workbook, DataType, Error, Reader, Xlsx};

pub struct Record {
    pub name: String,
    pub times: Vec<Time>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Time {
    SunAfternoon,
    SunEvening,
    MonEvening,
    TueEvening,
    WdeAfternoon,
    WedEvening,
}

impl Time {
    fn from_str(s: &str) -> Self {
        match s.trim() {
            "2.25（周日） 15:00" => Time::SunAfternoon,
            "2.25（周日） 19:00" => Time::SunEvening,
            "2.26（周一） 21:30" => Time::MonEvening,
            "2.27（周二） 21:30" => Time::TueEvening,
            "2.28（周三） 15:00" => Time::WdeAfternoon,
            "2.28（周三） 21:30" => Time::WedEvening,
            _ => panic!("Invalid time"),
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Time::SunAfternoon => "2.25（周日） 15:00",
            Time::SunEvening => "2.25（周日） 19:00",
            Time::MonEvening => "2.26（周一） 21:30",
            Time::TueEvening => "2.27（周二） 21:30",
            Time::WdeAfternoon => "2.28（周三） 15:00",
            Time::WedEvening => "2.28（周三） 21:30",
        }
    }
}

pub fn parse(path: &str) -> Result<Vec<Record>, Error> {
    let mut excel: Xlsx<_> = open_workbook(path)?;
    let mut records: Vec<Record> = Vec::new();
    if let Ok(r) = excel.worksheet_range("Sheet1") {
        for row in r.rows().skip(1) {
            if row[5].is_empty() {
                continue;
            }
            let times: Vec<Time> = row[5]
                .as_string()
                .ok_or("no time data")?
                .split(|c| c == '，' || c == ',')
                .collect::<Vec<&str>>()
                .iter()
                .map(|&s| Time::from_str(s))
                .collect();
            let name = row[6].as_string().ok_or("no name data")?;
            records.push(Record { name, times });
        }
    }

    Ok(records)
}
