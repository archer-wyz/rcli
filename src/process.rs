use crate::opts::OutputFormat;
use anyhow::Result;
use csv::Reader;
use serde_json::Value;
use std::fs;

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let headers = reader.headers()?.clone();
    let mut records = Vec::new();
    for result in reader.records() {
        let record = result?;
        // zip 用于迭代器之间的元素配对，collect 用于将配对的元素转换为 json 对象
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        records.push(json_value);
    }
    println!("{:?}", records);
    let content = match format {
        OutputFormat::Json => serde_json::to_string(&records)?,
        OutputFormat::Yaml => serde_yaml::to_string(&records)?,
        OutputFormat::Toml => toml::to_string(&records)?,
    };
    fs::write(output, content)?;
    Ok(())
}
