use crate::database;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, Read, Result};
use std::path::PathBuf;

use crate::database::tx_db;
use std::path::Path;

const GENESIS_FPATH: &str = "src/database/genesis.json";
const TX_FPATH: &str = "src/database/tx.json";

pub fn load_genesis(path: &PathBuf) {
    // println!("{:#?}", u);
    unimplemented!()
}

pub fn load_tx_db(cwd: &PathBuf) -> database::tx_db::TxVec {
    tx_db::Tx::read_tx_from_file(&cwd.join(TX_FPATH)).unwrap()
}
