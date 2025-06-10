use bitcoin::{
    absolute::LockTime,
    address::NetworkUnchecked,
    blockdata::transaction::{OutPoint, Transaction, TxIn, TxOut},
    script::ScriptBuf,
    sighash::{Prevouts, SighashCache, TapSighashType},
    transaction::{Sequence, Version},
    Amount, Network, Txid, Witness,
};
use bitcoin::consensus::encode::deserialize;
use bitcoin::sighash::ScriptPath;
use bitcoin::taproot::{ControlBlock}; // only this from taproot


use hex::decode;

use hex::FromHex;


fn main() {

   
    let raw_tx_hex = "02000000014eeb7466e814a86fbadd776b027ff66d5452ca6cb35b617ccb9f988297ba89520000000000fdffffff01905f010000000000160014eb8ad234e24b89225c1f75e2abc8c01d3523e95500000000";

    // Deserialize
    let tx_bytes = decode(raw_tx_hex).expect("invalid hex");
    let transaction: Transaction = deserialize(&tx_bytes).expect("failed to deserialize tx");
    
    let script_pubkey = ScriptBuf::from_hex(
        "5120aabbccddeeff00112233445566778899aabbccddeeff0011223344556677", //for e.g.
    )
    .unwrap();

    let prev_output = TxOut {
        value: Amount::from_sat(1000),
        script_pubkey,
    };

    // let input = TxIn {
    //     previous_output: OutPoint {
    //         txid,
    //         vout: 0,
    //     },
    //     script_sig: ScriptBuf::new(),
    //     sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
    //     witness: Witness::default(),
    // };

    let output = TxOut {
        value: Amount::from_sat(900),
        script_pubkey: ScriptBuf::new(), 
    };

    // let mut tx = Transaction {
    //     version: Version(2),
    //     lock_time: LockTime::ZERO,
    //     input: vec![input],
    //     output: vec![output],
    // };
    let binding = [prev_output];
    let prevouts = Prevouts::All(&binding);
    let sighash_type = TapSighashType::Default;

    let sighash = SighashCache::new(&transaction)
        .taproot_key_spend_signature_hash(0, &prevouts, sighash_type)
        .expect("failed to create sighash");

    println!("Taproot Sighash: {}", sighash);
}