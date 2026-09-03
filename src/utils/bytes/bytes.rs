pub fn to_hex(data: &[u8]) -> String {
    data.iter().map(|b| format!("{:02x}", b)).collect()
}

pub fn to_hex_upper(data: &[u8]) -> String {
    data.iter().map(|b| format!("{:02X}", b)).collect()
}

pub fn from_hex(hex: &str) -> Result<Vec<u8>, String> {
    if hex.len() % 2 != 0 {
        return Err("Hex string must have even length".to_string());
    }
    (0..hex.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex[i..i + 2], 16)
                .map_err(|e| format!("Invalid hex character: {}", e))
        })
        .collect()
}

pub fn to_base64(data: &[u8]) -> String {
    use base64::Engine as _;
    base64::engine::general_purpose::STANDARD.encode(data)
}

pub fn from_base64(encoded: &str) -> Result<Vec<u8>, String> {
    use base64::Engine as _;
    base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|e| e.to_string())
}

pub fn concat_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    result.extend_from_slice(a);
    result.extend_from_slice(b);
    result
}

pub fn split_at_bytes(data: &[u8], index: usize) -> (&[u8], &[u8]) {
    data.split_at(index)
}

pub fn take_first_n(data: &[u8], n: usize) -> &[u8] {
    data.get(..n).unwrap_or(data)
}

pub fn skip_first_n(data: &[u8], n: usize) -> &[u8] {
    data.get(n..).unwrap_or(&[])
}

pub fn is_empty(data: &[u8]) -> bool {
    data.is_empty()
}

pub fn len(data: &[u8]) -> usize {
    data.len()
}

pub fn to_string_lossy(data: &[u8]) -> String {
    String::from_utf8_lossy(data).to_string()
}

pub fn to_string(data: &[u8]) -> Result<String, std::string::FromUtf8Error> {
    String::from_utf8(data.to_vec())
}
