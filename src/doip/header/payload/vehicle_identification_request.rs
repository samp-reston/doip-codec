use super::payload::{DoipPayload, PayloadError, PayloadType};

#[derive(Copy, Clone, Debug)]
pub struct VehicleIdentificationRequest {}

impl DoipPayload for VehicleIdentificationRequest {
    fn payload_type(&self) -> PayloadType {
        PayloadType::VehicleIdentificationRequest
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![]
    }

    fn from_bytes(_bytes: &[u8]) -> Result<Self, PayloadError> {
        Ok(Self {})
    }
}
