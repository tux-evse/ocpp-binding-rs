/*
 * Copyright (C) Codelab(Sweden) https://github.com/codelabsab
 * Author: https://github.com/codelabsab/ocpp-csms-server
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not type xxx= this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 */
use strum_macros::Display;
use afbv4::prelude::*;

use rust_ocpp::v1_6::*;
pub type AuthorizeRequest= messages::authorize::AuthorizeRequest;
pub type AuthorizeResponse= messages::authorize::AuthorizeResponse;
pub type BootNotificationRequest= messages::boot_notification::BootNotificationRequest;
pub type BootNotificationResponse= messages::boot_notification::BootNotificationResponse;
pub type CancelReservationRequest= messages::cancel_reservation::CancelReservationRequest;
pub type CancelReservationResponse= messages::cancel_reservation::CancelReservationResponse;
pub type ChangeAvailabilityRequest= messages::change_availability::ChangeAvailabilityRequest;
pub type ChangeAvailabilityResponse= messages::change_availability::ChangeAvailabilityResponse;
pub type ChangeConfigurationRequest= messages::change_configuration::ChangeConfigurationRequest;
pub type ChangeConfigurationResponse= messages::change_configuration::ChangeConfigurationResponse;
pub type ClearCacheRequest= messages::clear_cache::ClearCacheRequest;
pub type ClearCacheResponse= messages::clear_cache::ClearCacheResponse;
pub type ClearChargingProfileRequest= messages::clear_charging_profile::ClearChargingProfileRequest;
pub type ClearChargingProfileResponse= messages::clear_charging_profile::ClearChargingProfileResponse;
pub type DataTransferRequest= messages::data_transfer::DataTransferRequest;
pub type DataTransferResponse= messages::data_transfer::DataTransferResponse;
pub type DiagnosticsStatusNotificationRequest= messages::diagnostics_status_notification::DiagnosticsStatusNotificationRequest;
pub type DiagnosticsStatusNotificationResponse= messages::diagnostics_status_notification::DiagnosticsStatusNotificationResponse;
pub type FirmwareStatusNotificationRequest= messages::firmware_status_notification::FirmwareStatusNotificationRequest;
pub type FirmwareStatusNotificationResponse= messages::firmware_status_notification::FirmwareStatusNotificationResponse;
pub type GetCompositeScheduleRequest= messages::get_composite_schedule::GetCompositeScheduleRequest;
pub type GetCompositeScheduleResponse= messages::get_composite_schedule::GetCompositeScheduleResponse;
pub type GetConfigurationRequest= messages::get_configuration::GetConfigurationRequest;
pub type GetConfigurationResponse= messages::get_configuration::GetConfigurationResponse;
pub type GetDiagnosticsRequest= messages::get_diagnostics::GetDiagnosticsRequest;
pub type GetDiagnosticsResponse= messages::get_diagnostics::GetDiagnosticsResponse;
pub type GetLocalListVersionRequest= messages::get_local_list_version::GetLocalListVersionRequest;
pub type GetLocalListVersionResponse= messages::get_local_list_version::GetLocalListVersionResponse;
pub type HeartbeatRequest= messages::heart_beat::HeartbeatRequest;
pub type HeartbeatResponse= messages::heart_beat::HeartbeatResponse;
pub type MeterValuesRequest= messages::meter_values::MeterValuesRequest;
pub type MeterValuesResponse= messages::meter_values::MeterValuesResponse;
pub type RemoteStartTransactionRequest= messages::remote_start_transaction::RemoteStartTransactionRequest;
pub type RemoteStartTransactionResponse= messages::remote_start_transaction::RemoteStartTransactionResponse;
pub type RemoteStopTransactionRequest= messages::remote_stop_transaction::RemoteStopTransactionRequest;
pub type RemoteStopTransactionResponse= messages::remote_stop_transaction::RemoteStopTransactionResponse;
pub type ReserveNowRequest= messages::reserve_now::ReserveNowRequest;
pub type ReserveNowResponse= messages::reserve_now::ReserveNowResponse;
pub type ResetRequest= messages::reset::ResetRequest;
pub type ResetResponse= messages::reset::ResetResponse;
pub type SendLocalListRequest= messages::send_local_list::SendLocalListRequest;
pub type SendLocalListResponse= messages::send_local_list::SendLocalListResponse;
pub type SetChargingProfileRequest= messages::set_charging_profile::SetChargingProfileRequest;
pub type SetChargingProfileResponse= messages::set_charging_profile::SetChargingProfileResponse;
pub type StartTransactionRequest= messages::start_transaction::StartTransactionRequest;
pub type StartTransactionResponse= messages::start_transaction::StartTransactionResponse;
pub type StatusNotificationRequest= messages::status_notification::StatusNotificationRequest;
pub type StatusNotificationResponse= messages::status_notification::StatusNotificationResponse;
pub type StopTransactionRequest= messages::stop_transaction::StopTransactionRequest;
pub type StopTransactionResponse= messages::stop_transaction::StopTransactionResponse;
pub type TriggerMessageRequest= messages::trigger_message::TriggerMessageRequest;
pub type TriggerMessageResponse= messages::trigger_message::TriggerMessageResponse;
pub type UnlockConnectorRequest= messages::unlock_connector::UnlockConnectorRequest;
pub type UnlockConnectorResponse= messages::unlock_connector::UnlockConnectorResponse;
pub type UpdateFirmwareRequest= messages::update_firmware::UpdateFirmwareRequest;
pub type UpdateFirmwareResponse= messages::update_firmware::UpdateFirmwareResponse;

pub type ChargePointStatus= types::ChargePointStatus;
pub type ChargePointErrorCode= types::ChargePointErrorCode;
pub type CancelReservationStatus= types::CancelReservationStatus;
pub type AvailabilityType= types::AvailabilityType;
pub type AvailabilityStatus= types::AvailabilityStatus;
pub type ResetResponseStatus= types::ResetResponseStatus;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum RemoteStopTransaction {
    Request(RemoteStopTransactionRequest),
    Response(RemoteStopTransactionResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum RemoteStartTransaction {
    Request(RemoteStartTransactionRequest),
    Response(RemoteStartTransactionResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum GetDiagnostics {
    Request(GetDiagnosticsRequest),
    Response(GetDiagnosticsResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum GetConfiguration {
    Request(GetConfigurationRequest),
    Response(GetConfigurationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum DiagnosticsStatusNotification {
    Request(DiagnosticsStatusNotificationRequest),
    Response(DiagnosticsStatusNotificationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum ChangeConfiguration {
    Request(ChangeConfigurationRequest),
    Response(ChangeConfigurationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum StartTransaction {
    Request(StartTransactionRequest),
    Response(StartTransactionResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum StopTransaction {
    Request(StopTransactionRequest),
    Response(StopTransactionResponse),
}

// charger -> backend OCPP-1.6 Authorize.req(p60/#6.1) Authorize.conf(6.2)
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum Authorize {
    Request(AuthorizeRequest),
    Response(AuthorizeResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum BootNotification {
    Request(BootNotificationRequest),
    Response(BootNotificationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum CancelReservation {
    Request(CancelReservationRequest),
    Response(CancelReservationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum ChangeAvailability {
    Request(ChangeAvailabilityRequest),
    Response(ChangeAvailabilityResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum ClearCache {
    Request(ClearCacheRequest),
    Response(ClearCacheResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum ClearChargingProfile {
    Request(ClearChargingProfileRequest),
    Response(ClearChargingProfileResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum DataTransfer {
    Request(DataTransferRequest),
    Response(DataTransferResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum FirmwareStatusNotification {
    Request(FirmwareStatusNotificationRequest),
    Response(FirmwareStatusNotificationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum GetCompositeSchedule {
    Request(GetCompositeScheduleRequest),
    Response(GetCompositeScheduleResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum GetLocalListVersion {
    Request(GetLocalListVersionRequest),
    Response(GetLocalListVersionResponse),
}

// Warning: declare empty object and the end other wise they always match
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum Heartbeat {
    Response(HeartbeatResponse),
    Request(HeartbeatRequest),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum MeterValues {
    Request(MeterValuesRequest),
    Response(MeterValuesResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum ReserveNow {
    Request(ReserveNowRequest),
    Response(ReserveNowResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum Reset {
    Request(ResetRequest),
    Response(ResetResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum SendLocalList {
    Request(SendLocalListRequest),
    Response(SendLocalListResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum SetChargingProfile {
    Request(SetChargingProfileRequest),
    Response(SetChargingProfileResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum StatusNotification {
    Request(StatusNotificationRequest),
    Response(StatusNotificationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum TriggerMessage {
    Request(TriggerMessageRequest),
    Response(TriggerMessageResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum UnlockConnector {
    Request(UnlockConnectorRequest),
    Response(UnlockConnectorResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum UpdateFirmware {
    Request(UpdateFirmwareRequest),
    Response(UpdateFirmwareResponse),
}

// implement afb-v4 type encoder/decoder
AfbDataConverter!(boot_notification, BootNotification);
AfbDataConverter!(authorization, Authorize);
AfbDataConverter!(heartbeat, Heartbeat);
AfbDataConverter!(status_notification, StatusNotification);
AfbDataConverter!(cancel_reservation, CancelReservation);
AfbDataConverter!(change_availability, ChangeAvailability);
AfbDataConverter!(remote_start_transaction, RemoteStartTransaction);
AfbDataConverter!(remote_stop_transaction, RemoteStopTransaction);
AfbDataConverter!(remote_reset, Reset);

pub fn register_datatype() -> Result<(),AfbError> {
    // Custom type should be registered at binding startup time
   boot_notification::register()?;
   authorization::register()?;
   heartbeat::register()?;
   status_notification::register()?;
   cancel_reservation::register()?;
   change_availability::register()?;
   remote_start_transaction::register()?;
   remote_stop_transaction::register()?;
   remote_reset::register()?;
   Ok(())
}
