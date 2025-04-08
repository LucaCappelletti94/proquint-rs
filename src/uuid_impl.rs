//! Submodule providing the implementation of the [`Quintable`] trait for the
//! [`Uuid`](uuid::Uuid) type.

use uuid::Uuid;
use crate::{Quintable, QuintError, unquint_exactly};

impl Quintable for Uuid {
    fn to_quint(&self) -> String {
        let bytes = self.as_bytes();
        
        // Convert UUID bytes to two u64 values for processing
        let high = u64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7],
        ]);
        
        let low = u64::from_be_bytes([
            bytes[8], bytes[9], bytes[10], bytes[11],
            bytes[12], bytes[13], bytes[14], bytes[15],
        ]);
        
        // Generate quint string for both halves and join them
        let high_quint = high.to_quint();
        let low_quint = low.to_quint();
        
        format!("{}-{}", high_quint, low_quint)
    }

    fn from_quint(quint: &str) -> Result<Self, QuintError> {
        // Parse first 64 bits
        let (high, last_i) = unquint_exactly::<u64>(quint, 64)?;
        if last_i + 1 >= quint.len() {
            return Err(QuintError::InputTooSmall);
        }
        
        // Parse remaining 64 bits
        let remaining = &quint[last_i + 1..];
        let (low, _) = unquint_exactly::<u64>(remaining, 64)?;
        
        // Convert the two u64 values back to UUID bytes
        let high_bytes = high.to_be_bytes();
        let low_bytes = low.to_be_bytes();
        
        let mut uuid_bytes = [0u8; 16];
        uuid_bytes[0..8].copy_from_slice(&high_bytes);
        uuid_bytes[8..16].copy_from_slice(&low_bytes);
        
        Ok(Uuid::from_bytes(uuid_bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Quintable;

    #[test]
    fn uuid_conversion() {
        // Create a known UUID
        let uuid = Uuid::parse_str("f81d4fae-7dec-11d0-a765-00a0c91e6bf6").unwrap();
        
        // Convert to quint and back
        let quint = uuid.to_quint();
		assert_eq!(quint, "zobit-huvov-lulos-dalib-pitoj-bafob-sohiv-kozuk");

        let converted = Uuid::from_quint(&quint).unwrap();
        
        // Verify they match
        assert_eq!(uuid, converted);
    }
}

