use clap::{ArgAction, Parser};
use glob::glob;
use std::{
    io::Write,
    mem::swap,
    ops::Range,
    path::{Path, PathBuf},
    process::{self, exit},
};

#[derive(Parser)]
#[command(about)]
struct Args {
    /// Optional: defaults to latest
    year: Option<u32>,
    /// Optional: defaults to latest
    day: Option<u32>,
    /// Run with example input
    #[arg(short, long, action(ArgAction::SetTrue))]
    example: bool,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut args = Args::parse();

    // Swap year/day arguments if only one is provided and has a high value
    if args.day.is_none() && args.year < Some(100) {
        swap(&mut args.year, &mut args.day);
    }

    // Detect if we're running from project dir or root dir
    let exe_path = std::env::current_exe().unwrap();
    let mut root_dir = exe_path.parent().unwrap();
    if root_dir.file_name().unwrap() == "debug" || root_dir.file_name().unwrap() == "release" {
        root_dir = root_dir
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap();
    }

    if args.year.is_none() {
        args.year = find_highest_filename(root_dir.join("[0-9][0-9][0-9][0-9]"), 0..4);
    }

    let year = args.year.expect("YEAR argument is missing");

    let year_dir = root_dir.join(year.to_string());
    if !year_dir.exists() {
        eprintln!("Directory {} does not exist", year_dir.display());
        exit(1);
    }

    if args.day.is_none() {
        // Detect different pattern types, because there are python/rust/c++ solutions that require different director
        if year_dir.join("solutions").exists() {
            args.day =
                find_highest_filename(year_dir.join("solutions").join("day[0-9][0-9]*"), 3..5);
        } else {
            args.day = find_highest_filename(year_dir.join("day[0-9][0-9]*"), 3..5);
            if args.day.is_none() {
                args.day = find_highest_filename(year_dir.join("[0-9][0-9]*"), 0..2);
            }
        }
    }

    let day = args.day.expect("DAY argument is missing");

    // Create .input directory for this year
    let input_path = root_dir.join(".input").join(year.to_string());
    std::fs::create_dir_all(&input_path).expect("Failed to create .input directory");

    // Download and print description
    let desc_text = download_description(year, day, input_path.join(format!("{day:02}.html")))
        .await
        .unwrap();
    println!("--- Year {year} Day {day}: {} ---", parse_title(&desc_text));

    // Download input
    let input_file = input_path.join(format!("{day:02}.txt"));
    let input_bytes = download_input(year, day, root_dir, &input_file)
        .await
        .unwrap();

    // Spawn child process and pipe input
    let mut proc_args = vec![format!("{day:02}")];
    if args.example {
        proc_args.push(String::from("--example"));
    }
    let mut proc = process::Command::new(year_dir.join("aoc.bat"))
        .current_dir(year_dir)
        .args(proc_args)
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit())
        .spawn()
        .expect("Failed to launch child process");
    {
        let mut stdin = proc.stdin.take().expect("Failed to open stdin");
        stdin
            .write_all(&input_bytes)
            .expect("Failed to write to stdin");
    }
    proc.wait().unwrap();
}

fn parse_title(desc_text: &str) -> String {
    // Title can be found between <h2>--- ?? ---</h2>
    let title_begin = desc_text.find("<h2>").expect("Missing <h2> tag") + 4;
    let title_begin = desc_text[title_begin..].find(": ").unwrap() + title_begin + 2;
    let title_end = desc_text[title_begin..]
        .find(" ---")
        .expect("Missing ' ---' in title")
        + title_begin;

    let mut title = String::new();
    html_escape::decode_html_entities_to_string(&desc_text[title_begin..title_end], &mut title);
    title
}

// TODO: Might use this in the future while generating template code
#[allow(dead_code)]
fn parse_example(desc_text: &str) -> String {
    // Find <pre><code> with preference of it appearing after "example:" text
    let example_begin = desc_text.find("example:").unwrap_or(0);
    let example_begin = desc_text[example_begin..]
        .find("<pre><code>")
        .expect("Missing <pre><code> tag")
        + example_begin
        + 11;
    let example_end = desc_text[example_begin..]
        .find("</code></pre>")
        .expect("Missing </code></pre> tag")
        + example_begin;

    let mut example = String::new();
    html_escape::decode_html_entities_to_string(
        &desc_text[example_begin..example_end],
        &mut example,
    );

    // Some examples have <em> tags that we need to remove
    if example.contains("<em>") {
        example = example.replace("<em>", "").replace("</em>", "");
    }
    example
}

async fn download_description(
    year: u32,
    day: u32,
    dest: PathBuf,
) -> Result<String, Box<dyn std::error::Error>> {
    let text;
    if dest.exists() {
        text = std::fs::read_to_string(dest)?;
    } else {
        println!("Downloading description...");
        text = reqwest::get(format!("https://adventofcode.com/{year}/day/{day}"))
            .await?
            .error_for_status()
            .unwrap()
            .text()
            .await?;
        std::fs::write(dest, &text)?;
    }
    Ok(text)
}

async fn download_input(
    year: u32,
    day: u32,
    root_dir: &Path,
    dest: &PathBuf,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    if dest.exists() {
        let input_bytes = std::fs::read(dest)?;
        return Ok(input_bytes);
    }

    println!("Downloading input...");

    let env_file = root_dir.join(".env");
    if !env_file.exists() {
        eprintln!(
            "Missing environment file: {}.\nThis is required to download your input.",
            env_file.display()
        );
        exit(1);
    }
    let env = std::fs::read_to_string(env_file)?;
    let session_cookie = env
        .trim()
        .strip_prefix("session=")
        .expect(".env file should contain your session cookie: \"session=XXXX\"");

    let input_bytes = reqwest::Client::new()
        .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
        .header(
            reqwest::header::COOKIE,
            &format!("session={session_cookie}"),
        )
        .send()
        .await?
        .bytes()
        .await?;

    std::fs::write(dest, &input_bytes)?;
    Ok(input_bytes.into())
}

fn find_highest_filename(pattern: PathBuf, pattern_range: Range<usize>) -> Option<u32> {
    let mut result: Option<u32> = None;
    for path in glob(pattern.to_str().unwrap()).unwrap() {
        let path = path.unwrap();
        let filename = path.file_name().unwrap().to_str().unwrap();
        let filename_u32 = filename[pattern_range.clone()].parse::<u32>().ok();
        if filename_u32.is_some() {
            if result.is_none() || result < filename_u32 {
                result = filename_u32;
            }
        }
    }
    result
}
