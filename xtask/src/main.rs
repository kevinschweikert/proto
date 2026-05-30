use anyhow::{Context, Result};
use std::fmt::Write;
use std::{fs, process::Command};

const START_MARKER: &'static str = "<!-- BEGIN CLI:";
const CLOSE_MARKER: &'static str = "-->";
const END_MARKER: &'static str = "<!-- END CLI -->";

const README_PATH: &'static str = "README.md";

fn main() -> Result<()> {
    let readme = fs::read_to_string(README_PATH)?;
    let updated = repace_blocks(&readme)?;

    fs::write(README_PATH, &updated)?;
    Ok(())
}

fn repace_blocks(content: &str) -> Result<String> {
    let mut result = String::new();
    let mut lines = content.lines().peekable();
    while let Some(line) = lines.next() {
        if let Some(args) = parse_start(line) {
            writeln!(result, "{line}")?;
            writeln!(result, "<!-- AUTO-GENERATED: run `cargo -p xtask` -->")?;
            writeln!(result)?;

            let output = run_command(&args)?;

            writeln!(result, "```bash")?;
            writeln!(result, "proto {}", &args)?;
            writeln!(result, "```")?;
            writeln!(result)?;
            writeln!(result, "Output:")?;
            writeln!(result)?;

            writeln!(result, "```")?;
            writeln!(result, "{}", &output)?;
            writeln!(result, "```")?;
            writeln!(result)?;

            while let Some(next) = lines.next() {
                if next.trim() == END_MARKER {
                    writeln!(result, "{}", END_MARKER)?;
                    break;
                }
            }
        } else {
            writeln!(result, "{line}")?;
        }
    }

    Ok(result)
}

fn run_command(args: &str) -> Result<String> {
    let parsed_args: Vec<String> = shlex::split(args).context("invalid args")?;

    let output = Command::new("cargo")
        .args(["run", "--quiet", "-p", "proto", "--"])
        .args(&parsed_args)
        .env("NO_COLOR", "1")
        .env("COLUMNS", "80")
        .output()
        .with_context(|| format!("failed command for '{args}'"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("command failed for '{args}'\n{stderr}")
    }

    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

fn parse_start(line: &str) -> Option<String> {
    let line = line.trim();

    if line.starts_with(START_MARKER) && line.ends_with(CLOSE_MARKER) {
        let inner = &line[START_MARKER.len()..line.len() - CLOSE_MARKER.len()];
        Some(inner.trim().to_string())
    } else {
        None
    }
}
