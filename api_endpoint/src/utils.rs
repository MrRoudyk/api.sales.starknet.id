use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use starknet::core::types::FieldElement;
use std::fmt::Write;

#[macro_export]
macro_rules! pub_struct {
    ($($derive:path),*; $name:ident {$($field:ident: $t:ty),* $(,)?}) => {
        #[derive($($derive),*)]
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}

pub fn get_error(error: String) -> Response {
    (StatusCode::INTERNAL_SERVER_ERROR, error).into_response()
}

pub fn get_specific_error(code: StatusCode, error: String) -> Response {
    (code, error).into_response()
}

pub fn to_hex(felt: FieldElement) -> String {
    let bytes = felt.to_bytes_be();
    
    if bytes.iter().all(|&b| b == 0) {
        return String::from("0x0");
    }

    let non_zero_bytes = bytes.iter().skip_while(|&&b| b == 0);

    let mut result = String::with_capacity(bytes.len() * 2 + 2);
    result.push_str("0x");
    for &byte in non_zero_bytes {
        write!(&mut result, "{:02x}", byte).unwrap();
    }
    
    result
}

#[cfg(test)]
mod mytests{
    use starknet::core::types::FieldElement;
    use super::to_hex;

    #[test]
    fn test_to_hex_small_number() {
        let num = FieldElement::from(255u64);
        assert_eq!(to_hex(num), "0xff");
    }

    #[test]
    fn test_to_hex_large_number() {
        let num = FieldElement::from(1234567890u64);
        assert_eq!(to_hex(num), "0x499602d2");
    }

    #[test]
    fn test_single_digit() {
        let num = FieldElement::from(10u64);
        assert_eq!(to_hex(num), "0x0a");
    }

    #[test]
    fn test_boundary_values() {
        let cases = [
            (u64::MAX, "0xffffffffffffffff"),
            (u64::MIN, "0x0"),
            (u32::MAX as u64, "0xffffffff"),
        ];

        for (input, expected) in cases {
            let num = FieldElement::from(input);
            assert_eq!(to_hex(num), expected);
        }
    }

    #[test]
    fn test_to_hex_max() {
        let max = FieldElement::MAX;
        assert_eq!(to_hex(max).len(), 66);
        assert!(to_hex(max).starts_with("0x"));
    }

}
