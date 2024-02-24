mod parser;

use std::collections::{HashMap, HashSet};

use clap::Parser;
use parser::{Record, Time};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// xlsx file path
    #[arg(short, long)]
    path: String,
    /// sheet name
    #[arg[short, long]]
    sheet: String,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let records = parser::parse(&args.path, &args.sheet)?;

    println!("Total records: {}", records.len());

    statistics(&records);

    Ok(())
}

fn statistics(records: &Vec<Record>) {
    let mut result: HashMap<&Time, Vec<&String>> = HashMap::new();

    for record in records {
        for time in &record.times {
            result.entry(time).or_insert(Vec::new()).push(&record.name);
        }
    }

    let mut result: Vec<(&Time, Vec<&String>)> = result.into_iter().collect();

    result.sort_by(|(_, names1), (_, names2)| names2.len().cmp(&names1.len()));

    let mut repeat_names = HashSet::new();

    result.iter_mut().for_each(|(time, names)| {
        names.retain(|name| !repeat_names.contains(*name));

        if names.len() == 0 {
            return;
        }
        println!("{} 共有 {} 人", time.to_string(), names.len());
        for name in names {
            print!("{}\t", name);
            repeat_names.insert(name.to_string());
        }
        println!("\n");
    });
}
