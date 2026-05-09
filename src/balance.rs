use crate::utxo::Utxo;

pub fn calculate_balance(utxos: &[Utxo]) -> u64 {
    let mut total_balance = 0;
    for utxo in utxos {
        total_balance += utxo.value.to_sats();
    }

    total_balance
}
