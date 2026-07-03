//! Deterministic text normalization for VP-RULE-0011 (VP-RFC-0011).

use unicode_general_category::{get_general_category, GeneralCategory};
use unicode_normalization::UnicodeNormalization;

/// Normalization failed — invalid UTF-8 or protocol preconditions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalizationError {
    InvalidUtf8,
}

/// Returns true when `c` is removed during trim or collapsed during step 3.
pub fn is_normalization_whitespace(c: char) -> bool {
    matches!(c, '\t' | '\n' | '\r' | '\x0b' | '\x0c')
        || matches!(
            get_general_category(c),
            GeneralCategory::SpaceSeparator
                | GeneralCategory::LineSeparator
                | GeneralCategory::ParagraphSeparator
        )
}

/// Validates UTF-8 bytes and applies the Platform 1.3 normalization pipeline.
pub fn try_normalize_text_bytes(bytes: &[u8]) -> Result<String, NormalizationError> {
    let text = std::str::from_utf8(bytes).map_err(|_| NormalizationError::InvalidUtf8)?;
    Ok(normalize_text(text))
}

/// Applies NFC → trim → internal whitespace collapse on valid UTF-8 text.
pub fn normalize_text(input: &str) -> String {
    let nfc: String = input.chars().nfc().collect();
    let trimmed = trim_normalization_whitespace(&nfc);
    collapse_internal_whitespace(trimmed)
}

fn trim_normalization_whitespace(input: &str) -> &str {
    input.trim_matches(is_normalization_whitespace)
}

fn collapse_internal_whitespace(input: &str) -> String {
    let mut result = String::new();
    let mut pending_space = false;

    for c in input.chars() {
        if is_normalization_whitespace(c) {
            if !result.is_empty() {
                pending_space = true;
            }
        } else {
            if pending_space {
                result.push(' ');
                pending_space = false;
            }
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_utf8_bytes_are_rejected() {
        assert_eq!(
            try_normalize_text_bytes(&[0xFF, 0xFE]),
            Err(NormalizationError::InvalidUtf8)
        );
    }

    #[test]
    fn nfc_composes_equivalent_forms() {
        let nfc = "café";
        let nfd = "cafe\u{0301}";
        assert_eq!(normalize_text(nfc), normalize_text(nfd));
    }

    #[test]
    fn whitespace_only_collapses_to_empty() {
        assert_eq!(normalize_text("   \t\n  "), "");
    }
}
