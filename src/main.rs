use bitcoin::{
    absolute::LockTime,
    address::NetworkUnchecked,
    blockdata::transaction::{OutPoint, Transaction, TxIn, TxOut},
    script::ScriptBuf,
    sighash::{Prevouts, SighashCache, TapSighashType},
    transaction::{Sequence, Version},
    Amount, Network, Txid, Witness,
};
use hex::FromHex;

// use bitcoin::bitcoin_hashes::Hash;
fn main() {

    let txid_bytes = <[u8; 32]>::from_hex(
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef", //for e.g.
    )
    .unwrap();
    let txid_hash = Hash::from_byte_array(txid_bytes);
    
    let txid = Txid::from_raw_hash(txid_hash);

  
    let script_pubkey = ScriptBuf::from_hex(
        "5120aabbccddeeff00112233445566778899aabbccddeeff0011223344556677", //for e.g.
    )
    .unwrap();

    let prev_output = TxOut {
        value: Amount::from_sat(1000),
        script_pubkey,
    };

    let input = TxIn {
        previous_output: OutPoint {
            txid,
            vout: 0,
        },
        script_sig: ScriptBuf::new(),
        sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
        witness: Witness::default(),
    };

    let output = TxOut {
        value: Amount::from_sat(900),
        script_pubkey: ScriptBuf::new(), 
    };

    let mut tx = Transaction {
        version: Version(2),
        lock_time: LockTime::ZERO,
        input: vec![input],
        output: vec![output],
    };

    let prevouts = Prevouts::All(&[prev_output]);
    let sighash_type = TapSighashType::Default;

    let sighash = SighashCache::new(&mut tx)
        .taproot_key_spend_signature_hash(0, &prevouts, sighash_type)
        .expect("failed to create sighash");

    println!("Taproot Sighash: {}", sighash);
}
