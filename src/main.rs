mod app;

use app::version::version_cmd;

use std::fs::read_to_string;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let pattern = &args.pattern;

    match &pattern[..] {
        "version" => version_cmd(),
        _ => (),
    };

    let ctx = read_to_string(&args.path);
    ctx.iter().map(|line| println!("{}", line)).collect()
}
