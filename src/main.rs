//**************************************************for key path sighash***************************************************
// use bitcoin::{
//     absolute::LockTime,
//     address::NetworkUnchecked,
//     blockdata::transaction::{OutPoint, Transaction, TxIn, TxOut},
//     script::ScriptBuf,
//     sighash::{Prevouts, SighashCache, TapSighashType},
//     transaction::{Sequence, Version},
//     Amount, Network, Txid, Witness,
// };
// use bitcoin::consensus::encode::deserialize;
// use bitcoin::sighash::ScriptPath;


// use hex::decode;

// use hex::FromHex;


// fn main() {

   
//     let raw_tx_hex = "02000000014eeb7466e814a86fbadd776b027ff66d5452ca6cb35b617ccb9f988297ba89520000000000fdffffff01905f010000000000160014eb8ad234e24b89225c1f75e2abc8c01d3523e95500000000";

//     // Deserialize
//     let tx_bytes = decode(raw_tx_hex).expect("invalid hex");
//     let transaction: Transaction = deserialize(&tx_bytes).expect("failed to deserialize tx");
    
//     let script_pubkey = ScriptBuf::from_hex(
//         "5120aabbccddeeff00112233445566778899aabbccddeeff0011223344556677", //for e.g.
//     )
//     .unwrap();

//     let prev_output = TxOut {
//         value: Amount::from_sat(1000),
//         script_pubkey,
//     };

//     // let input = TxIn {
//     //     previous_output: OutPoint {
//     //         txid,
//     //         vout: 0,
//     //     },
//     //     script_sig: ScriptBuf::new(),
//     //     sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
//     //     witness: Witness::default(),
//     // };

//     let output = TxOut {
//         value: Amount::from_sat(900),
//         script_pubkey: ScriptBuf::new(), 
//     };

//     // let mut tx = Transaction {
//     //     version: Version(2),
//     //     lock_time: LockTime::ZERO,
//     //     input: vec![input],
//     //     output: vec![output],
//     // };
//     let binding = [prev_output];
//     let prevouts = Prevouts::All(&binding);
//     let sighash_type = TapSighashType::Default;

//     let sighash = SighashCache::new(&transaction)
//         .taproot_key_spend_signature_hash(0, &prevouts, sighash_type)
//         .expect("failed to create sighash");

//     println!("Taproot Sighash: {}", sighash);
// }

//**************************************************for script path sighash***************************************************
use bitcoin::consensus::encode::deserialize;
use bitcoin::sighash::{Prevouts, SighashCache, TapSighashType, ScriptPath};
use bitcoin::{Transaction, TxOut, Amount, ScriptBuf};
use bitcoin::taproot::ControlBlock;
use hex::decode;
use bitcoin::TapLeafHash;
use bitcoin::taproot::LeafVersion;
fn main() {

    let raw_tx_hex = "02000000014eeb7466e814a86fbadd776b027ff66d5452ca6cb35b617ccb9f988297ba89520000000000fdffffff01905f010000000000160014eb8ad234e24b89225c1f75e2abc8c01d3523e95500000000";


    let tx_bytes = decode(raw_tx_hex).expect("Invalid hex");
    let transaction: Transaction = deserialize(&tx_bytes).expect("Deserialization failed");

  
    let script = ScriptBuf::from(decode("2025f1a245ff572ac11fc1e5da5f6a5a93c946f17c20f1c317c5bae2a0ef2d821cad20d2c1cb1575d323b6120b6e5bcc9ce5ad373e88e73e675030f1c2c5261b4dbc86ac").expect("Invalid script hex")); // Replace with actual script hex


    // let control_block_bytes = decode("c15bf08d58a430f8c222bffaf9127249c5cdff70a2d68b2b45637eb662b6b88eb5747f67099a8ea09f5d9f590c1d12f38f58838872d081646f4e252c90f0ac86d3d29ab618193c0908c50339f77cce4b89935f4df11ed45f3bdff6f7395edd59fb").expect("Invalid control block"); // Replace with valid control block
    // let control_block = ControlBlock::decode(&control_block_bytes).expect("Invalid control block format");

    let script_path = ScriptPath::new(&script, LeafVersion::TapScript);

    let prev_output = TxOut {
        value: Amount::from_sat(100_000),
        script_pubkey: ScriptBuf::from(decode("51201d8e516e4dc5f094cd9ba04ce7ba847e1be7b4e9e4b9dda76a5f567832401860").expect("Invalid script_pubkey hex")), // Same Taproot scriptPubKey
    };
    let binding = [prev_output];
    let prevouts = Prevouts::All(&binding);

    let sighash_type = TapSighashType::Default;
   
    let leaf_hash = TapLeafHash::from(script_path.clone());
let sighash = SighashCache::new(&transaction)
    .taproot_script_spend_signature_hash(
        0,
        &prevouts,
        leaf_hash,
        sighash_type,
    )
    .expect("Failed to generate sighash");

    println!("Taproot Script Path Sighash: {}", sighash);
}

//*************************************************************************************************************