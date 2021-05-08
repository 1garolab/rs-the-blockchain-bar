use structopt::StructOpt;

use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct TxVec {
    tx: Vec<Tx>,
}

#[derive(Deserialize, Debug, StructOpt)]
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
    pub fn read_tx_from_file<P: AsRef<std::path::Path>>(path: &P) -> std::io::Result<TxVec> {
        println!("opening -> {:?}", path.as_ref());
        let file = std::fs::File::open(path).unwrap();
        println!("reading -> {:?}", path.as_ref().file_name().unwrap());
        let reader = std::io::BufReader::new(file);

        let mut de = serde_json::Deserializer::from_reader(reader);
        let u = TxVec::deserialize(&mut de).unwrap();
        Ok(u)
    }

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
