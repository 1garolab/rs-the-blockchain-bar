const FLAG_FROM: &str = "from";
const FLAG_TO: &str = "to";
const FLAG_VALUE: &str = "value";
const FLAG_DATA: &str = "data";

// #[path = "../database/tx_db.rs"]
// mod tx_db;
// use tx_db::Tx;
use crate::database::tx_db;

// pub fn tx_cmd(transaction: Tx) {
pub fn tx_cmd(new_tx: tx_db::Tx) {
    let _tx = tx_db::Tx::new(new_tx.from, new_tx.to, new_tx.value, new_tx.data);
    // TODO: missing (new_state_from_disk - add - persist) implementations;
    unimplemented!()
}
