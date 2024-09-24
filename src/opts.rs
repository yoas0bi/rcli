use std::path::Path;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", version = "1.0", author = "yang", about = "convert csv to json", long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "convert csv to json")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    // 添加缺失的字段
    #[arg(long, default_value_t = true)]
    pub header: bool,

    #[arg(short, long, default_value = ",")]
    pub delimiter: char,
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    let path = Path::new(filename);
    if path.exists() {
        Ok(filename.to_string())
    } else {
        Err(format!("{} not exists", filename))
    }
}
