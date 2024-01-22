/*
 * Copyright (C) 2015-2022 IoT.bzh Company
 * Author: Fulup Ar Foll <fulup@iot.bzh>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 */

use afbv4::prelude::*;
use ocpp::prelude::*;

use v106::messages::{
    authorize::AuthorizeRequest, authorize::AuthorizeResponse,
    boot_notification::BootNotificationRequest, boot_notification::BootNotificationResponse,
    cancel_reservation::CancelReservationRequest, cancel_reservation::CancelReservationResponse,
    change_availability::ChangeAvailabilityRequest,
    change_availability::ChangeAvailabilityResponse,
    change_configuration::ChangeConfigurationRequest,
    change_configuration::ChangeConfigurationResponse, clear_cache::ClearCacheRequest,
    clear_cache::ClearCacheResponse, clear_charging_profile::ClearChargingProfileRequest,
    clear_charging_profile::ClearChargingProfileResponse, data_transfer::DataTransferRequest,
    data_transfer::DataTransferResponse,
    diagnostics_status_notification::DiagnosticsStatusNotificationRequest,
    diagnostics_status_notification::DiagnosticsStatusNotificationResponse,
    firmware_status_notification::FirmwareStatusNotificationRequest,
    firmware_status_notification::FirmwareStatusNotificationResponse,
    get_composite_schedule::GetCompositeScheduleRequest,
    get_composite_schedule::GetCompositeScheduleResponse,
    get_configuration::GetConfigurationRequest, get_configuration::GetConfigurationResponse,
    get_diagnostics::GetDiagnosticsRequest, get_diagnostics::GetDiagnosticsResponse,
    get_local_list_version::GetLocalListVersionRequest,
    get_local_list_version::GetLocalListVersionResponse, heart_beat::HeartbeatRequest,
    heart_beat::HeartbeatResponse, meter_values::MeterValuesRequest,
    meter_values::MeterValuesResponse, remote_start_transaction::RemoteStartTransactionRequest,
    remote_start_transaction::RemoteStartTransactionResponse,
    remote_stop_transaction::RemoteStopTransactionRequest,
    remote_stop_transaction::RemoteStopTransactionResponse, reserve_now::ReserveNowRequest,
    reserve_now::ReserveNowResponse, reset::ResetRequest, reset::ResetResponse,
    send_local_list::SendLocalListRequest, send_local_list::SendLocalListResponse,
    set_charging_profile::SetChargingProfileRequest,
    set_charging_profile::SetChargingProfileResponse, start_transaction::StartTransactionRequest,
    start_transaction::StartTransactionResponse, status_notification::StatusNotificationRequest,
    status_notification::StatusNotificationResponse, stop_transaction::StopTransactionRequest,
    stop_transaction::StopTransactionResponse, trigger_message::TriggerMessageRequest,
    trigger_message::TriggerMessageResponse, unlock_connector::UnlockConnectorRequest,
    unlock_connector::UnlockConnectorResponse, update_firmware::UpdateFirmwareRequest,
    update_firmware::UpdateFirmwareResponse,
};

AfbDataConverter!(authorize_rqt, AuthorizeRequest);
AfbDataConverter!(authorize_rsp, AuthorizeResponse);
AfbDataConverter!(boot_notification_rqt, BootNotificationRequest);
AfbDataConverter!(boot_notification_rsp, BootNotificationResponse);

pub fn register(_binding: AfbApiV4) -> Result<(), AfbError> {
    // Custom type should be registered at binding startup time
    authorize_rqt::register()?;
    authorize_rsp::register()?;
    boot_notification_rqt::register()?;
    boot_notification_rsp::register()?;
    Ok(())
}
