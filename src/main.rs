use kcwrap::*;

use std::{
    env::args,
    io::{self, Write},
    os::unix::process::parent_id,
    process::{self, Command, Output},
};

use chrono::{Duration, Utc};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::{
    config::{Config, init_config_from_env},
    session::{SessionState, load_sessions_state, update_session},
};

static SUPPORTED_COMMANDS: [&str; 5] = ["kubectl", "flux", "helm", "kubeadm", "istioctl"];

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        print_instructions();
        process::exit(1);
    }

    let cli_tool = &args[1];

    if !SUPPORTED_COMMANDS.contains(&cli_tool.as_str()) {
        println!(
            "Unsupported tool: {cli_tool}. Please use {:?}",
            format_supported_commands()
        );
        process::exit(1);
    }

    pass_through_completion_call(&cli_tool, &args);

    force_skip_confirmation_if_requested(&cli_tool, &args);

    let config: Config = init_config_from_env();

    let context_cmd = Command::new("kubectl")
        .arg("config")
        .arg("current-context")
        .output()
        .expect("Failed to read command output");

    let (success, output) = get_output_from_command(context_cmd);

    if !success {
        println!("{output}");
        process::exit(1);
    }

    let current_context = output;

    let current_session = parent_id();
    let mut session_state: SessionState = load_sessions_state(current_session);

    print_context(&current_context, &config).expect("Failed to print context to stdout");

    let should_prompt: bool = should_prompt(&session_state, &current_context);

    if !should_prompt {
        run_command_and_exit(cli_tool, &args);
    }

    let should_execute: bool = prompt_if_should_execute(cli_tool);

    if should_execute {
        update_session(
            &mut session_state,
            current_context.to_string(),
            current_session,
        );
        run_command_and_exit(cli_tool, &args);
    } else {
        println!("Command aborted");
        process::exit(0);
    }
}

fn get_output_from_command(output: Output) -> (bool, String) {
    if !output.status.success() {
        return (false, String::from_utf8_lossy(&output.stderr).into_owned());
    }

    (true, String::from_utf8_lossy(&output.stdout).into_owned())
}

fn prompt_if_should_execute(cli_tool: &str) -> bool {
    let args: Vec<String> = args().collect();
    let args_string = &args[2..].join(" ");

    println!("Command: {cli_tool} {args_string}");
    println!("Do you want to continue? [y/N]: ");

    let mut answer = String::new();

    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read user input");

    answer = answer.trim().to_lowercase();

    answer == "y" || answer == "yes"
}

fn run_command_and_exit(cli_tool: &str, args: &[String]) {
    let status = Command::new(cli_tool)
        .args(&args[2..])
        .status()
        .expect("Failed to execute command {cli_tool}");

    process::exit(status.code().unwrap_or(1))
}

fn should_prompt(session_state: &SessionState, current_context: &str) -> bool {
    let now = Utc::now();
    let duration_since = now.signed_duration_since(session_state.last_confirmed_date_time);
    if duration_since > Duration::minutes(10) {
        return true;
    }

    match session_state.confirmed_contexts.get(current_context) {
        Some(has_confirmed) => !*has_confirmed,
        None => true,
    }
}

fn get_color(current_context: &str, config: &Config) -> ColorSpec {
    let lower = current_context.trim().to_lowercase();
    let mut color = ColorSpec::new();

    match lower {
        s if contains_list_item(&s, &config.prod_name) => color.set_fg(Some(Color::Red)),
        s if contains_list_item(&s, &config.test_name) => color.set_fg(Some(Color::Yellow)),
        s if contains_list_item(&s, &config.dev_name) => color.set_fg(Some(Color::Green)),
        _ => color.set_fg(Some(Color::Blue)), // default to blue
    };

    color
}

fn force_skip_confirmation_if_requested(cli_tool: &str, args: &[String]) {
    if args.len() < 3 {
        return;
    }

    let should_skip = args[2] == "kcw_no_wrap";

    if should_skip {
        let status = Command::new(cli_tool)
        .args(&args[3..]) // Skip "kcwrap", "kubectl", and "kcw_no_wrap"
        .status()
        .expect("Failed to execute command");

        process::exit(status.code().unwrap_or(1));
    }
}

fn pass_through_completion_call(cli_tool: &str, args: &[String]) {
    // Check internal completion arguments
    let command_args = &args[2..];

    // kubectl uses __complete for shell completion
    let is_completion_call = command_args
        .iter()
        .any(|arg| arg.contains("__complete") || arg.contains("completion"));

    if is_completion_call {
        run_command_and_exit(cli_tool, args);
    }
}

fn contains_list_item(str: &str, strings: &[String]) -> bool {
    strings.iter().any(|s| str.contains(s))
}

fn print_context(current_context: &str, config: &Config) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(&get_color(current_context, config))?;
    write!(&mut stdout, "Current context: {current_context}")?;
    // Reset color
    stdout.reset()?;
    stdout.flush()
}

fn print_instructions() {
    println!(
        "Usage: kcwrap {:?} [arguments...]",
        format_supported_commands()
    );
    println!("Override enivronment names (dev/test/prod) by setting the env variables");
    println!("KCWRAP_DEV1, KCWRAP_TEST1, KCWRAP_PROD1");
    println!("e.g. export KCWRAP_TEST1='test'");
    println!();
    println!("For multiple environments, use KCWRAP_DEV1, KCWRAP_DEV2, etc");
}

fn format_supported_commands() -> String {
    SUPPORTED_COMMANDS.join("|")
}
