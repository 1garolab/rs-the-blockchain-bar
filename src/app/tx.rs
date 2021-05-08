use crate::database::{genesis, state, tx_db};
const FLAG_FROM: &str = "from";
const FLAG_TO: &str = "to";
const FLAG_VALUE: &str = "value";
const FLAG_DATA: &str = "data";

pub fn tx_cmd(new_tx: tx_db::Tx, cwd: std::path::PathBuf) {
    let _tx = tx_db::Tx::new(new_tx.from, new_tx.to, new_tx.value, new_tx.data);
    // let _gen = genesis::load_genesis(cwd.join(GENESIS_FPATH));
    let _tx_db = genesis::load_tx_db(&cwd);
    // TODO: missing (new_state_from_disk - add - persist) implementations;
    unimplemented!()
}
