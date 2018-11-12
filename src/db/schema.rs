table! {
    blocks (hash) {
        id -> Int8,
        height -> Int8,
        hash -> Bpchar,
        prev_hash -> Bpchar,
    }
}

table! {
    inputs (utxo_tx_hash, utxo_tx_idx) {
        id -> Int8,
        height -> Int8,
        utxo_tx_hash -> Bpchar,
        utxo_tx_idx -> Int4,
    }
}

table! {
    outputs (tx_hash, tx_idx) {
        id -> Int8,
        height -> Int8,
        tx_hash -> Bpchar,
        tx_idx -> Int4,
        value -> Int8,
        address -> Nullable<Text>,
        coinbase -> Bool,
    }
}

table! {
    txs (hash) {
        id -> Int8,
        height -> Int8,
        hash -> Bpchar,
        coinbase -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    blocks,
    inputs,
    outputs,
    txs,
);
