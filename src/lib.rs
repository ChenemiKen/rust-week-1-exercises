// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    let transaction_bytes_res = hex::decode(raw_tx_hex);
    if transaction_bytes_res.is_err() {return Err(String::from("Hex decode error"));}
    let transaction_bytes = transaction_bytes_res.unwrap();
    if transaction_bytes.len() < 4 {return Err(String::from("Transaction data too short"))}
    let version_bytes = (&transaction_bytes[0..4]).try_into().unwrap();
    let version = u32::from_le_bytes(version_bytes);
    Ok(version)
}
