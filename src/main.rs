use std::error::Error;
use std::process::Command;
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_args();
    set_active_input(&config.input)?;
    Ok(())
}

#[cfg(windows)]
fn set_active_input(input: &DisplayInput) -> Result<(), Box<dyn Error>> {
    let ddm_path = r#"C:\Program Files (x86)\Dell\Dell Display Manager\ddm.exe"#;

    let output = Command::new("cmd")
        .args(&[
            "/C",
            &ddm_path,
            "1:SetActiveInput",
            input.ddm_command(),
            "/Exit",
        ])
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        Err("Failed to switch monitor active input".into())
    }
}

#[cfg(unix)]
fn set_active_input(input: &DisplayInput) -> Result<(), Box<dyn Error>> {
    let output = Command::new("ddcctl")
        .args(&["-d", "1", "-i", input.ddcctl_command()])
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        Err("Failed to switch monitor active input".into())
    }
}

enum DisplayInput {
    DP,
    MDP,
}

impl DisplayInput {
    #[cfg(windows)]
    fn ddm_command(&self) -> &str {
        match self {
            DisplayInput::DP => "DP",
            DisplayInput::MDP => "mDP",
        }
    }

    #[cfg(unix)]
    fn ddcctl_command(&self) -> &str {
        match self {
            DisplayInput::DP => "15",
            DisplayInput::MDP => "16",
        }
    }
}

#[derive(StructOpt)]
struct Config {
    #[structopt(parse(try_from_str = "parse_input"))]
    input: DisplayInput,
}

fn parse_input(src: &str) -> Result<DisplayInput, Box<dyn Error>> {
    match src {
        "DP" => Ok(DisplayInput::DP),
        "mDP" => Ok(DisplayInput::MDP),
        _ => Err("Input must be either DP or mDP".into()),
    }
}
