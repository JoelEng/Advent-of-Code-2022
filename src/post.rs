use dotenv::dotenv;
use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;
use std::process::{exit, Command};

const TOO_FAST: &str = "(You gave an answer too recently.*to wait.)";
const INCORRECT: &str = r"(That's not the right answer)";
const ALREADY_DONE: &str = r"(You don't seem to be solving.*\.)";
const CORRECT: &str = "(That's the right answer!)";

#[tokio::main]
async fn main() {
    dotenv().ok();
    let part_one: Regex = Regex::new(r"Part one: ([^\n]+)").unwrap();
    let part_two: Regex = Regex::new(r"Part two: ([^\n]+)").unwrap();
    let example_input: Regex = Regex::new(r"USING EXAMPLE INPUT").unwrap();

    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    let part = &args[2];
    let year = env::var("YEAR").unwrap();

    let day_num: i32 = day.parse().unwrap();
    if day_num < 1 || day_num > 25 {
        eprintln!("\x1b[41;30mIncorrect day. Should be between 1 and 25\x1b[0m");
        exit(1);
    }

    if !Path::new(&format!("src/bin/{}.rs", day)).exists() {
        eprintln!("\x1b[41;30mYou do not have a solution for this day\x1b[0m");
        exit(1);
    }

    let cmd = Command::new("cargo")
        .args(&["run", "--release", "--bin", &day])
        .output()
        .unwrap();
    let output = String::from_utf8(cmd.stdout).unwrap();

    if example_input.is_match(&output) {
        eprintln!("\x1b[41;30mTried to submit with example input\x1b[0m");
        exit(1);
    }

    let answer: String = match part.as_str() {
        "1" => part_one.captures(&output).unwrap()[1].to_string(),
        "2" => part_two.captures(&output).unwrap()[1].to_string(),
        _ => {
            eprintln!("\x1b[41;30mIncorrect puzzle part. Should be 1 or 2\x1b[0m");
            exit(1);
        }
    };

    let ansi_escape = Regex::new(r"\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])").unwrap();
    let answer = ansi_escape.replace_all(&answer, "").to_string();

    let form = [("answer", &answer), ("level", part)];
    println!(
        "\x1b[4;1mPosting {} to day {} part {} ({})\x1b[0m\n",
        answer, day, part, year
    );

    let short_day = day.parse::<i32>().unwrap().to_string();

    let html = post(year, short_day, form).await;

    for err in [TOO_FAST, INCORRECT, ALREADY_DONE] {
        let err_re = Regex::new(err).unwrap();
        if err_re.is_match(&html) {
            eprintln!(
                "\x1b[41;30m{}\x1b[0m",
                err_re.captures(&html).unwrap().get(1).unwrap().as_str()
            );
        }
    }

    let corr_re = Regex::new(CORRECT).unwrap();
    if corr_re.is_match(&html) {
        println!(
            "\x1b[102;30m{}\x1b[0m",
            corr_re.captures(&html).unwrap().get(1).unwrap().as_str()
        );
        if part == "1" {
            write_ans(&day, &answer, "one".to_string());
        } else {
            write_ans(&day, &answer, "two".to_string());
        }
    }
}

async fn post(year: String, short_day: String, form: [(&str, &String); 2]) -> String {
    let client = reqwest::Client::new();
    let res = client
        .post(format!(
            "https://adventofcode.com/{}/day/{}/answer",
            year, short_day
        ))
        .form(&form)
        .header(
            "Cookie",
            format!("session={}", env::var("AOC_SESSION").unwrap()),
        )
        .header("User-Agent", "Me")
        .send()
        .await
        .unwrap();

    res.text().await.unwrap()
}

fn write_ans(day: &String, answer: &String, part_string: String) {
    let ans_path = format!("answers/{}.sol", day);
    let ans_file = fs::read_to_string(&ans_path).unwrap();
    let re = Regex::new(&(format!("part {}: ", part_string).to_owned() + r"([^\n]*)")).unwrap();
    let new_ans_file = re
        .replace(&ans_file, format!("part {}: {}", part_string, answer))
        .to_string();
    let ansi_escape = Regex::new(r"\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])").unwrap();
    let new_ans_file = ansi_escape.replace_all(&new_ans_file, "").to_string();

    fs::write(&ans_path, new_ans_file).unwrap();
}
