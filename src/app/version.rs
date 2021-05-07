const MAJOR: &str = "0";
const MINOR: &str = "1";
const FIX: &str = "1";
const VERBAL: &str = "TX Add && Balances List";

pub fn version_cmd() {
    println!("Version: {}.{}.{}-beta {}", MAJOR, MINOR, FIX, VERBAL);
}
