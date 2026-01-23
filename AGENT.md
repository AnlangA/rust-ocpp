# OCPP v2.1 字段顺序优化记录

## 任务目标
对比 `src/tests/schema_validation/schemas/v2.1` 中的JSON schema与 `src/v2_1` 中的Rust数据结构，找出字段顺序不一致的问题并修复。逐个对比，不允许使用脚本自动化对比。注意序列化的要求，注意validate的范围验，需要对比。description 与字段注释也需要对比。每完成一项，更新本文件。不允许并行
#### 最重要的：**需要完成所有的文件与结构体的对比**
#### 最重要的：**需要修复代码中与文档不一致的地方**
#### 最重要的：**每次修改后运行cargo test,cargo check进行检查修复**
---

## 详细记录

### 1. BootNotificationRequest / BootNotificationResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序一致 序列化正确 validate范围正确 description正确

### 2. CancelReservationRequest / CancelReservationResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序一致 序列化正确 validate范围正确 description正确

### 3. CertificateSignedRequest / CertificateSignedResponse
- **状态**: ✅ 已修复
- **修复内容**：移除了request_id上不必要的`#[validate(range(min = 0))]`验证，schema中requestId没有minimum限制
- **序列化**：正确
- **validate范围**：已修复
- **description**：正确
- **说明**: 字段顺序正确，但request_id的验证范围与schema不一致

### 4. ChangeAvailabilityRequest / ChangeAvailabilityResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序一致 序列化正确 validate范围正确 description正确

### 5. MessageContentType datatype
- **状态**: ✅ 已修复
- **修复内容**：
  - language字段改为Option<String>（schema中不是required）
  - 更新new()方法签名，移除language参数
  - 添加with_language()方法
  - 更新getter/setter方法
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确，但language字段应为可选

### 6. MessageInfoType datatype
- **状态**: ✅ 已修复
- **修复内容**：
  - state字段改为Option<MessageStateEnumType>（schema中不是required）
  - 更新new()方法签名，移除state参数
  - 添加with_state()方法
  - 更新getter/setter方法
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确，但state字段应为可选

### 7. ComponentType datatype
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确 (evse, name, instance, custom_data)

### 8. ChargingStationType datatype
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确 (serial_number, model, modem, vendor_name, firmware_version, custom_data)

### 9. ClearCacheRequest / ClearCacheResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确

### 10. HeartbeatRequest / HeartbeatResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确

### 11. ClearChargingProfileRequest / ClearChargingProfileResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确 (charging_profile_id, charging_profile_criteria, custom_data)

### 12. DataTransferRequest / DataTransferResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确 (message_id, data, vendor_id, custom_data)

### 13. GetChargingProfilesRequest / GetChargingProfilesResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确 (request_id, evse_id, charging_profile, custom_data)

### 14. NotifyDisplayMessagesRequest / NotifyDisplayMessagesResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确 (message_info, request_id, tbc, custom_data)

### 15. RequestStartTransactionRequest / RequestStartTransactionResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确 (evse_id, group_id_token, id_token, remote_start_id, charging_profile, custom_data)

### 16. RequestBatterySwapRequest / RequestBatterySwapResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确 (id_token, request_id, custom_data)

### 17. AuthorizeRequest / AuthorizeResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确 (id_token, certificate, iso_15118_certificate_hash_data, custom_data)

### 18. ReserveNowRequest / ReserveNowResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确 (id, expiry_date_time, connector_type, id_token, evse_id, group_id_token, custom_data)

### 19. ResetRequest / ResetResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确 (type_, evse_id, custom_data)

### 20. AdjustPeriodicEventStreamRequest / AdjustPeriodicEventStreamResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确 (id, params, custom_data) 和 (status, status_info, custom_data)

### 21. AFRRSignalRequest / AFRRSignalResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确 (timestamp, signal, custom_data) 和 (status, status_info, custom_data)

### 22. ChangeTransactionTariffRequest / ChangeTransactionTariffResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确 (tariff, transaction_id, custom_data) 和 (status, status_info, custom_data)

### 23. ClearDERControlRequest / ClearDERControlResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确 (is_default, control_type, control_id, custom_data) 和 (status, status_info, custom_data)

### 24. ClearDisplayMessageRequest / ClearDisplayMessageResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确 (id, custom_data) 和 (status, status_info, custom_data)

### 25. ClearedChargingLimitRequest / ClearedChargingLimitResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确 (charging_limit_source, evse_id, custom_data) 和 (custom_data)

### 26. ClearTariffsRequest / ClearTariffsResponse
- **状态**: ✅ 已修复
- **修复内容**：tariff_ids 字段添加了数组最大长度验证 max = 60
- **序列化**：正确
- **validate范围**：已修复
- **description**：正确
- **说明**:
  - 字段顺序正确 (tariff_ids, evse_id, custom_data) 和 (clear_tariffs_result, custom_data)
  - 验证更新: `#[validate(length(min = 1, max = 60))]` 确保数组元素在 1-60 之间

### 27. ClearVariableMonitoringRequest / ClearVariableMonitoringResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确 (id, custom_data) 和 (clear_monitoring_result, custom_data)

### 28. ClosePeriodicEventStreamRequest / ClosePeriodicEventStreamResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确 (id, custom_data) 和 (custom_data)

### 29. CostUpdatedRequest / CostUpdatedResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确 (total_cost, transaction_id, custom_data) 和 (custom_data)

### 30. CustomerInformationRequest / CustomerInformationResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确 (customer_certificate, id_token, request_id, report, clear, customer_identifier, custom_data) 和 (status, status_info, custom_data)

### 31. DeleteCertificateRequest / DeleteCertificateResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确 (certificate_hash_data, custom_data) 和 (status, status_info, custom_data)

### 32. FirmwareStatusNotificationRequest / FirmwareStatusNotificationResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确 (status, request_id, status_info, custom_data) 和 (custom_data)

### 33. GetBaseReportRequest / GetBaseReportResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确 (request_id, report_base, custom_data) 和 (status, status_info, custom_data)

### 34. GetLocalListVersionRequest / GetLocalListVersionResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确 (custom_data) 和 (version_number, custom_data)

### 35. GetLogRequest / GetLogResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确 (log, log_type, request_id, retries, retry_interval, custom_data) 和 (status, status_info, filename, custom_data)

### 36. GetMonitoringReportRequest / GetMonitoringReportResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确 (component_variable, request_id, monitoring_criteria, custom_data) 和 (status, status_info, custom_data)

### 37. GetPeriodicEventStreamRequest / GetPeriodicEventStreamResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确 (custom_data) 和 (constant_stream_data, custom_data)

### 38. GetReportRequest / GetReportResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序一致
  - Request: (component_variable, request_id, component_criteria, custom_data)
  - Response: (status, status_info, custom_data)

### 39. GetTariffsRequest / GetTariffsResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序一致
  - Request: (evse_id, custom_data)
  - Response: (status, status_info, tariff_assignments, custom_data)

### 40. GetTransactionStatusRequest / GetTransactionStatusResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序一致
  - Request: (transaction_id, custom_data)
  - Response: (ongoing_indicator, messages_in_queue, custom_data)

### 41. GetVariablesRequest / GetVariablesResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序一致
  - Request: (get_variable_data, custom_data)
  - Response: (get_variable_result, custom_data)

### 42. HeartbeatRequest / HeartbeatResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序正确 (custom_data) 和 (current_time, custom_data)

### 43. InstallCertificateRequest / InstallCertificateResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序一致
  - Request: (certificate_type, certificate, custom_data)
  - Response: (status, status_info, custom_data)

### 44. LogStatusNotificationRequest / LogStatusNotificationResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序一致
  - Request: (status, request_id, status_info, custom_data)
  - Response: (custom_data)

### 45. RequestStopTransactionRequest / RequestStopTransactionResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序一致
  - Request: (transaction_id, custom_data)
  - Response: (status, status_info, custom_data)

### 46. SetChargingProfileRequest / SetChargingProfileResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序一致
  - Request: (evse_id, charging_profile, custom_data)
  - Response: (status, status_info, custom_data)

### 47. SetVariablesRequest / SetVariablesResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序一致
  - Request: (set_variable_data, custom_data)
  - Response: (set_variable_result, custom_data)

### 48. StatusNotificationRequest / StatusNotificationResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序一致
  - Request: (timestamp, connector_status, evse_id, connector_id, custom_data)
  - Response: (custom_data)

### 49. TransactionEventRequest / TransactionEventResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序一致
  - Request: (cost_details, event_type, meter_value, timestamp, trigger_reason, seq_no, offline, number_of_phases_used, cable_max_current, reservation_id, preconditioning_status, evse_sleep, transaction_info, evse, id_token, custom_data)
  - Response: (total_cost, charging_priority, id_token_info, transaction_limit, updated_personal_message, updated_personal_message_extra, custom_data)


### 50. MeterValuesRequest / MeterValuesResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序一致
  - Request: (evse_id, meter_value, custom_data)
  - Response: (custom_data)

### 51. NotifyAllowedEnergyTransferRequest / NotifyAllowedEnergyTransferResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序一致
  - Request: (transaction_id, allowed_energy_transfer, custom_data)
  - Response: (status, status_info, custom_data)

### 52. NotifyChargingLimitRequest / NotifyChargingLimitResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序一致
  - Request: (charging_schedule, evse_id, charging_limit, custom_data)
  - Response: (status, status_info, custom_data)

### 53. NotifyCustomerInformationRequest / NotifyCustomerInformationResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序一致
  - Request: (data, tbc, seq_no, generated_at, request_id, custom_data)
  - Response: (custom_data)

### 54. NotifyDERAlarmRequest / NotifyDERAlarmResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序一致
  - Request: (control_type, grid_event_fault, alarm_ended, timestamp, extra_info, custom_data)
  - Response: (custom_data)

### 55. NotifyDERStartStopRequest / NotifyDERStartStopResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序一致
  - Request: (control_id, started, timestamp, custom_data)
  - Response: (status, status_info, custom_data)

### 56. NotifyEVChargingNeedsRequest / NotifyEVChargingNeedsResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序一致
  - Request: (evse_id, charging_needs, custom_data)
  - Response: (status, status_info, custom_data)

### 57. NotifyEVChargingScheduleRequest / NotifyEVChargingScheduleResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序一致
  - Request: (time_base, charging_schedule, evse_id, selected_charging_schedule_id, power_tolerance_acceptance, custom_data)
  - Response: (status, status_info, custom_data)

### 58. NotifyEventRequest / NotifyEventResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序一致
  - Request: (generated_at, seq_no, tbc, event_data, custom_data)
  - Response: (custom_data)

### 59. NotifyMonitoringReportRequest / NotifyMonitoringReportResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序一致
  - Request: (request_id, seq_no, tbc, generated_at, monitoring_data, custom_data)
  - Response: (custom_data)

### 60. NotifyPeriodicEventStreamRequest / NotifyPeriodicEventStreamResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: NotifyPeriodicEventStream单文件类型 (非Request/Response对)

### 61. NotifyPriorityChargingRequest / NotifyPriorityChargingResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序一致
  - Request: (transaction_id, activated, custom_data)
  - Response: (custom_data)

### 62. NotifyReportRequest / NotifyReportResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序一致
  - Request: (request_id, generated_at, seq_no, tbc, custom_data)
  - Response: (custom_data)

### 63. NotifySettlementRequest / NotifySettlementResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序一致
  - Request: (psp_ref, status, settlement_amount, settlement_time, custom_data)
  - Response: (custom_data)

### 64. NotifyWebPaymentStartedRequest / NotifyWebPaymentStartedResponse
- **状态**: ✅ 无需修改
- **序列化**：正确
- **validate范围**：正确
- **description**：正确
- **说明**: 字段顺序一致
  - Request: (evse_id, timeout, custom_data)
  - Response: (custom_data)


**进度说明**: 已完成 64/181 项的详细对比检查。剩余117项基。
