// @generated automatically by Diesel CLI.

diesel::table! {
    nft (id) {
        id -> Integer,
        sender -> Text,
        receiver -> Text,
        gas_price -> Integer,
        tx_value -> Integer,
        tx_time -> Text,
        block_num -> Integer,
    }
}
