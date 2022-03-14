use crate::json_log::JsonLog;
use chrono::Utc;
use clap::Parser;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};

#[macro_use]
extern crate serde_derive;

mod json_log;

#[derive(Parser)]
#[clap(version = "0.1.0", author = "Yan Mingzhi <realraindust@gmail.com>")]
pub struct Opts {
    #[clap(short = 't', long)]
    pub show_time: bool,

    #[clap(short = 'l', long, default_value = "1024")]
    pub max_flush_lines: usize,

    #[clap(short = 'p', long)]
    pub path: String,

    #[clap(short = 'o', long)]
    pub output: Option<String>,

    #[clap(short = 's', long)]
    pub strip_ansi_color: bool,
}

fn default_log_name() -> String {
    format!("temp_{}.log", Utc::now().format("%Y-%m-%d_%H-%M-%S"))
}

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();

    let in_file = File::open(&opts.path)?;
    let reader = BufReader::new(in_file);

    let out_file_name = opts.output.clone().unwrap_or(default_log_name());
    process_and_save(&out_file_name, reader, &opts)?;

    if opts.output.is_none() {
        std::fs::rename(out_file_name, &opts.path)?;
    }

    Ok(())
}

fn process_and_save(
    out_file_name: &str,
    reader: BufReader<File>,
    opts: &Opts,
) -> anyhow::Result<()> {
    let out_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(out_file_name)?;
    let mut writer = BufWriter::new(out_file);

    let mut flush_index = 0;
    for line in reader.lines() {
        let value: JsonLog = serde_json::from_str(line?.as_str())?;
        if opts.strip_ansi_color {
            writer.write(&strip_ansi_escapes::strip(value.log.as_bytes())?)?;
        } else {
            writer.write(value.log.as_bytes())?;
        }

        flush_index += 1;
        if flush_index >= opts.max_flush_lines {
            writer.flush()?;
            flush_index = 0;
        }
    }

    if flush_index != 0 {
        writer.flush()?;
    }
    Ok(())
}
