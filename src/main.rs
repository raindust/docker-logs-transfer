use crate::json_log::JsonLog;
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

    #[clap(short = 'o', long, default_value = "out.log")]
    pub output: String,
}

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();

    let in_file = File::open(opts.path)?;
    let reader = BufReader::new(in_file);

    let out_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(opts.output)?;
    let mut writer = BufWriter::new(out_file);

    let mut flush_index = 0;
    for line in reader.lines() {
        let value: JsonLog = serde_json::from_str(line?.as_str())?;
        writer.write(value.log.as_bytes())?;

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
