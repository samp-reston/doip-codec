use crate::doip::{
    definitions::{
        DOIP_COMMON_EID_LEN, DOIP_COMMON_VIN_LEN, DOIP_DIAG_COMMON_SOURCE_LEN,
        DOIP_VEHICLE_ANNOUNCEMENT_ACTION_LEN, DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN,
    },
    message::{action_code::ActionCode, sync_status::SyncStatus},
};

use super::payload::{DoipPayload, PayloadType};

#[derive(Copy, Clone, Debug)]
pub struct VehicleAnnouncementMessage {
    pub vin: [u8; DOIP_COMMON_VIN_LEN],
    pub logical_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN],
    pub eid: [u8; DOIP_COMMON_EID_LEN],
    pub gid: [u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
    pub further_action: ActionCode,
    pub vin_gid_sync: Option<SyncStatus>,
}

impl DoipPayload for VehicleAnnouncementMessage {
    fn payload_type(&self) -> PayloadType {
        PayloadType::VehicleAnnouncementMessage
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(&self.vin);
        bytes.extend_from_slice(&self.logical_address);
        bytes.extend_from_slice(&self.eid);
        bytes.extend_from_slice(&self.gid);
        bytes.extend_from_slice(&[self.further_action as u8]);

        if let Some(sync_status) = self.vin_gid_sync {
            bytes.push(sync_status as u8); // Assuming `SyncStatus` can be cast to `u8`
        }

        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        // Check that bytes have sufficient length
        let min_length = DOIP_COMMON_VIN_LEN
            + DOIP_DIAG_COMMON_SOURCE_LEN
            + DOIP_COMMON_EID_LEN
            + DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN
            + DOIP_VEHICLE_ANNOUNCEMENT_ACTION_LEN;

        if bytes.len() < min_length {
            return None;
        }

        let vin_offset = DOIP_COMMON_VIN_LEN;
        let vin: [u8; DOIP_COMMON_VIN_LEN] = bytes[0..vin_offset].try_into().ok()?;

        let logical_address_offset = vin_offset + DOIP_DIAG_COMMON_SOURCE_LEN;
        let logical_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN] =
            bytes[vin_offset..logical_address_offset].try_into().ok()?;

        let eid_offset = logical_address_offset + DOIP_COMMON_EID_LEN;
        let eid: [u8; DOIP_COMMON_EID_LEN] =
            bytes[logical_address_offset..eid_offset].try_into().ok()?;

        let gid_offset = eid_offset + DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN;
        let gid: [u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN] =
            bytes[eid_offset..gid_offset].try_into().ok()?;

        let further_action_offset = gid_offset;
        let vin_gid_sync_offset = further_action_offset + DOIP_VEHICLE_ANNOUNCEMENT_ACTION_LEN;

        let further_action = match &bytes[further_action_offset] {
            0x00 => ActionCode::NoFurtherActionRequired,
            0x01 => ActionCode::ReservedByIso13400_01,
            0x02 => ActionCode::ReservedByIso13400_02,
            0x03 => ActionCode::ReservedByIso13400_03,
            0x04 => ActionCode::ReservedByIso13400_04,
            0x05 => ActionCode::ReservedByIso13400_05,
            0x06 => ActionCode::ReservedByIso13400_06,
            0x07 => ActionCode::ReservedByIso13400_07,
            0x08 => ActionCode::ReservedByIso13400_08,
            0x09 => ActionCode::ReservedByIso13400_09,
            0x0A => ActionCode::ReservedByIso13400_0A,
            0x0B => ActionCode::ReservedByIso13400_0B,
            0x0C => ActionCode::ReservedByIso13400_0C,
            0x0D => ActionCode::ReservedByIso13400_0D,
            0x0E => ActionCode::ReservedByIso13400_0E,
            0x0F => ActionCode::ReservedByIso13400_0F,
            0x10 => ActionCode::RoutingActivationRequired,
            _ => return None,
        };

        let vin_gid_sync: Option<SyncStatus> = match bytes.get(vin_gid_sync_offset) {
            Some(0x00) => Some(SyncStatus::VinGinSynchronized),
            Some(0x01) => Some(SyncStatus::ReservedByIso13400_01),
            Some(0x02) => Some(SyncStatus::ReservedByIso13400_02),
            Some(0x03) => Some(SyncStatus::ReservedByIso13400_03),
            Some(0x04) => Some(SyncStatus::ReservedByIso13400_04),
            Some(0x05) => Some(SyncStatus::ReservedByIso13400_05),
            Some(0x06) => Some(SyncStatus::ReservedByIso13400_06),
            Some(0x07) => Some(SyncStatus::ReservedByIso13400_07),
            Some(0x08) => Some(SyncStatus::ReservedByIso13400_08),
            Some(0x09) => Some(SyncStatus::ReservedByIso13400_09),
            Some(0x0A) => Some(SyncStatus::ReservedByIso13400_0A),
            Some(0x0B) => Some(SyncStatus::ReservedByIso13400_0B),
            Some(0x0C) => Some(SyncStatus::ReservedByIso13400_0C),
            Some(0x0D) => Some(SyncStatus::ReservedByIso13400_0D),
            Some(0x0E) => Some(SyncStatus::ReservedByIso13400_0E),
            Some(0x0F) => Some(SyncStatus::ReservedByIso13400_0F),
            Some(0x10) => Some(SyncStatus::VinGinNotSynchronised),
            _ => None,
        };

        Some(Self {
            vin,
            logical_address,
            eid,
            gid,
            further_action,
            vin_gid_sync,
        })
    }
}
