use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct User {
    name: String,
    position: String,
    dob: String,
    nationality: String,
    kitnumber: u8,
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let record: User = result?;
        println!("{:?}\n", record);
        ret.push(record);
    }
    let json = serde_json::to_string_pretty(&ret)?;
    std::fs::write(output, json)?;
    Ok(())
}
