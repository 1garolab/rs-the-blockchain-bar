mod app;
mod database;

use app::{balances::balances_cmd, tx::tx_cmd, version::version_cmd};
use database::tx_db;

use std::env;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
}

fn main() {
    let args = Cli::from_args();
    let pattern = &args.pattern;
    let path = env::current_dir().unwrap();
    match &pattern[..] {
        "version" => version_cmd(),
        "balances" => balances_cmd(path),
        "tx" => tx_cmd(tx_db::Tx::from_args(), path),
        _ => unimplemented!(),
    };

    // let ctx = read_to_string(&args.path);
    // ctx.iter().map(|line| println!("{}", line)).collect()
}
