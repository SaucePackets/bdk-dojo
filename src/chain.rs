use crate::utxo::Utxo;

pub const COINBASE_MATURITY: u32 = 100;

pub fn is_spendable(utxo: &Utxo, tip_height: u32) -> bool {
    todo!("combine ownership, lock height, and coinbase maturity")
}

pub fn confirmations(utxo: &Utxo, tip_height: u32) -> u32 {
    match utxo.seen_at_height {
        Some(height) if height <= tip_height => tip_height - height + 1,
        _ => 0,
    }
}
