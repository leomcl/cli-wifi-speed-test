use clap::Parser;

/// Wifi speed test cli
/// test comment test comment (ment to be in the help message)
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    /// check down
    #[arg(short, long)]
    down: bool,

    /// check up
    #[arg(short, long)]
    up: bool,
}

fn main() {
    let args = CliArgs::parse();

    let run_downlaod = args.down || (!args.down && !args.up);
    let run_upload = args.up;

    if let Err(e) = run_logic(run_downlaod, run_upload) {
        eprintln!("Error: {}", e);
    }
}

fn run_logic(do_down: bool, do_up: bool) -> Result<(), String> {
    if do_down {
        println!("Checking download speed...");
    }
    if do_up {
        println!("Checking upload speed...");
    }
    Ok(())
}
