use super::payload::{DoipPayload, PayloadType};
use crate::doip::{definitions::DOIP_GENERIC_NACK_LEN, message::nack_codes::NackCodes};

#[derive(Copy, Clone, Debug)]
pub struct GenericNack {
    pub nack_code: NackCodes,
}

impl DoipPayload for GenericNack {
    fn payload_type(&self) -> PayloadType {
        PayloadType::GenericNack
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        let nc = [(self.nack_code as u8)];

        bytes.extend_from_slice(&nc);
        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        // Check that bytes have sufficient length
        let min_length = DOIP_GENERIC_NACK_LEN;

        if bytes.len() < min_length {
            return None;
        }

        let nack_code_offset = DOIP_GENERIC_NACK_LEN;
        let nack_code = match &bytes[nack_code_offset] {
          0x00 => NackCodes::IncorrectPatternFormat,
          0x01 => NackCodes::UnknownPayloadType,
          0x02 => NackCodes::MessageTooLarge,
          0x03 => NackCodes::OutOfMemory,
          0x04 => NackCodes::InvalidPayloadLength,
            _ => return None,
        };

        Some(Self { nack_code })
    }
}