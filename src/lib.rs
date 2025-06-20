// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    // todo!()
    let hex_clean = raw_tx_hex.trim();

    if hex_clean.len() < 8 {
        return Err("Transaction data too short".to_string());
    }

    // Take first 8 hex characters (4 bytes for version)
    let version_hex = &hex_clean[0..8];

    // Convert hex to bytes using hex crate
    let version_bytes = hex::decode(version_hex).map_err(|_| "Hex decode error".to_string())?;

    // Convert little-endian bytes to u32
    let version = u32::from_le_bytes([
        version_bytes[0],
        version_bytes[1],
        version_bytes[2],
        version_bytes[3],
    ]);

    Ok(version)
}
