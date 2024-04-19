use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
struct Player {
    #[serde[rename = "姓名"]]
    name: String,
    #[serde[rename = "号码"]]
    kit: u8,
    #[serde[rename = "位置"]]
    position: String,
    #[serde[rename = "身高（英尺-英寸）"]]
    height: String,
    #[serde[rename = "体重（磅）"]]
    weight: u16,
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let records = reader
        .deserialize()
        .map(|record| record.unwrap())
        .collect::<Vec<Player>>();
    println!("{:?}", records);

    let json = serde_json::to_string(&records)?;
    fs::write(output, json)?;

    Ok(())
}
