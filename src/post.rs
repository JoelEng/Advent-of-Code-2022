use dotenv::dotenv;
use regex::Regex;
use std::env;
use std::process::Command;

const TOO_FAST: &str = "(You gave an answer too recently.*to wait.)";
const INCORRECT: &str = r"(That's not the right answer)";
const ALREADY_DONE: &str = r"(You don't seem to be solving.*\.)";
const CORRECT: &str = "(That's the right answer!)";

#[tokio::main]
async fn main() {
    dotenv().ok();
    let part_one: Regex = Regex::new(r"Part one: ([^\n]+)").unwrap();
    let part_two: Regex = Regex::new(r"Part two: ([^\n]+)").unwrap();

    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    let part = &args[2];
    let year = env::var("YEAR").unwrap();

    let cmd = Command::new("cargo")
        .args(&["run", "--release", "--bin", &day])
        .output()
        .unwrap();
    let output = String::from_utf8(cmd.stdout).unwrap();

    let answer: String = match part.as_str() {
        "1" => part_one.captures(&output).unwrap()[1].to_string(),
        "2" => part_two.captures(&output).unwrap()[1].to_string(),
        _ => panic!("Incorrect puzzle part"),
    };

    let form = [("answer", &answer), ("level", part)];
    println!(
        "\x1b[4;1mPosting {} to day {} part {} ({})\x1b[0m",
        answer, day, part, year
    );

    let short_day = day.parse::<i32>().unwrap().to_string();

    let html = post(year, short_day, form).await;

    for err in [TOO_FAST, INCORRECT, ALREADY_DONE] {
        let err_re = Regex::new(err).unwrap();
        if err_re.is_match(&html) {
            println!(
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
