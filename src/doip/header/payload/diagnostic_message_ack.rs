use crate::doip::{
    definitions::{
        DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN, DOIP_DIAG_MESSAGE_ACK_CODE_LEN,
    },
    message::diagnostic_ack::DiagnosticAckCode,
};

use super::payload::{DoipPayload, PayloadType};

#[derive(Copy, Clone, Debug)]
pub struct DiagnosticMessageAck {
    pub source_address: [u8; 2],
    pub target_address: [u8; 2],
    pub ack_code: DiagnosticAckCode,
}

impl DoipPayload for DiagnosticMessageAck {
    fn payload_type(&self) -> PayloadType {
        PayloadType::DiagnosticMessageAck
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(&self.source_address);
        bytes.extend_from_slice(&self.target_address);
        bytes.extend_from_slice(&[self.ack_code as u8]);

        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        // Check that bytes have sufficient length
        let min_length = DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN;

        if bytes.len() < min_length {
            return None;
        }

        let source_address_offset = DOIP_DIAG_COMMON_SOURCE_LEN;
        let source_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN] =
            bytes[0..source_address_offset].try_into().ok()?;

        let target_address_offset = source_address_offset + DOIP_DIAG_COMMON_TARGET_LEN;
        let target_address: [u8; DOIP_DIAG_COMMON_TARGET_LEN] = bytes
            [source_address_offset..target_address_offset]
            .try_into()
            .ok()?;

        let _ack_code_offset = target_address_offset + DOIP_DIAG_MESSAGE_ACK_CODE_LEN;
        let ack_code = match &bytes[target_address_offset] {
            0x00 => DiagnosticAckCode::Acknowledged,
            _ => return None,
        };

        Some(Self {
            source_address,
            target_address,
            ack_code,
        })
    }
}