use crate::utxo::{self, Utxo};

pub const COINBASE_MATURITY: u32 = 100;

pub fn is_spendable(utxo: &Utxo, tip_height: u32) -> bool {
    if !utxo.owned {
        return false;
    }

    if let Some(height) = utxo.locked_until {
        if tip_height < height {
            return false;
        }
    }

    if utxo.coinbase && confirmations(utxo, tip_height) < COINBASE_MATURITY {
        return false;
    }

    true
}

pub fn confirmations(utxo: &Utxo, tip_height: u32) -> u32 {
    match utxo.seen_at_height {
        Some(height) if height <= tip_height => tip_height - height + 1,
        _ => 0,
    }
}
