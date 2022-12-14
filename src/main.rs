use itertools::Itertools;
use onig::Regex;
use std::error::Error;
use std::fs;
use std::process::Command;

fn extract_microseconds(output: &str) -> Result<usize, Box<dyn Error>> {
    let out = output.lines().last().unwrap();
    let ansi_escape = Regex::new(r"\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])").unwrap();
    let out = ansi_escape.replace_all(out, "").to_string();
    let time = if out.ends_with("ms") {
        out["Time: ".len()..out.len() - 2].parse::<usize>()? * 1000
    } else {
        out["Time: ".len()..out.len() - 3].parse::<usize>()?
    };
    Ok(time)
}

fn main() -> Result<(), Box<dyn Error>> {
    let days = fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src/bin/"))?
        .filter_map(|p| p.ok()?.path().file_stem()?.to_str().map(str::to_string))
        .sorted()
        .collect::<Vec<_>>();
    let mut total_time = 0;
    for day in &days {
        let cmd = Command::new("cargo")
            .args(&["run", "--release", "--bin", day])
            .output()?;
        let output = String::from_utf8(cmd.stdout)?;
        println!("{}", output);
        total_time += extract_microseconds(&output)?;
    }
    print!("\x1b[4;1m");
    let days_completed = days.len();
    if days_completed == 25 {
        println!(
            "🎄 All days completed! 🎄 Total time: {}ms\x1b[0m",
            total_time / 1000
        );
    } else {
        println!(
            "{} days completed in {}ms\x1b[0m",
            days_completed,
            total_time / 1000
        );
    }
    Ok(())
}
