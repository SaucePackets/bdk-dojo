use crate::utxo::Utxo;

pub fn confirmations(utxo: &Utxo, tip_height: u32) -> u32 {
    match utxo.seen_at_height {
        Some(height) => tip_height - height + 1,
        None => 0,
    }
}
