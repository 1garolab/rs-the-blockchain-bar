use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Tx {
    #[structopt(short = "f", long = "from")]
    pub from: String,
    #[structopt(short = "t", long = "to")]
    pub to: String,
    #[structopt(short = "v", long = "value")]
    pub value: u64,
    #[structopt(short = "d", long = "data")]
    pub data: String,
}

impl Tx {
    pub fn new(from: String, to: String, value: u64, data: String) -> Tx {
        Tx {
            from,
            to,
            value,
            data,
        }
    }

    pub fn is_reward(&self) -> bool {
        self.data == "reward"
    }
}
