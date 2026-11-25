/// Decodes the docs file, which ships as UTF-16 LE with a BOM in most game versions.
pub fn decode_text(bytes: &[u8]) -> Result<String, String> {
    match bytes {
        [0xFF, 0xFE, rest @ ..] => decode_utf16(rest, u16::from_le_bytes),
        [0xFE, 0xFF, rest @ ..] => decode_utf16(rest, u16::from_be_bytes),
        [0xEF, 0xBB, 0xBF, rest @ ..] => utf8(rest),
        _ => utf8(bytes),
    }
}

fn utf8(bytes: &[u8]) -> Result<String, String> {
    String::from_utf8(bytes.to_vec()).map_err(|e| format!("File is not valid UTF-8: {e}"))
}

fn decode_utf16(bytes: &[u8], to_unit: fn([u8; 2]) -> u16) -> Result<String, String> {
    if !bytes.len().is_multiple_of(2) {
        return Err("UTF-16 file has an odd number of bytes".into());
    }
    let units: Vec<u16> = bytes.chunks_exact(2).map(|c| to_unit([c[0], c[1]])).collect();
    String::from_utf16(&units).map_err(|e| format!("File is not valid UTF-16: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn utf16le_with_bom(text: &str) -> Vec<u8> {
        let mut bytes = vec![0xFF, 0xFE];
        bytes.extend(text.encode_utf16().flat_map(u16::to_le_bytes));
        bytes
    }

    #[test]
    fn decode_text_utf16le_with_bom_returns_text() {
        assert_eq!(decode_text(&utf16le_with_bom("[{\"a\":\"ñ\"}]")).unwrap(), "[{\"a\":\"ñ\"}]");
    }

    #[test]
    fn decode_text_utf8_with_bom_strips_bom() {
        assert_eq!(decode_text(b"\xEF\xBB\xBF[1]").unwrap(), "[1]");
    }

    #[test]
    fn decode_text_plain_utf8_returns_text() {
        assert_eq!(decode_text(b"[2]").unwrap(), "[2]");
    }

    #[test]
    fn decode_text_odd_utf16_length_is_an_error() {
        assert_eq!(
            decode_text(&[0xFF, 0xFE, 0x41]).unwrap_err(),
            "UTF-16 file has an odd number of bytes"
        );
    }
}
