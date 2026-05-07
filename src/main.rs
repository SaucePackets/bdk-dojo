use bdk_dojo::{calculate_balance, Utxo};

fn main() {
    let utxos = vec![
        Utxo {
            value: 50_000,
            confirmed: true,
            spendable: true,
        },
        Utxo {
            value: 20_000,
            confirmed: false,
            spendable: true,
        },
        Utxo {
            value: 10_000,
            confirmed: false,
            spendable: false,
        },
    ];

    let summary = calculate_balance(&utxos);

    println!("{summary:?}");
}
