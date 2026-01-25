# Part 5: Certification Profiles
> **Source**: `OCPP-2.1_edition2_part5_certification_profiles.pdf`
> **Total Pages**: 70

---

## Table of Contents

- OCPP 2.1: Part 5 - Certification Profiles (Page 1)
- Table of Contents (Page 2)
- Disclaimer (Page 3)
- Version History (Page 4)
- Chapter 1. Introduction & Reading Guide (Page 5)
- Chapter 2. Certification profiles (Page 6)
- Chapter 3. Features (Page 10)
- Chapter 4. List of test cases (Page 17)
- Chapter 5. OCPP 2.1 Mandatory Controller components per profile (Page 64)
- Chapter 6. Appendix A: additional questions for the Protocol Implementation Conformance Statement (Page 65)
- Chapter 7. Appendix B: Hardware feature set (Page 66)
- Chapter 8. Appendix C: Features vs. OCPP use cases (Page 67)

---

## Content


### OCPP 2.1: Part 5 - Certification Profiles

*_Source: Page 1 - 1_*

OCPP 2.1
Part 5 - Certification Profiles
Edition 2, 2025-12-03


### Table of Contents

*_Source: Page 2 - 2_*

Table of Contents
Disclaimer . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  1
Version History . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  2
1. Introduction & Reading Guide . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  3
2. Certification profiles . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  4
3. Features. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  8
3.1. Optional feature list for charging station. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  9
3.2. Optional feature list for CSMS . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  13
4. List of test cases . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  15
4.1. Introduction. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  15
4.2. Test Cases Core . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  15
4.3. Test Cases Advanced Security . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  50
4.4. Test Cases Smart Charging . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  51
4.5. Test Cases ISO 15118 Support. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  56
4.6. Test Cases Bidirectional Power Transfer (2.1). . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  60
4.7. Test Cases DER Control (2.1) . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  61
5. OCPP 2.1 Mandatory Controller components per profile . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  62
6. Appendix A: additional questions for the Protocol Implementation Conformance Statement . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  63
6.1. Questions for Charging Stations . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  63
6.2. Questions for CSMSs. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  63
7. Appendix B: Hardware feature set . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  64
8. Appendix C: Features vs. OCPP use cases . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  65


### Disclaimer

*_Source: Page 3 - 3_*

Disclaimer
Copyright © 2010 – 2025 Open Charge Alliance. All rights reserved.
This document is made available under the *Creative Commons Attribution-NoDerivatives 4.0 International Public License*
(https://creativecommons.org/licenses/by-nd/4.0/legalcode).
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 1/68 Part 5 - Certification Profiles


### Version History

*_Source: Page 4 - 4_*

Version History
Version Date Description
2.1 Edition 2 2025-12-03 OCPP 2.1 Edition 2. This is the first published version of
OCPP 2.1 Part 5.
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 2/68 Part 5 - Certification Profiles


### Chapter 1. Introduction & Reading Guide

*_Source: Page 5 - 5_*

Chapter 1. Introduction & Reading Guide
This document describes the certification profiles for OCPP 2.1. These profiles are sets of use cases that can be certified via the
Open Charge Alliance. This document contains the details on what is part of the OCPP 2.1 Certification. This document contains:
• The certification profiles and an overview of the functionality per profile.
• The list of optional features. This list contains specific functionality that is not mandatory for certification, but which can
optionally be certified.
• The list of test cases for each of the certification profiles.
• The overview of the controller components that must be implemented per profile for certification testing.
For clarity: in the context of the OCPP Certification Program, the term test case refers to a sequence of messages for testing a use
case from OCPP. The term feature refers to a functionality, that can be tested with one or more test cases (see Features for a more
detailed explanation). Instead of making specific test cases mandatory or optional, the certification program for OCPP 2.1 works
with features that are optional. Depending on whether the System Under Test (SUT) has implemented a feature, the test case(s)
that belong to this feature, must be successfully passed or not.
Figure 1. Link between different OCPP Documents in OCPP Certification Program
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 3/68 Part 5 - Certification Profiles


### Chapter 2. Certification profiles

*_Source: Page 6 - 9_*

Chapter 2. Certification profiles
The OCPP protocol has been designed to support a wide variety of charging stations ranging from simple AC home chargers to
advanced DC hyperchargers and megawatt chargers. It will be obvious that these charging stations and associated CSMSs will
have very different capabilities. As a result it does not make sense to require every vendor to certify for the full OCPP functionality,
when only subset is needed for the specific application.
The OCPP certification is built around certification profiles that describe a set of supported functions. A full OCPP certification
comprises all certification profiles, but it is possible to get certified for a subset, since not all OCPP functionality may be needed for
some vendors.
The OCPP "Core" profile must always be present. It contains the basic OCPP functionality. On top of that other profiles can be
added to the certification. These profiles are independent of each other, the only exception being the "ISO 15118 support" profile,
which requires a number of "Advanced security" and "Smart charging" test cases to be implemented.
The following table lists the certification profiles and an overview of the functionality per profile:
Table 1. Certification profiles
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 4/68 Part 5 - Certification Profiles

Certification Profile Description
Core Basic Authentication
TLS - server-side certificate
Update Charging Station Password for HTTP Basic Authentication
Security Event Notification
Booting a Charging Station
Configuring a Charging Station
Resetting a Charging Station / EVSE
Authorization incl. GroupId 
Stop Transaction with a Master Pass
Local start transaction - Cable plugin first & Authorization first
Start / Stop transaction options
Disconnect cable on EV-side
Check Transaction status
Remote start / stop transaction
Remote unlock Connector 
Remote Trigger
Change Availability - Charging Station / EVSE / Connector
Clock-aligned Meter & Sampled Meter Values
Install CA certificates
Retrieve & delete certificates from Charging Station
AdditionalRootCertificateCheck
Retrieve Log Information
Get / Clear Customer Information
Secure Firmware Update
Store / Clear Authorization Data in Authorization Cache
Authorization through authorization cache
Local Authorization List Management
Authorization through local authorization list
Send / Get Local Authorization List
Advanced Device Management
Get /Monitoring report 
Set Monitoring Base
Set Variable Monitoring
Set Monitoring Level
Get Custom Report
Clear / Remove Monitoring
Event Notification
Advanced User Interface
Set / Get Display Message
Clear a Display Message
Show EV Driver Running / Final Total Cost During / After Charging
Show EV Driver-specific Tariff Information
Update Tariff Information During Transaction
Configure Fallback Tariff Information & Total Cost Message
Reservation
Reserve a specific EVSE
Reserve an unspecified EVSE
Reserve a connector with a specific type
Reservations using GroupIdToken
Cancel reservation of an EVSE
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 5/68 Part 5 - Certification Profiles

Certification Profile Description
Payment (2.1)
Set Default Tariff
Local Cost Calculation
Prepaid card
Integrated Payment Terminal
Standalone Payment Terminal
Settlement
QR codes
Advanced Security TLS - Client-side certificate
Update Charging Station Certificate
Upgrade Charging Station Security Profile
Smart Charging (2.0.1 / 2.1) Set charging profile
Remote start transaction with charging profile
Get Composite Schedule
Get Charging Profile
Clear Charging Profile 
Priority Charging (2.1)
Dynamic Charging Profiles (2.1)
EMS Control
Dynamic charging profiles by external system
External V2X control (Bidirectional Power Transfer)
External V2X control (with ISO 15118-20)
ISO 15118 support (2.0.1 / 2.1) This certification profile covers both ISO 15118-2 (2.0.1 and 2.1) as well as ISO 15118-
20 (2.1).
ISO 15118 Certificate Management:
Update Charging Station Certificate
(Contract) Certificate Installation / Update EV
Get Certificate Status
Install V2G / MO / OEMRoot CA certificates
Retrieve V2G / MO certificates from Charging Station
Delete a certificate from a Charging Station
ISO 15118 EIM / PnC Authorization:
Authorization using External Identification Means
Authorization using Contract Certificates
ISO 15118 Smart Charging:
Set charging profile
Remote start transaction with charging profile
Get Composite Schedule
Get Charging Profile
Clear Charging Profile
Renegotiating a Charging Schedule
ISO 15118 signed meter values
Bidirectional Power Transfer (2.1) Central & Local V2X control
Frequency support
V2X Authorisation - ISO15118-20
Idle, minimizing energy consumption
Prerequisite for this certification profile is supporting the Certification profile Smart
Charging.
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 6/68 Part 5 - Certification Profiles

Certification Profile Description
DER control (2.1) Starting a V2X session with DER control
Configure DER control settings at CS
Charging station reporting a DER event
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 7/68 Part 5 - Certification Profiles


### Chapter 3. Features

*_Source: Page 10 - 16_*

Chapter 3. Features
The concept of certification profiles is not enough to cover the variety in OCPP implementations. The OCPP specification contains
many optional features, often in the form of optional message fields or configuration variables, that a vendor can use to support
advanced functions. Whereas the certification profiles determine which OCPP functionality is implemented, the features describe
how much of a certain functionality in a profile has been implemented.
A Test System uses the features to determine which test cases have to be executed for a charging station or CSMS. For example,
the set of TxStartPoints that a charging station supports, has a big impact on the execution of certain test cases. The behavior of a
charging station that starts a transaction based on a successful authorization is different from a charging station that starts a
transaction as soon as a cable is connected. Similarly, a CSMS that only controls DC fast chargers will not need functionality to
unlock a cable at the charging station. For such a CSMS the vendor may decide to not implement the feature Support for unlocking
connector.
In most cases a feature corresponds the existence of a configuration variable or its value.
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 8/68 Part 5 - Certification Profiles

3.1. Optional feature list for charging station
The following table lists the optional features. These are features that are not mandatory to implement within a certification profile.
Where applicable the associated configuration variable is mentioned in parenthesis.
Table 2. Optional features for charging stations
Id Feature Charging Station
Core
C-01 Support for offline authorization of transactions Conditional.
Supporting this feature depends
on whether at least one of the
following feature combinations
is supported;
- Certification Profile: Local
Authorization List Management
AND at least one of the
following local authorization
options; C-30 or C-31 or C-32
- C-02: Support for allowing
offline authorization for
unknown ids AND at least one
of the following local
authorization options; C-30 or
C-31 or C-32 or C-33 or C-34
- C-49: Authorization Cache
AND at least one of the
following local authorization
options; C-30 or C-31 or C-32
- C-35: Local Authorization -
NoAuthorization - start
(Because there is no
authorization, no local
authorization mechanism is
needed.)
C-02 Support for allowing Offline Authorization for Unknown Ids
(OfflineTxForUnknownIdEnabled)
Optional
C-03 Support for maximizing energy for invalid ids (MaxEnergyOnInvalidId) Optional
C-04 Support to limit StatusNotifications (MinimumStatusDuration) Optional
C-06 Authorization status after cable disconnected on EV side
(StopTxOnEVSideDisconnect)
(At least one of the suboptions
below is required)
C-06.1 Support for maintaining authorization when cable disconnected on EV side Optional
C-06.2 Support for not maintaining authorization when cable disconnected on EV side Optional
C-07 Support for using a Master Pass for charging stations with UI
(MasterPassGroupId)
Optional
C-08 Support for using a Master Pass for charging stations without UI
(MasterPassGroupId)
Optional
C-09 Supported Transaction Start points (TxStartPoint) (At least one of the suboptions
below is required)
C-09.1 Start transaction options - EVConnected Optional
C-09.2 Start transaction options - Authorized Optional
C-09.3 Start transaction options - DataSigned Optional
C-09.4 Start transaction options - PowerPathClosed Optional
C-09.5 Start transaction options - EnergyTransfer Optional
C-09.6 Start transaction options - ParkingBayOccupancy Optional
C-10 Supported Transaction Stop points (TxStopPoint) (At least one of the suboptions
below is required)
C-10.1 Stop transaction options - EVConnected Optional
C-10.2 Stop transaction options - Authorized Optional
C-10.3 Stop transaction options - PowerPathClosed Optional
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 9/68 Part 5 - Certification Profiles

Id Feature Charging Station
C-10.4 Stop transaction options - EnergyTransfer Optional
C-10.5 Stop transaction options - ParkingBayOccupancy Optional
C-12 Unlocking of connector when cable disconnected on EV side
(UnlockOnEVSideDisconnect)
(At least one of the suboptions
below is required)
C-12.1 Support for unlocking connector when cable disconnected on EV side Optional
C-12.2 Support for not unlocking when cable disconnected on EV side Optional
C-13 Support for Reset per EVSE (AllowReset) Optional
C-14 Support for retrieving / deleting CustomerInformation - CustomerIdentifier Optional
C-20 Allowing New Sessions Pending a FirmwareUpdate
(AllowNewSessionsPendingFirmwareUpdate )
Optional
C-21 Support for queuing all or only Transaction related messages until they are
delivered to the CSMS (QueueAllMessages)
Optional
Time related settings
C-23 Supported time sources (TimeSource) { list } at least Heartbeat
C-25 Support for setting a TimeOffset (TimeOffset) Optional
C-26 Support for setting the TimeZone (TimeZone) Optional
C-28 Toggle sending clock aligned meter values when a transaction is ongoing / Idle
(AlignedDataSendDuringIdle)
Optional
C-29 TriggerMessage (Select all supported
suboptions)
C-29.1 Trigger message - MeterValues Optional
C-29.2 Trigger message - TransactionEvent Optional
C-29.3 Trigger message - LogStatusNotification Optional
C-29.4 Trigger message - FirmwareStatusNotification Optional
C-29.5 Trigger message - StatusNotification Optional
C-29.6 Trigger message - BootNotification Optional
C-29.7 (2.1) Trigger message - CustomTrigger Optional
Authorization options for local start
C-30 Authorization - using RFID ISO14443 Optional
C-31 Authorization - using RFID ISO15693 Optional
C-32 Authorization - using KeyCode Optional
C-33 Authorization - using locally generated id Optional
C-34 Authorization - MacAddress Optional
C-35 Authorization - NoAuthorization Optional
Authorization options for remote start (mandatory to support at least one)
C-36 Authorization - using RFID ISO14443 Optional
C-37 Authorization - using RFID ISO15693 Optional
C-38 Authorization - using centrally, in the CSMS (or other server) generated id Optional
C-39 Authorization - NoAuthorization Optional
C-40 Supported MeterValue Measurands (Please fill in the following
fields)
C-40.1 SampledTxStartedMeasurands { list of supported } at least one
C-40.2 SampledTxUpdatedMeasurands { list of supported } at least one
C-40.3 SampledTxEndedMeasurands { list of supported } at least one
C-40.4 AlignedDataMeasurands { list of supported } at least one
C-40.5 AlignedDataTxEndedMeasurands { list of supported } at least one
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 10/68 Part 5 - Certification Profiles

Id Feature Charging Station
C-41 Supported Cipher Suites { list of cipher suites } → at
least one of
TLS_ECDHE_ECDSA_WITH_AES
_128_GCM_SHA256
TLS_ECDHE_ECDSA_WITH_AES
_256_GCM_SHA384
OR
TLS_RSA_WITH_AES_128_GCM
_SHA256
TLS_RSA_WITH_AES_256_GCM
_SHA384
C-42 Signed Metervalues (SampledDataSignReadings) Optional
C-43 Install Firmware with ongoing transaction(s) Optional
C-47 Support for falling back to default OCPP reconnection mechanism when
NetworkConnection profile connection has failed
Optional
C-48 Authorization of remote start (AuthorizeRemoteStart) (At least one of the suboptions
below is required)
C-48.1 Option for authorization in case of a remote start Optional
C-48.2 Option for no authorization in case of a remote start Optional
C-58 Option for disabling remote authorization (DisableRemoteAuthorization) Optional
C-49 Authorization Cache (AuthCacheEnabled) Optional
C-59 Option for disabling remote authorization for cached invalid idTokens
(AuthCacheDisablePostAuthorize)
Optional
C-51 Configurable TxStartPoint Optional
C-52 Configurable TxStopPoint Optional
C-53 Support for lifetime cached token (AuthCacheLifeTime) Optional
C-54 Supported policies for replacing cached entries (AuthCachePolicy) { list of supported } at least one
C-56 Support for providing the SummaryInventory Optional
C-57 Support for cancelling ongoing log file upload Optional
C-60 Support for cancelling ongoing firmware update Optional
C-61 Security Profile 1 - Unsecured Transport with Basic Authentication Optional
C-62 (2.1) Support for resuming transactions (ImmediateAndResume) Optional
C-63 (2.1) Support for transaction limits Optional
C-64 (2.1) Support for resuming transaction after interruption Optional
C-65 (2.1) Support for DataCollectorLog Optional
Authorization options for local stop
C-70 Authorization - using RFID ISO14443 Optional
C-71 Authorization - using RFID ISO15693 Optional
C-72 Authorization - using KeyCode Optional
C-75 Authorization - NoAuthorization Optional
Reservation
R-0 Support for Reservation Optional
R-1 Support for reservations of connectorType Conditional.
Supporting this feature depends
on whether at least one
connectorType is supported
that is part of the
ConnectorEnumType list from
part 2 specification.
R-2 Support for reservation of unspecified EVSE (ReservationNonEvseSpecific) Optional
R-3 Support for disabling Reservations (ReservationEnabled) Optional
Advanced Device Management
DM-0 Support for Advanced Device Management Optional
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 11/68 Part 5 - Certification Profiles

Id Feature Charging Station
DM-3 Queue notifyEventRequest messages for specific severities
(OfflineMonitoringEventQueuingSeverity)
Optional
Local Authorization List Management
LA-0 Support for Local Authorization List Management Optional
LA-1 Authorization list support (LocalAuthListEnabled) Optional
LA-3 Option for disabling remote authorization for invalid idTokens stored at the Local
Authorization List (LocalAuthListDisablePostAuthorize)
Optional
Advanced User Interface
UI-0 Support for Advanced User Interface Optional
UI-1 Supported message priorities (DisplayMessageSupportedPriorities) (At least one of the suboptions
below is required, if UI-0 is
supported)
UI-1.1 AlwaysFront Optional
UI-1.2 InFront Optional
UI-1.3 NormalCycle Optional
UI-2 Supported message formats (DisplayMessageSupportedFormats) (At least one of the suboptions
below is required, if UI-0 is
supported)
UI-2.1 ASCII Optional
UI-2.2 HTML Optional
UI-2.3 URI Optional
UI-2.4 UTF8 Optional
UI-3 (2.1) Multi-language support Optional, based on AQ-20
Payment (2.1)
P-0 Support for Payment Optional
P-1 Support for Tariff conditions Optional
P-2 Supported Payment options (At least one of the suboptions
below is required, if P-0 is
supported)
P-2.1 Payment by prepaid card Optional
P-2.2 Integrated payment terminal Optional
P-2.3 Stand alone payment terminal Optional
P-2.4 QR code payment Optional
Advanced Security
AS-2 Additional root certificate check mechanism implemented
(AdditionalRootCertificateCheck)
Optional
AS-3 Update Charging Station Certificate - CertificateSignedRequest Timeout
(CertSigningWaitMinimum,CertSigningRepeatTimes)
Optional
AS-4 Security downgrades from profile 3 to 2 (AllowSecurityProfileDowngrade) (At least one of the suboptions
below is required)
AS-4.1 Support for disallowing security downgrades from profile 3 to 2 Optional
AS-4.2 Support for allowing security downgrades from profile 3 to 2 Optional
Smart Charging (2.0.1 / 2.1)
SC-2 Supported charging rate units (ChargingScheduleChargingRateUnit) (Select all supported
suboptions)
SC-2.1 A Optional
SC-2.2 W Optional
SC-3 (2.1) Support for limiting based on SoC (limitAtSoC) Optional
SC-5 (2.1) Support for using local time (useLocalTime) (At least one of the suboptions
below is required)
SC-5.1 TimeOffset Optional
SC-5.2 TimeZone Optional
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 12/68 Part 5 - Certification Profiles

Id Feature Charging Station
SC-6 (2.1) Support for using priority charging (PriorityCharging) Optional
SC-7 (2.1) Support for using randomized delays (randomizedDelay) Optional
SC-8 (2.1) Support for dynamic charging profiles Optional
SC-9 (2.1) Support for operationMode Idle Optional
SC-9.1 (2.1) Support for operationMode Idle with EvseSleep Optional
EMS Control (2.1)
SC-10 (2.1) Support for EMS Control Optional
ISO 15118 support (2.0.1 / 2.1)
ISO-1 (2.1) Supported ISO 15118 version (At least one of the suboptions
below is required)
ISO-1.1 ISO 15118-2 Optional, required if ISO-1.2 is
supported
ISO-1.2 ISO 15118-20 Optional
ISO-4 Support for retrieving / deleting CustomerInformation - CustomerCertificate Optional
ISO-5 Charging Station can provide a contract certificate that it cannot validate to the
CSMS (CentralContractValidationAllowed)
Optional
ISO-6 (2.1) Support for ServiceRenegotiation Optional
Bidirectional Power Transfer (2.1)
BPT-1 Frequency support Optional
BPT-2 Support for local loadbalancing (LocalLoadBalancing) Optional
DER control (2.1)
No optional features for this profile
3.2. Optional feature list for CSMS
The features of a CSMS are not determined by configuration variables. Features in the list below are allowed not to be supported by
a CSMS.
Table 3. Optional features for CSMS
Id Feature CSMS
Core
C-11 Support for unlocking connector for charging station with detachable cable
(UnlockConnector message).
Optional
C-13 Support for Reset per EVSE Optional
C-14 Support for retrieving / deleting CustomerInformation - CustomerIdentifier Optional
C-15 Support for scheduled firmware updates Optional
C-16 Support for checking the TransactionStatus Optional
C-17 Support for retrieving the ConfigurationInventory Optional
C-29 TriggerMessage (Select all supported
suboptions)
C-29.1 Trigger message - MeterValues Optional
C-29.2 Trigger message - TransactionEvent Optional
C-29.3 Trigger message - LogStatusNotification Optional
C-29.4 Trigger message - FirmwareStatusNotification Optional
C-29.5 Trigger message - StatusNotification Optional
C-29.7 (2.1) Trigger message - CustomTrigger Optional
Authorization options for local start
C-30 Authorization - using RFID ISO14443 Required
C-31 Authorization - using RFID ISO15693 Required
C-32 Authorization - using KeyCode Optional
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 13/68 Part 5 - Certification Profiles

Id Feature CSMS
C-33 Authorization - using locally generated id Optional
C-34 Authorization - MacAddress Optional
C-35 Authorization - NoAuthorization Required
Authorization options for remote start (mandatory to support at least one)
C-36 Authorization - using RFID ISO14443 Required
C-37 Authorization - using RFID ISO15693 Required
C-38 Authorization - using centrally, in the CSMS (or other server) generated id Optional
C-39 Authorization - NoAuthorization Optional
C-44 Support for sending a BootNotification Pending before Accepting Optional
C-45 Support for Multiple elements GetVariablesRequest Optional
C-46 Support for Multiple elements SetVariablesRequest Optional
C-50 GetBaseReport - FullInventory (At least one of the suboptions
below is required)
C-50.1 GetBaseReport - FullInventory - During onboarding Optional
C-50.2 GetBaseReport - FullInventory - Manual trigger Optional
C-61 Security Profile 1 - Unsecured Transport with Basic Authentication Optional
C-65 (2.1) Support for DataCollectorLog Optional
C-76 (2.1) Support for Battery Swapping Stations Optional
Reservation
R-0 Support for Reservation Optional
R-1 Support for reservations of connectorType Optional
R-2 Support for reservations of unspecified EVSE Optional
Local Authorization List Management
LA-0 Support for Local Authorization List Management Optional
LA-2 Support for GetLocalListVersion Optional
Advanced Device Management
DM-0 Support for Advanced Device Management Optional
Advanced User Interface
UI-0 Support for Advanced User Interface Optional
Payment (2.1)
P-0 Support for Payment Optional
Advanced Security
No optional features for this profile
Smart Charging (2.0.1 / 2.1)
SC-3 (2.1) Support for limiting based on SoC (limitAtSoC) Optional
SC-4 Support for TxDefaultProfile on EVSEID #0 Optional
SC-6 (2.1) Support for using priority charging (PriorityCharging) Optional
SC-7 (2.1) Support for using randomized delays (randomizedDelay) Optional
SC-10 (2.1) Support for EMS Control Optional
ISO 15118 support (2.0.1 / 2.1)
ISO-4 Support for retrieving / deleting CustomerInformation - CustomerCertificate Optional
Bidirectional Power Transfer (2.1)
BPT-1 Frequency support Optional
DER control (2.1)
No optional features for this profile
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 14/68 Part 5 - Certification Profiles


### Chapter 4. List of test cases

*_Source: Page 17 - 63_*

Chapter 4. List of test cases
4.1. Introduction
This table lists the test cases that are part of the OCPP Certification program. For each of the test cases, the columns "Conf. Test for Charging Station" and "Conf. Test for CSMS" indicate
whether the test case is mandatory or not within a Certification Profile. The abbreviations have the following meaning:
• M = Mandatory . This means that IF you implement the certification profile this test case belongs to, you MUST successfully pass this test case.
• C = Conditional. This means that IF you meet a condition, you MUST pass this test case. Most conditions refer to the optional features that are listed in the Features.
4.2. Test Cases Core
TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
Basic Authentication
TC_A_01 Valid username/password combination M M
TC_A_02 Username does not equal ChargingStationId M
TC_A_03 Invalid password M
Update Charging Station Password for HTTP
Basic Authentication
TC_A_09 Accepted M M
TC_A_10 Rejected M M
TLS - server-side certificate
TC_A_04 Valid certificate M M
TC_A_05 Invalid certificate M
TC_A_06 TLS version too low M M
Upgrade Charging Station Security Profile
TC_A_19 Accepted C C C-61 OR Advanced
Security
TC_A_20 No valid CSMSRootCertificate installed C If the last CSMSRootCertificate can be
removed and Security Profile 1 is supported.
AQ-1 and C-61 Can the last CSMSRootCertificate
be removed?
Security Profile 1 - Unsecured
Transport with Basic
Authentication
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 15/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_A_22 Downgrade security profile - Rejected M
Cold Boot Charging Station
TC_B_01 Accepted M M
TC_B_02 Pending M C CSMS: If Pending mechanism is implemented C-44 BootNotification Pending
TC_B_03 Rejected M
TC_B_30 Pending/Rejected - SecurityError M C For CSMS: if CSMS can be configured to first
respond to a BootNotificationRequest with
status Pending or Rejected
C-44 or NOT AQ-16 BootNotification Pending or
Does the CSMS reject unknown
Charging Stations during
websocket connection setup?
TC_B_31 Pending/Rejected - TriggerMessage C For CSMS: if CSMS can be configured to first
respond to a BootNotificationRequest with
status Pending or Rejected
C-44 or NOT AQ-16 BootNotification Pending or
Does the CSMS reject unknown
Charging Stations during
websocket connection setup?
Status change during offline period
TC_B_51 > Offline Threshold M
TC_B_52 < Offline Threshold M
Get Variables
TC_B_06 single value M M
TC_B_07 multiple values M C If the CSMS supports multiple elements in a
GetVariablesRequest
C-45 multiple values elements
GetVariablesRequest
TC_B_32 Unknown component M
TC_B_33 Unknown variable M
TC_B_34 Not supported attribute type M
Set Variables
TC_B_09 single value M M
TC_B_10 multiple values M C If the CSMS supports multiple elements in a
SetVariablesRequest
C-46 multiple values elements
SetVariablesRequest
TC_B_35 Unknown component M
TC_B_36 Unknown variable M
TC_B_37 Not supported attribute type M
TC_B_11 invalidly formatted values M
TC_B_39 Read-only M
Get Base Report
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 16/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_B_12 ConfigurationInventory M C C-17 OR DM-0 ConfigurationInventory
TC_B_13 FullInventory M C C-50.2 GetBaseReport - FullInventory -
Manual trigger
TC_B_14 SummaryInventory C C-56 OR DM-0
TC_B_15 Not Supported base report C For CS: If reportBase SummaryInventory is not
supported. This is the case when Certification
Profile Advanced Device Management is not
supported.
Not C-56
TC_B_53 Test mandatory DM variables via FullInventory M
Reset Charging Station
TC_B_20
(2.1)
Without ongoing transaction - OnIdle C M NOT HFS-13
TC_B_21
(2.1)
With Ongoing Transaction - OnIdle C M NOT HFS-13
TC_B_22 With Ongoing Transaction - Immediate M M
TC_B_23 Unavailable persists reset M
TC_B_41
(2.1)
With multiple ongoing transactions - OnIdle C For CS: if no. of EVSEs > 1 HFS-8 > 1 AND NOT
HFS-13
Reset EVSE
TC_B_25 Without ongoing transaction C C C-13 Reset per EVSE
TC_B_26 With Ongoing Transaction - OnIdle C C C-13 Reset per EVSE
TC_B_27 With Ongoing Transaction - Immediate C C C-13 Reset per EVSE
TC_B_28 Not Supported C For CS: Charging Station does not support
resetting an individual EVSE
NOT C-13 Reset per EVSE
TC_B_29 With ongoing transaction - Not Supported C For CS: Charging Station does not support
resetting an individual EVSE
NOT C-13 Reset per EVSE
Reset ImmediateAndResume
TC_B_101
(2.1)
With Ongoing Transaction -
TxResumptionTimeout 0 or <omitted> -
Rejected
M
TC_B_102
(2.1)
With ongoing transaction - Energy Transfer
Suspended
C C-62 Support for resuming transactions
TC_B_103
(2.1)
With Ongoing Transaction - Resuming Energy
Transfer
C M C-62 Support for resuming transactions
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 17/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_B_104
(2.1)
Without ongoing transaction C M C-62 Support for resuming transactions
Set new NetworkConnectionProfile
TC_B_42 Accepted M
TC_B_43 Rejected M
TC_B_44 Failed M
Set new NetworkConnectionProfile (2.1)
TC_B_100
(2.1)
Identity and password M M
TC_B_105
(2.1)
Add new NetworkConfiguration using
SetVariables
M M
TC_B_107
(2.1)
Add and remove slot from
NetworkConfigurationPriority
M
TC_B_108
(2.1)
Prevent overwriting configured Network Profile
slot
M
TC_B_109
(2.1)
When changing
SecurityCtrlr.Identity/BasicAuthPassword the
NetworkProfiles.Identity/BasicAuthPassword
must be cleared
M
TC_B_110
(2.1)
No security downgrade to profile #1 M
TC_B_111
(2.1)
No security downgrade to profile #1 - DM M
Migrate to new ConnectionProfile
TC_B_45 Success - Same CSMS Root M For CS: at least two configuration slots for
networkConnectionProfiles must be supported
TC_B_46 Fallback mechanism - Same CSMS Root M For CS: at least two configuration slots for
networkConnectionProfiles must be supported
TC_B_47 Fallback after
NetworkProfileConnectionAttempts per
NetworkConfigurationPriority failed - New
CSMS Root - New CSMS
C For CS: at least two configuration slots for
networkConnectionProfiles must be supported
AS-2 and C-47 Additional Root Certificate check
mechanism implemented &
Reconnect after
NetworkProfileConnectionAttempt
s
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 18/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_B_49 Fallback after
NetworkProfileConnectionAttempts per
NetworkConfigurationPriority failed - Same
CSMS Root
C For CS: at least two configuration slots for
networkConnectionProfiles must be supported
C-47 Reconnect after
NetworkProfileConnectionAttempt
s
TC_B_50 Success - New CSMS Root - New CSMS M For CS: at least two configuration slots for
networkConnectionProfiles must be supported
Network Reconnection
TC_B_57 After connection loss M
TC_B_58 WebSocket Subprotocol negotiation M
Local start transaction
TC_C_02 Authorization Invalid/Unknown C M Charging Station:
- The Charging Station supports at least one of
the following local start authorization options
C-30, C-31, C-32
- The Charging Station does NOT have a cable
lock that prevents the EV driver to connect the
EV and EVSE before authorization.
(C-30 or C-31 or C-32)
and NOT AQ-2
Local Authorization - using RFID
ISO14443 / RFID ISO15693 /
KeyCode and
Does the Charging Station have a
cable lock, which prevents the EV
driver to connect the EV and EVSE
before authorization?
TC_C_06 Authorization Blocked C M For CS:
- The Charging Station supports at least one of
the following local start authorization options:
C-30, C-31, C-32
- The Charging Station does NOT have a cable
lock, which prevents the EV driver to connect
the EV and EVSE
before authorization.
NOT AQ-2 and (C-30 or
C-31 or C-32)
Local Authorization - using RFID
ISO14443 / RFID ISO15693 /
KeyCode
TC_C_07 Authorization Expired C M For CS:
- The Charging Station supports at least one of
the following local start authorization options :
C-30, C-31, C-32
- The Charging Station does NOT have a cable
lock, which prevents the EV driver to connect
the EV and EVSE
before authorization.
NOT AQ-2 and (C-30 or
C-31 or C-32)
Local Authorization - using RFID
ISO14443 / RFID ISO15693 /
KeyCode
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 19/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_E_38
(2.1)
EV not ready C M For CS: not supporting start transaction
options EnergyTransfer
NOT C-09.5 and NOT
Product Subtype
"Mode 1/2-only
Charging Station" AND
NOT HFS-13
Start transaction options -
EnergyTransfer
TC_C_56 Authorization Unknown C Charging Station:
- The Charging Station supports at least one of
the following local start authorization options:
C-30, C-31, C-32
(C-30 or C-31 or C-32)
and NOT AQ-2
Local Authorization - using RFID
ISO14443 / RFID ISO15693 /
KeyCode
TC_C_05 Authorization invalid - Cable lock C For CS:
- The Charging Station has a cable lock, which
prevents the EV driver to connect the EV and
EVSE before authorization.
- The Charging Station supports at least one of
the following local start authorization options
C-30, C-31, C-32
- The Charging Station does NOT have the
following configuration: TxStartPoint ReadOnly
AND value Authorized is NOT set.
(C-30 or C-31 or C-32)
and AQ-2
Local Authorization - using RFID
ISO14443 / RFID ISO15693 /
KeyCode
TC_C_100
(2.1)
Authorization first - Cable plugin timeout C For CS:
- The Charging Station supports at least one of
the following local start authorization options
C-30, C-31, C-32
- The Charging Station does NOT have the
following configuration: TxStartPoint ReadOnly
AND value Authorized is NOT set.
(C-30 or C-31 or C-32) Local Authorization - using RFID
ISO14443 / RFID ISO15693 /
KeyCode
Local Stop Transaction
TC_C_04 Different idToken C Charging Station:
- The Charging Station supports at least one of
the following local start authorization options:
C-70, C-71, C-72
C-70 or C-71 or C-72 Local Authorization - using RFID
ISO14443 / RFID ISO15693 /
KeyCode
TC_E_06 Accepted C The Charging Station supports E07 Transaction
locally stopped by IdToken with at least one of
the following local start authorization options:
C-70, C-71, C-72, C-75
C-70 or C-71 or C-72 or
C-75
Local Authorization - using RFID
ISO14443 / RFID ISO15693 /
KeyCode / NoAuthorization
Authorization by GroupId
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 20/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_C_39 Success C M For CS: the Charging Station supports at least
one of the following local start authorization
options: C-30, C-31, C-32
(C-30 and C-70) or (C-
31 and C-71) or (C-32
and C-72)
Local Authorization - using RFID
ISO14443 / RFID ISO15693 /
KeyCode
TC_C_45 Master pass - Not able to start transaction +
groupId
C For CS: the Charging Station supports at least
one of the following local start authorization
options: C-30, C-31, C-32 and Master Pass
((C-30 and C-70) or (C-
31 and C-71) or (C-32
and C-72)) AND (C-07
OR C-08)
Local Authorization - using RFID
ISO14443 / RFID ISO15693 /
KeyCode and Master Pass
TC_C_42 Not stopped by GroupId C For CS: the Charging Station supports at least
one of the following local start authorization
options: C-30, C-31, C-32
(C-30 and C-70) or (C-
31 and C-71) or (C-32
and C-72)
Local Authorization - using RFID
ISO14443 / RFID ISO15693 /
KeyCode
Offline Authorization
TC_C_26 Unknown Id C If the feature Unknown Offline Authorization is
supported AND the Charging Station supports
at least one of the following local start
authorization options: C-30, C-31, C-32, C-33, C-
34
C-02 and (C-30 or C-31
or C-32 or C-33 or C-
34)
Unknown Offline Authorization
Stop Transaction with a Master Pass
TC_C_47 With UI - All transactions C M CS: If the feature Master Pass with UI is
supported AND the Charging Station supports
at least one of the following local start
authorization options: C-70, C-71, C-72
C-07 and (C-70 or C-71
or C-72)
Master Pass - With UI
TC_C_48 With UI - With UI - Specific transactions C M CS: If the feature Master Pass with UI is
supported AND the Charging Station supports
at least one of the following local start
authorization options: C-70, C-71, C-72
C-07 and (C-70 or C-71
or C-72)
Master Pass - With UI
TC_C_49 Without UI C M CS: If the feature Master Pass with UI is
supported AND the Charging Station supports
at least one of the following local start
authorization options: C-70, C-71, C-72
C-08 and (C-70 or C-71
or C-72)
Master Pass - Without UI
Store Authorization Data in the Authorization
Cache
TC_C_32 Persistent over reboot C If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32 OR
supports at least one of the following remote
start authorization options: C-36, C-37
C-49 and (((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72)) or
(C-36 or C-37))
Authorization Cache
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 21/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_C_33 Update on AuthorizeResponse C If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32 OR
supports at least one of the following remote
start authorization options: C-36, C-37
C-49 and (((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72)) or
(C-36 or C-37))
Authorization Cache
TC_C_34 Update on TransactionResponse C If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32 OR
supports at least one of the following remote
start authorization options: C-36, C-37
C-49 and (((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72)) or
(C-36 or C-37))
Authorization Cache
TC_C_36 AuthCacheCtrlr.LocalPreAuthorize = false C If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32 OR
supports at least one of the following remote
start authorization options: C-36, C-37
C-49 and (((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72)) or
(C-36 or C-37))
Authorization Cache
TC_C_46 AuthCacheLifeTime C If the Charging Station has an authorization
cache AND supports to set a lifetime for its
entries AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32 OR
supports at least one of the following remote
start authorization options: C-36, C-37
C-49 and C-53 and (((C-
30 and C-70) or (C-31
and C-71) or (C-32 and
C-72)) or (C-36 or C-
37))
Authorization Cache &
AuthCacheLifeTime
Clear Authorization Data in Authorization
Cache
TC_C_37 Accepted C M If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32 OR
supports at least one of the following remote
start authorization options: C-36, C-37
C-49 and (((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72)) or
(C-36 or C-37))
Authorization Cache
TC_C_38 Rejected C M If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32 OR
supports at least one of the following remote
start authorization options: C-36, C-37
C-49 and (((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72)) or
(C-36 or C-37))
Authorization Cache
Authorization by GroupId
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 22/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_C_41 Success with Authorization Cache C For CS:
- The Charging Station supports at least one of
the following local start authorization options:
C-30, C-31, C-32
- If the Charging Station has an authorization
cache.
C-49 and ((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72))
Authorization Cache
TC_C_44 Invalid status with Authorization Cache C For CS:
- The Charging Station supports at least one of
the following local start authorization options:
C-30, C-31, C-32
- If the Charging Station has an authorization
cache.
C-49 and ((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72))
Authorization Cache
Authorization through authorization cache
TC_C_08 Accepted C M If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32 OR
supports at least one of the following remote
start authorization options: C-36, C-37
C-49 and (((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72)) or
(C-36 or C-37))
Authorization Cache
TC_C_09 Invalid & Not Accepted C If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32 OR
supports at least one of the following remote
start authorization options: C-36, C-37
C-49 and (((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72)) or
(C-36 or C-37))
Authorization Cache
TC_C_12 Invalid & Accepted C If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32 OR
supports at least one of the following remote
start authorization options: C-36, C-37
C-49 and (((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72)) or
(C-36 or C-37))
Authorization Cache
TC_C_10 Blocked C If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32 OR
supports at least one of the following remote
start authorization options: C-36, C-37
C-49 and (((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72)) or
(C-36 or C-37))
Authorization Cache
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 23/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_C_11 Expired C If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32 OR
supports at least one of the following remote
start authorization options: C-36, C-37
C-49 and (((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72)) or
(C-36 or C-37))
Authorization Cache
TC_C_13 Accepted but cable not connected yet. C If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32 OR
supports at least one of the following remote
start authorization options: C-36, C-37
C-49 and (((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72)) or
(C-36 or C-37))
Authorization Cache
TC_C_15 StopTxOnInvalidId = false,
MaxEnergyOnInvalidId > 0
C If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32
If MaxEnergyOnInvalidId is implemented.
C-49 and C-03 and ((C-
30 and C-70) or (C-31
and C-71) or (C-32 and
C-72))
Authorization Cache &
MaxEnergyOnInvalidId
TC_C_16 StopTxOnInvalidId = true C If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32
C-49 and ((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72))
Authorization Cache
TC_C_17 StopTxOnInvalidId = false C If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32
C-49 and ((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72))
Authorization Cache
TC_C_18 StopTxOnInvalidId = true,
MaxEnergyOnInvalidId > 0
C If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32
If MaxEnergyOnInvalidId is implemented.
C-49 and C-03 and ((C-
30 and C-70) or (C-31
and C-71) or (C-32 and
C-72))
Authorization Cache &
MaxEnergyOnInvalidId
TC_C_20 Invalid M
TC_C_57 AuthCacheDisablePostAuthorize C If the Charging Station supports the option for
disabling remote authorization for cached
invalid idTokens AND has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32, C-34 OR
supports at least one of the following remote
start authorization options: C-36, C-37
C-59 and C-49 and (((C-
30 and C-70) or (C-31
and C-71) or (C-32 and
C-72)) or (C-36 or C-
37))
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 24/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
Local start transaction - Cable plugin first
TC_E_03 Success C M Applicable if one or more of the local start
authorization options is implemented.
NOT AQ-2 and (C-30 or
C-31 or C-32 or C-33 or
C-34 or C-35 or ISO
15118 support)
Authorization options for local start
Local start transaction - Authorization first
TC_E_04 Success C M Applicable if one or more of the local start
authorization options is implemented.
C-30 or C-31 or C-32 or
C-33 or C-35
Authorization options for local start
TC_E_05
(2.1)
Cable plugin timeout C Applicable if one or more of the local start
authorization options is implemented.
(C-30 or C-31 or C-32
or C-33 or C-35) AND
NOT HFS-13
Authorization options for local start
TC_E_52 DisableRemoteAuthorization C If the Charging Station supports the option for
disabling remote authorization and
The Charging Station supports at least one of
the following local start authorization options
C-30, C-31, C-32 and
Either Authorization Cache or Local
Authorization List is supported.
C-58 and (C-30 or C-31
or C-32) and (C-49 or
Local Authorization
List Management)
Local Authorization - using RFID
ISO14443 / RFID ISO15693 /
KeyCode & Authorization Cache &
Local Authorization List.
Start transaction options
TC_E_09 EVConnected C M TxStartPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value EVConnected
is a supported value. And it should be possible
to not set ParkingBayOccupancy.
(C-09.1 and (C-51 or
NOT C-09.6)) and NOT
AQ-2
TC_E_10 Authorized - Local C M TxStartPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value Authorized is
a supported value.
- If one or more of the local start authorization
options is implemented.
C-09.2 and (C-30 or C-
31 or C-32 or C-33 or C-
34 or C-35 or ISO
15118 support)
Supported Transaction Start Points
& Authorization options for local
start & Authorization - eMAID
TC_E_13 Authorized - Remote C TxStartPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value Authorized is
a supported value.
C-09.2 Supported Transaction Start points
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 25/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_E_11 DataSigned C M CS: TxStartPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value DataSigned is
a supported value. And it should be possible to
not set ParkingBayOccupancy and
EVConnected and Authorized.
CSMS: Must at least be able to receive a signed
MeterValue. It does not need to be able to read
it.
C-09.3 and (C-51 or
NOT (C-09.1 or C-09.2
or C-09.6))
Supported Transaction Start points
TC_E_01 PowerPathClosed C M TxStartPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value
PowerPathClosed is a supported value. And it
should be possible to not set
ParkingBayOccupancy and EVConnected and
Authorized and DataSigned.
C-09.4 and (C-51 or
NOT (C-09.1 or C-09.2
or C-09.3 or C-09.6))
Supported Transaction Start points
TC_E_02 EnergyTransfer C M TxStartPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value
EnergyTransfer is a supported value. And it
should be possible to not set
ParkingBayOccupancy and EVConnected and
Authorized and DataSigned and
PowerPathClosed
C-09.5 and (C-51 or
NOT (C-09.1 or C-09.2
or C-09.3 or C-09.4 or
C-09.6))
Supported Transaction Start points
TC_E_12 ParkingBayOccupied C M TxStartPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value
ParkingBayOccupied is a supported value.
C-09.6 Supported Transaction Start points
Stop transaction options
TC_E_14
(2.1)
EVDisconnected - Charging Station side C M TxStopPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value EVConnected
is a supported value. And it should be possible
to not set EnergyTransfer and
PowerPathClosed and Authorized.
Charging Station does NOT have a fixed cable.
HFS-1 and C-10.1 and
(C-52 or NOT (C-10.2
or C-10.3 or C-10.4))
AND NOT HFS-13
Supported Transaction Stop points
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 26/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_E_20 EVDisconnected - EV side (able to charge IEC
61851-1 EV)
C M TxStopPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value EVConnected
is a supported value. And it should be possible
to not set EnergyTransfer and
PowerPathClosed AND
The Charging Station does NOT have following
configuration combination;
StopTxOnEVSideDisconnect mutability
ReadOnly with value true AND TxStopPoint
mutability is ReadOnly and contains Authorized
(C-10.1 AND (NOT
(NOT C-52 AND (C-10.3
or C-10.4))) AND NOT
(NOT C-06.1 AND NOT
C-52 AND C-10.2))
AND (AQ-9 OR Product
Subtype "Mode 1/2-
only Charging Station")
AND NOT Product
Subtype "Wireless
Charging Station"
Supported Transaction Stop points
TC_E_54
(2.1)
EVDisconnected - EV side (not able to charge
IEC 61851-1 EV)
C TxStopPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value EVConnected
is a supported value. And it should be possible
to not set EnergyTransfer and
PowerPathClosed AND
The Charging Station does NOT have following
configuration combination;
StopTxOnEVSideDisconnect mutability
ReadOnly with value true AND TxStopPoint
mutability is ReadOnly and contains Authorized
C-10.1 AND (NOT (NOT
C-52 AND (C-10.2 or C-
10.3 or C-10.4))) AND
(HFS-4 OR ISO 15118
support OR Product
Subtype "Wireless
Charging Station") AND
NOT Product Subtype
"Mode 1/2-only
Charging Station" AND
NOT HFS-13
Supported Transaction Stop points
TC_E_15 StopAuthorized - Local C M TxStopPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value Authorized is
a supported value.
The Charging Station supports E07 Transaction
locally stopped by IdToken with at least one of
the following local stop authorization options:
C-70, C-71, C-72, C75
C-10.2 and (C-70 or C-
71 or C-72 or C-75)
Supported Transaction Stop Points
& Local Authorization - using RFID
ISO14443 / RFID ISO15693 /
KeyCode / NoAuthorization
TC_E_21 StopAuthorized - Remote C M TxStopPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value Authorized is
a supported value.
C-10.2 Supported Transaction Stop points
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 27/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_E_16 Deauthorized - Invalid idToken C M TxStopPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value Authorized or
PowerPathClosed is a supported value.
Charging Station: If one or more of the local
start authorization options is implemented.
AND either a cache, local authorization list or
UnknownIdtag (C15) is implemented.
(C-10.2 or C-10.3) and
C-01 and NOT C-35
Supported Transaction Stop Points
& Local Authorization options for
local start
TC_E_17 Deauthorized - EV side disconnect C M This testcase is applicable if the value
Authorized is a supported value for
TxStopPoint AND
EVConnected, PowerPathClosed and
EnergyTransfer must not be set as TxStopPoint
AND
StopTxOnEVSideDisconnect true must be a
supported value.
C-10.2 and C-06.2 and
AQ-9 and NOT (NOT C-
52 AND (C-10.1 OR C-
10.3 OR C-10.4))
Supported Transaction Stop points
TC_E_39
(2.1)
Deauthorized - timeout C M NOT HFS-13
TC_E_07 PowerPathClosed - Local stop C M TxStopPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value
PowerPathClosed is a supported value. And it
should be possible to not set Authorized.
The Charging Station supports E07 Transaction
locally stopped by IdToken with at least one of
the following local stop authorization options:
C-70, C-71, C-72, C75
C-10.3 and (C-52 or
NOT C-10.2) and (C-70
or C-71 or C-72 or C-
75)
Supported Transaction Stop Points
& Local Authorization - using RFID
ISO14443 / RFID ISO15693 /
KeyCode / NoAuthorization
TC_E_35 PowerPathClosed - Remote stop C TxStopPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value
PowerPathClosed is a supported value. And it
should be possible to not set Authorized.
C-10.3 and (C-52 or
NOT C-10.2)
Supported Transaction Stop points
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 28/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_E_37 PowerPathClosed - EV side disconnect C TxStopPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value
PowerPathClosed is a supported value. And it
should be possible to not set EnergyTransfer
and EVConnected.
C-10.3 and (C-52 or
NOT (C-10.1 or C-
10.4)) AND (AQ-9 OR
Product Subtype
"Mode 1/2-only
Charging Station" OR
Product Subtype
"Wireless Charging
Station")
Supported Transaction Stop points
TC_E_08 EnergyTransfer stopped - StopAuthorized C M TxStopPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value
EnergyTransfer is a supported value. And it
should be possible to not set PowerPathClosed
and Authorized.
C-10.4 and (C-52 or
NOT (C-10.2 or C-
10.3))
Supported Transaction Stop points
TC_E_22 EnergyTransfer stopped - SuspendedEV C M TxStopPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value
EnergyTransfer is a supported value.
C-10.4 Supported Transaction Stop points
TC_E_19 ParkingBayUnoccupied C M TxStopPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value
ParkingBayOccupancy is a supported value.
And it should be possible to not set
EnergyTransfer and Authorized and
PowerPathClosed and EVConnected.
C-10.5 and (C-52 or
NOT (C-10.1 or C-10.2
or C-10.3 or C-10.4))
Supported Transaction Stop points
Disconnect cable on EV-side
TC_E_24 Deauthorize transaction -
UnlockOnEVSideDisconnect is true
C The Charging Station does NOT have a
permanently attached cable.
UnlockOnEVSideDisconnect can be set to true
StopTxOnEVSideDisconnect can be set to true
HFS-1 and C-06.2 and
C-12.1 and AQ-18 and
NOT (HFS-4 or (ISO
15118 support and
NOT AQ-9 ))
Support for not maintaining
authorization when cable
disconnected on EV side & Support
for unlocking connector when
cable disconnected on EV side
TC_E_25 Deauthorize transaction -
UnlockOnEVSideDisconnect is false
C UnlockOnEVSideDisconnect can be set to false
StopTxOnEVSideDisconnect can be set to true
C-06.2 and C-12.2 and
AQ-18 and NOT (HFS-4
or (ISO 15118 support
and NOT AQ-9 ))
Support for not maintaining
authorization when cable
disconnected on EV side & Support
for not unlocking connector when
cable disconnected on EV side
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 29/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_E_26 Suspend transaction C M TxStopPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value
ParkingBayOccupancy or Authorized is a
supported value. And it should be possible to
not set EnergyTransfer and PowerPathClosed
and EVConnected.
UnlockOnEVSideDisconnect can be set to false
StopTxOnEVSideDisconnect can be set to false
(C-10.2 or C-10.5) and
(C-52 or NOT (C-10.1
or C-10.3 or C-10.4))
and C-06.1 and C-12.2
and NOT (HFS-4 or
(ISO 15118 support
and NOT AQ-9 )) and
NOT Product Subtype
"Mode 1/2-only
Charging Station"
TC_E_27 Suspend transaction - Fixed cable connection
timeout
C TxStopPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value
ParkingBayOccupancy or Authorized is a
supported value. And it should be possible to
not set EnergyTransfer and PowerPathClosed
and EVConnected.
The Charging Station has a permanently
attached cable at the Charging Station side.
UnlockOnEVSideDisconnect can be set to false
StopTxOnEVSideDisconnect can be set to false
(C-10.2 or C-10.5) and
(C-52 or NOT (C-10.1
or C-10.3 or C-10.4))
and C-06.1 and C-12.2
and HFS-2 and NOT
(HFS-4 or (ISO 15118
support and NOT AQ-9
)) and NOT Product
Subtype "Mode 1/2-
only Charging Station"
Transactions with fixed cost, energy or time
TC_E_100
(2.1)
CSMS specifies energy limit C C-63 Support for transaction limits
TC_E_101
(2.1)
CSMS calculates costs (through
CostUpdatedRequest) and CS specifies limit
C C-63 Support for transaction limits
TC_E_102
(2.1)
CSMS and CS both specify limits C C CS: C-63
CSMS: P-0
CS: Support for transaction limits
CSMS: Support for Payment
TC_E_103
(2.1)
CS calculates costs and CSMS specifies limit C C-63 Support for transaction limits
TC_E_104
(2.1)
CSMS calculates costs (through
TransactionEventResponse) and specifies limit
C C-63 Support for transaction limits
TC_E_105
(2.1)
CSMS specifies time limit C C-63 Support for transaction limits
TC_E_106
(2.1)
CS specifies energy limit C M C-63 Support for transaction limits
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 30/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_E_107
(2.1)
CS specifies time limit C M C-63 Support for transaction limits
TC_E_108
(2.1)
CS calculates costs and specifies limit C M C-63 Support for transaction limits
TC_E_109
(2.1)
CSMS calculates costs and specifies cost limit C P-0 Support for Payment
TC_E_110
(2.1)
CSMS specifies energy limit C P-0 Support for Payment
TC_E_111
(2.1)
CSMS specifies time limit C P-0 Support for Payment
Resuming transaction after interruption
TC_E_112
(2.1)
TxResumptionTimeout not expired -
TxAllowEnergyTransferResumption is false
C C-64 Support for resuming transaction
after interruption
TC_E_113
(2.1)
TxResumptionTimeout not expired -
TxAllowEnergyTransferResumption is true
C M C-64 Support for resuming transaction
after interruption
TC_E_114
(2.1)
Powerloss - TxResumptionTimeout absent C NOT C-62 AND NOT C-
64
Support for resuming transaction
after interruption
TC_E_115
(2.1)
Powerloss - TxResumptionTimeout = 0 C C-64 Support for resuming transaction
after interruption
TC_E_116
(2.1)
Powerloss - TxResumptionTimeout expired C C-64 Support for resuming transaction
after interruption
Retry sending transaction message when
failed
TC_E_41 Max retry count reached M
TC_E_42 Success before reaching the max retry count M
TC_E_50 Max retry count reached - CallError M
TC_E_51 Success before reaching the max retry count -
CallError
M
Offline Behaviour
TC_E_40 Connection loss during transaction M
TC_E_43 Transaction during offline period C Charging Station: If offline authorization is
supported.
C-01 Offline transaction support
TC_E_44 Stop transaction during offline period C Charging Station: If one or more of the local
stop authorization options is implemented.
C-70 or C-71 or C-72 or
C-75 or ISO 15118
support
Local Authorization options for
local start & Authorization - eMAID
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 31/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_E_45 Stop transaction during offline period - Same
GroupId
C For CS: the Charging Station supports at least
one of the following local stop authorization
options: C-30, C-31, C-32 and Local
Authorization List or Authorization Cache
(C-70 or C-71 or C-72)
AND (Local
Authorization List
Management or C-49)
Local Authorization - using RFID
ISO14443 / RFID ISO15693 /
KeyCode and Local Authorization
List or Authorization Cache
Check Transaction status
TC_E_28 TransactionId unknown M
TC_E_29 Transaction with id ongoing - with message in
queue
M C C-16 Check TransactionStatus
TC_E_30 Transaction with id ongoing - without message
in queue
M C C-16 Check TransactionStatus
TC_E_31 Transaction with id ended - with message in
queue
C C Charging Station:
The following combination of conditions are
NOT true:
- No local authorization methods are supported
AND
- TxStopPoint mutability is ReadOnly and only
contains Authorized AND
- TxCtrlr.StopTxOnEVSideDisconnect mutability
is false and value is false
CSMS: C-16
CS:
NOT (
NOT ((C-30 and C-70)
or (C-31 and C-71) or
(C-32 and C-72) or (C-
35 and C-75)) AND
(NOT C-10.1 AND NOT
C-10.3 AND NOT C-
10.4 AND NOT C-10.5)
AND
NOT C-06.2
)
CSMS:
Check TransactionStatus
TC_E_32 Transaction with id ended - without message in
queue
M
TC_E_33 Without transactionId - with message in queue M C C-16 Check TransactionStatus
TC_E_34 Without transactionId - without message in
queue
M C C-16 Check TransactionStatus
Reset Sequence Number
TC_E_53 CSMS accepting seqNo = 0 at start of
transaction
M
Remote start transaction
TC_F_01 Cable plugin first C M If the Charging Station does not have a cable
lock.
NOT AQ-2 and (C-36 or
C-37 or C-38 or C-39)
Authorization options for remote
start
TC_F_02 Remote start first - AuthorizeRemoteStart is
true
C M If AuthorizeRemoteStart can be set to true C-48.1 and (C-36 or C-
37 or C-38 or C-39)
Authorization options for remote
start
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 32/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_F_03 Remote start first - AuthorizeRemoteStart is
false
C M If AuthorizeRemoteStart can be set to false C-48.2 and (C-36 or C-
37 or C-38 or C-39)
Authorization options for remote
start
TC_F_04
(2.1)
Remote start first - Cable plugin timeout C M NOT HFS-13
Remote stop transaction
TC_F_08 Success M
TC_F_09 Rejected M
Remote unlock Connector
TC_F_05 With ongoing transaction C If the Charging Station has a detachable cable,
with a mechanized locking mechanism.
HFS-1 and AQ-18
TC_F_06 Without ongoing transaction - Accepted C C If the Charging Station has a detachable cable,
with a mechanized locking mechanism.
If the CSMS support the Unlocking connector
for Charging Station with detachable cable
(UnlockConnector) feature.
CSMS: C-11
CS: HFS-1 and AQ-18
TC_F_07 Without ongoing transaction - No cable
connected
C If the Charging Station has a detachable cable,
with a mechanized locking mechanism.
HFS-1 and AQ-18
TC_F_10 Without ongoing transaction -
UnknownConnector
C If the Charging Station has a detachable cable,
with a mechanized locking mechanism.
HFS-1 and AQ-18
Trigger message
TC_F_11 MeterValues - Specific EVSE C C If the SUT supports TriggerMessage for
requestedMessage MeterValues for a specific
EVSE.
C-29.1 TriggerMessage
TC_F_12 MeterValues - All EVSE C C If the SUT supports TriggerMessage for
requestedMessage MeterValues for a all EVSE.
C-29.1 TriggerMessage
TC_F_13 TransactionEvent - Specific EVSE C C If the SUT supports TriggerMessage for
requestedMessage TransactionEvent for a
specific EVSE.
C-29.2 TriggerMessage
TC_F_14 TransactionEvent - All EVSE C C If the SUT supports TriggerMessage for
requestedMessage TransactionEvent for a all
EVSE.
C-29.2 TriggerMessage
TC_F_15 LogStatusNotification - Idle C C If the SUT supports TriggerMessage for
requestedMessage LogStatusNotification.
C-29.3 TriggerMessage
TC_F_16 LogStatusNotification - Uploading C If the Charging Station supports
TriggerMessage for requestedMessage
LogStatusNotification.
C-29.3 TriggerMessage
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 33/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_F_17 FirmwareStatusNotification - Specific EVSE not
relevant
C If the Charging Station supports
TriggerMessage for requestedMessage
FirmwareStatusNotification.
C-29.4 TriggerMessage
TC_F_18 FirmwareStatusNotification - Idle C C If the SUT supports TriggerMessage for
requestedMessage
FirmwareStatusNotification.
C-29.4 TriggerMessage
TC_F_19 FirmwareStatusNotification - Downloading C If the Charging Station supports
TriggerMessage for requestedMessage
FirmwareStatusNotification.
C-29.4 TriggerMessage
TC_F_20 Heartbeat M M
TC_F_23 StatusNotification - Specific EVSE - Available C C If the SUT supports TriggerMessage for
requestedMessage StatusNotification for a
specific EVSE.
C-29.5 TriggerMessage
TC_F_24 StatusNotification - Specific EVSE - Occupied C C If the SUT supports TriggerMessage for
requestedMessage StatusNotification for a
specific EVSE.
C-29.5 TriggerMessage
TC_F_26 BootNotification - Rejected C If the Charging Station supports
TriggerMessage for requestedMessage
BootNotification.
C-29.6 TriggerMessage
TC_F_27 NotImplemented C M For CS: can only be done when
SignCombinedCertificate is notimplemented
NOT AQ-11
TC_F_100 CustomTrigger C C C-29.7 Trigger message
Connector status Notification Charging Station: This can either be
implemented with the StatusNotification or
NotifyEvent message.
CSMS: Both StatusNotification and NotifyEvent
must be supported.
TC_G_01 Available to Occupied M
TC_G_02 Occupied to Available M
TC_G_20 Lock Failure M
Change Availability EVSE Charging Station: This can either be
implemented with the StatusNotification or
NotifyEvent message.
CSMS: Both StatusNotification and NotifyEvent
must be supported.
TC_G_03 Operative to inoperative M M
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 34/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_G_09 Operative to operative M
TC_G_04 Inoperative to operative M M
TC_G_10 Inoperative to inoperative M
TC_G_11 With ongoing transaction M M
TC_G_18 state persists across reboot M
Change Availability Charging Station Charging Station: This can either be
implemented with the StatusNotification or
NotifyEvent message.
CSMS: Both StatusNotification and NotifyEvent
must be supported.
TC_G_05 Operative to inoperative M M
TC_G_12 Operative to operative M
TC_G_06 Inoperative to operative M M
TC_G_13 Inoperative to inoperative M
TC_G_21 state persists across reboot M
TC_G_14 With ongoing transaction M M
Change Availability Connector Charging Station: This can either be
implemented with the StatusNotification or
NotifyEvent message.
CSMS: Both StatusNotification and NotifyEvent
must be supported.
TC_G_07 Operative to inoperative M M
TC_G_15 Operative to operative M
TC_G_08 Inoperative to operative M M
TC_G_16 Inoperative to inoperative M
TC_G_17 With ongoing transaction M M
TC_G_19 state persists across reboot M
Clock-aligned Meter Values Charging Station can choose which
measurands are supported (At least one).
This can either be implemented with the
MeterValues or NotifyEvent message.
TC_J_01 No transaction ongoing M M C-40 Supported MeterValue Measurands
TC_J_02 Transaction ongoing M M C-40 Supported MeterValue Measurands
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 35/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_J_03 EventType Ended M M C-40 Supported MeterValue Measurands
TC_J_04 Signed C M Charging Station: If signed MeterValues is
implemented
CSMS: Must at least be able to receive a signed
MeterValue. It does not need to be able to read
it.
C-42 Supported MeterValue Measurands
& Signed Metervalues
TC_J_06 No Meter Values during transaction C If AlignedDataSendDuringIdle is supported. C-28 AlignedDataSendDuringIdle
Sampled Meter Values Charging Station can choose which
measurands are supported (At least one).
TC_J_07 EventType Started - EVSE known M M C-40 Supported MeterValue Measurands
TC_J_08 Context Transaction.Begin - EVSE not known C M C-40 and NOT AQ-8
AND (C-09.2 OR C-
09.6)
Supported MeterValue Measurands
& possibility to enforce EVSE being
known.
TC_J_09 EventType Updated M M C-40 Supported MeterValue Measurands
TC_J_10 EventType Ended M M C-40 Supported MeterValue Measurands
TC_J_11 Signed C M Charging Station: If signed MeterValues is
implemented
CSMS: Must at least be able to receive a signed
MeterValue. It does not need to be able to read
it.
C-42 Supported MeterValue Measurands
& Signed Metervalues
Remote start transaction with charging profile
TC_K_38 Ignore chargingProfile C The Charging Station does NOT support Smart
Charging.
NOT Smart Charging
Secure Firmware Update
TC_L_01 Installation successful M M
TC_L_02 InstallScheduled M C C-15 Scheduled firmware updates
TC_L_03 DownloadScheduled M C C-15 Scheduled firmware updates
TC_L_04 RevokedCertificate M
TC_L_05 InvalidCertificate M M
TC_L_06 InvalidSignature M M
TC_L_07 DownloadFailed M M
TC_L_08 InstallVerificationFailed or InstallationFailed M M
TC_L_09 InstallationFailed M
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 36/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_L_10 AcceptedCanceled C M The Charging Station supports cancellling an
ongoing firmware update
C-60
TC_L_11 Unable to cancel C M The Charging Station does NOT supports
cancellling an ongoing firmware update
NOT C-60
TC_L_18 Missing firmware signing certificate and
signature
M
TC_L_12 Unable to download/install firmware with
ongoing transaction -
AllowNewSessionsPendingFirmwareUpdate is
true
C AllowNewSessionsPendingFirmwareUpdate is
implemented.
The Charging Station is unable to download
AND install firmware while there is an ongoing
transaction.
C-20 and NOT C-43
and NOT AQ-7 and
HFS-8 > 1
AllowNewSessionsPendingFirmwa
reUpdate
TC_L_13 Unable to download/install firmware with
ongoing transaction -
AllowNewSessionsPendingFirmwareUpdate is
false
C M AllowNewSessionsPendingFirmwareUpdate is
implemented.
The Charging Station is unable to download
AND install firmware while there is an ongoing
transaction.
NOT C-43 and NOT AQ-
7
TC_L_14 Unable to install and activate firmware with
ongoing transaction -
AllowNewSessionsPendingFirmwareUpdate is
true
C AllowNewSessionsPendingFirmwareUpdate is
implemented.
The Charging Station is unable to install
firmware while there is an ongoing transaction
C-20 and NOT C-43
and AQ-7 and HFS-8 >
1
AllowNewSessionsPendingFirmwa
reUpdate
TC_L_15 Unable to install firmware with ongoing
transaction -
AllowNewSessionsPendingFirmwareUpdate is
false
C AllowNewSessionsPendingFirmwareUpdate is
implemented.
The Charging Station is unable to install
firmware while there is an ongoing transaction
NOT C-43 and AQ-7
TC_L_16 Able to update firmware with ongoing
transaction
C If the Charging Station supports Install
Firmware with ongoing transaction(s)
C-43 Install Firmware with ongoing
transaction(s)
Retrieve certificates from Charging Station
TC_M_12 CSMSRootCertificate M
TC_M_13 ManufacturerRootCertificate M M
TC_M_17 CSMSRootCertificate &
ManufacturerRootCertificate
M
TC_M_18 All certificateTypes M M
TC_M_19 No matching certificate found M M
Delete a certificate from a Charging Station
TC_M_20 Success M M
TC_M_21 Failed M
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 37/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_M_22 No matching certificate found M
Install CA certificate
TC_M_01 CSMSRootCertificate M M
TC_M_02 ManufacturerRootCertificate M M
TC_M_05 Failed M
TC_M_07 Rejected - Certificate invalid M
TC_M_09 AdditionalRootCertificateCheck - Rejected C If the Charging Station supports
AdditionalRootCertificateCheck with value true
AS-2 Additional Root Certificate check
mechanism implemented
TC_M_30 AdditionalRootCertificateCheck - Reconnect
using new CSMS Root - Success
C If the Charging Station supports
AdditionalRootCertificateCheck with value true
AS-2 Additional Root Certificate check
mechanism implemented
TC_M_31 AdditionalRootCertificateCheck - Reconnect
using new CSMS Root - Fallback mechanism
C If the Charging Station supports
AdditionalRootCertificateCheck with value true
AS-2 Additional Root Certificate check
mechanism implemented
Retrieve Log Information
TC_N_25 Diagnostics Log - Success M M
TC_N_34 Rejected M
TC_N_26 Diagnostics Log - Upload failed M
TC_N_35 Security Log - Success M M
TC_N_36 Second Request C M If the Charging Station is able to cancel an
ongoing log file upload.
C-57
TC_N_100
(2.1)
DataCollectorLog - Success C C CS: C-65
CSMS: C-65 OR
Bidirectional Power
Transfer OR DER
Control
Support for DataCollectorLog
TC_N_101
(2.1)
Validations C Charging station supports HTTP or HTTPS file
transfer protocol for logging
Supported file transfer
protocols contains
HTTP or HTTPS
TC_N_102
(2.1)
Authentication - HTTP C M Charging station supports HTTP file transfer
protocol for logging
Supported file transfer
protocols contains
HTTP
TC_N_103
(2.1)
Authentication - HTTPS C M Charging station supports HTTPS file transfer
protocol for logging
Supported file transfer
protocols contains
HTTPS
Get Customer Information
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 38/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_N_27 Accepted + data C M For CS: The Charging Station needs to support
Local Authorization and either the Local
Authorization List or Authorization Cache.
IdToken is used as customer information.
((C-30 and C-70) or (C-
31 and C-71)) and
(Local Authorization
List Management or C-
49)
TC_N_28 Accepted + no data C M For CS: The Charging Station needs to support
Local Authorization and either the Local
Authorization List or Authorization Cache.
IdToken is used as customer information.
((C-30 and C-70) or (C-
31 and C-71)) and
(Local Authorization
List Management or C-
49)
TC_N_29 Not Accepted M
Clear Customer Information
TC_N_30 Clear and report + data C M For CS: The Charging Station needs to support
Local Authorization or the Authorization Cache.
IdToken is used as customer information.
((C-30 and C-70) or (C-
31 and C-71)) and
(Local Authorization
List Management or C-
49)
TC_N_31 Clear and report + no data C M For CS: The Charging Station needs to support
Local Authorization or the Authorization Cache.
IdToken is used as customer information.
((C-30 and C-70) or (C-
31 and C-71)) and
(Local Authorization
List Management or C-
49)
TC_N_32 Clear and no report M M
TC_N_62 Clear and report - customerIdentifier C C Support for retrieving / deleting
CustomerInformation - CustomerIdentifier
C-14
Data Transfer to the Charging Station
TC_P_01 Rejected / Unknown VendorId / Unknown
MessageId
M Charging Station must be able to Reject the
message.
Data Transfer to the CSMS
TC_P_02 Rejected / Unknown VendorId / Unknown
MessageId
M CSMS must be able to Reject the message.
CustomData
TC_P_03 Receive custom data M M
Battery Swap (2.1)
TC_S_102 Remote Start - not enough batteries C C CS: HFS-13
CSMS: C-76
CSMS: Support for Battery
Swapping Stations
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 39/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_S_103 Remote Start - enough batteries available C C-76 Support for Battery Swapping
Stations
TC_S_104 Charging - Variables validation C HFS-13
TC_S_105 Charging - Battery Swap Charging C HFS-13
Local Authorization List Management
Offline authorization through local
authorization list
TC_C_21 Accepted C The Charging Station must support at least one
of the following local start authorization
options: C-30, C-31, C-32
LA-0 and C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72
TC_C_22 Invalid C The Charging Station must support at least one
of the following local start authorization
options: C-30, C-31, C-32
LA-0 and (C-30 or C-31
or C-32)
TC_C_23 Blocked C The Charging Station must support at least one
of the following local start authorization
options: C-30, C-31, C-32
LA-0 and (C-30 or C-31
or C-32)
TC_C_24 Expired C The Charging Station must support at least one
of the following local start authorization
options: C-30, C-31, C-32
LA-0 and (C-30 or C-31
or C-32)
TC_C_25 Local Authorization List > Authorization Cache C The Charging Station must support at least one
of the following local start authorization
options: C-30, C-31, C-32
LA-0 and C-49 and (C-
30 or C-31 or (C-32 and
C-72))
Online authorization through local
authorization list
TC_C_27 Accepted C LA-0
TC_C_28 Invalid & Not Accepted C LA-0
TC_C_31 Invalid & Accepted C LA-0
TC_C_29 Blocked C LA-0
TC_C_30 Expired C LA-0
TC_C_58 LocalAuthListDisablePostAuthorize C The Charging Station supports the option for
disabling remote authorization for invalid
idTokens stored at the Local Authorization List.
LA-0 and LA-3
Authorization by GroupId
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 40/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_C_40 Success with Local Authorization List C C For CS:
- The Charging Station supports at least one of
the following local start authorization options:
C-30, C-31, C-32
CS: LA-0 and C-30 and
C-70) or (C-31 and C-
71) or (C-32 and C-72
CSMS: LA-0
TC_C_43 Invalid status with Local Authorization List C C For CS:
- The Charging Station supports at least one of
the following local start authorization options:
C-30, C-31, C-32
CS: LA-0 and C-30 and
C-70) or (C-31 and C-
71) or (C-32 and C-72
CSMS: LA-0
Send Local Authorization List
TC_D_01 Full C C LA-0
TC_D_02 Differential Update C C LA-0
TC_D_03 Differential Remove C C LA-0
TC_D_04 Full with empy list C C LA-0
TC_D_05 Differential with empty list C LA-0
TC_D_06 VersionMismatch C LA-0
TC_D_07 Persistent over reboot C LA-0
Get Local List Version
TC_D_08 Success C C CS: LA-0
CSMS: LA-0 and LA-2
GetLocalListVersion
TC_D_09 No list available C LA-0
TC_D_10 Function disabled C LA-0
Clear Customer Information
TC_N_46 Update Local Authorization List C LA-0
Reservation
Reset Charging Station
TC_B_24 Reserved persists reset C R-0
Reserve a specific EVSE
TC_H_01 Accepted - Valid idToken C C R-0
TC_H_02 Accepted - Different idToken C R-0
TC_H_03 Occupied - EVSE Reserved C R-0
TC_H_04 Occupied - EVSE Occupied C R-0
TC_H_05 Faulted
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 41/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_H_06 Unavailable C R-0
TC_H_07 Reservation Ended / not used C C R-0
TC_H_22 Configured to Reject C C For CS: The configuration variable
ReservationCtrlr.ReservationEnabled is
implemented
CS: R-0 and R-3
CSMS: R-0
TC_H_23 Replace reservation C R-0
TC_H_19 Use a reserved EVSE with GroupId C C R-0
Reserve an unspecified EVSE
TC_H_08 Accepted C C For CS: Depending on configuration variable
ReservationNonSpecificEVSE
R-0 and R-2 Support reservations of
unspecified EVSE
TC_H_09 Occupied - EVSE Reserved C Depending on configuration variable
ReservationNonSpecificEVSE
R-0 and R-2 Support reservations of
unspecified EVSE
TC_H_10 Occupied - EVSE Occupied C Depending on configuration variable
ReservationNonSpecificEVSE
R-0 and R-2 Support reservations of
unspecified EVSE
TC_H_12 Unavailable C Depending on configuration variable
ReservationNonSpecificEVSE
R-0 and R-2 Support reservations of
unspecified EVSE
TC_H_13 Rejected C Depending on the Charging Station not
supporting the configuration variable
ReservationNonSpecificEVSE
R-0 and NOT R-2 Support reservations of
unspecified EVSE
TC_H_14 Amount of EVSEs available equals the amount
of reservations
C C For CS: Depending on configuration variable
ReservationNonSpecificEVSE
For CSMS: this wil be tested with > 1 EVSE
R-0 and R-2 Support reservations of
unspecified EVSE
TC_H_24 GroupIdToken C For CS: Depending on configuration variable
ReservationNonSpecificEVSE
R-0 and R-2 Support reservations of
unspecified EVSE
Reserve a connector with a specific type
TC_H_15 Success C C R-0 and R-1 Support reservations of
connectorType
TC_H_16 Amount of available connectors of a type
equals the amount of reservations
C R-0 and R-1 Support reservations of
connectorType
Cancel reservation of an EVSE
TC_H_17 Success C C R-0
TC_H_18 Rejected C R-0
TC_H_20 Charging Station cancels reservation when
Faulted
C R-0
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 42/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_H_21 Charging Station cancels reservation when
Unavailable
C R-0
 
Payment
Set Default Tariff (2.1)
TC_I_100 validFrom C P-0
TC_I_101 startTimeOfDay, endTimeOfDay C C P-0
TC_I_102 TariffMaxElements C C P-0
TC_I_104 transaction with default tariff C P-0
TC_I_105 TariffConditionsSupported is false C C CS: P-0 and NOT P-1
CSMS: P-0
No Support for Tariff conditions
TC_I_106 validations C C P-0
Receive Driver Tariff (2.1)
TC_I_107 CS cannot process tariff -
UseDefault/CentralCost
C P-0
TC_I_108 CS cannot process tariff - Deauthorize C P-0
TC_I_109 Goodflow C C P-0
Clear Tariffs (2.1)
TC_I_110 DefaultTariff C C P-0
TC_I_111 Tariff in use C P-0
Local Cost Calculation (2.1)
TC_I_113 Change transaction tariff - TariffMaxElements C C P-0
TC_I_114 Change transaction tariff -
TariffConditionsSupported is false
C C P-0
TC_I_115 Change transaction tariff -
TariffConditionsSupported is true
C C CS: P-0 and P-1
CSMS: P-0
Support for Tariff conditions
TC_I_116 Change transaction tariff - goodflow C P-0
TC_I_117 Change transaction tariff - validations C P-0
TC_I_118 Cost Details of Transaction - no tariff
conditions
C P-0
TC_I_119 Cost Details of Transaction - with tariff
conditions
C CS: P-0 and P-1
CSMS: P-0
Support for Tariff conditions
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 43/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_I_120 Cost Details of Transaction - reservation C Certification Profile: Reservation supported P-0
TC_I_121 Cost Details of Transaction - minCost/maxCost C P-0
TC_I_122 Cost Details of Transaction C P-0
Authorization with Prepaid card (2.1)
TC_C_103 Success C C CS: P-0 and P-2.1
CSMS: P-0
Payment by prepaid card
TC_C_104 No credit C C CS: P-0 and P-2.1
CSMS: P-0
Payment by prepaid card
Integrated Payment Terminal (2.1)
TC_C_105 CSMS rejects authorization C P-0 and P-2.2 Integrated payment terminal
TC_C_106 Only Payment Terminal authorises C P-0 and P-2.2 Integrated payment terminal
TC_C_107 Payment Terminal and CSMS authorises C P-0 and P-2.2 Integrated payment terminal
TC_C_108 VAT number validation C C CS: P-0 and P-2.2
CSMS: P-0
Integrated payment terminal
TC_C_109 Cancelation prior to transaction - Only Payment
Terminal authorised - EVConnectionTimeout
C P-0 and P-2.2 Integrated payment terminal
TC_C_110 Cancelation prior to transaction - Payment
Terminal and CSMS authorised - stopped by EV
driver
C P-0 and P-2.2 Integrated payment terminal
TC_C_111 Cancelation prior to transaction - Only Payment
Terminal authorised - stopped by EV driver
C P-0 and P-2.2 Integrated payment terminal
TC_C_112 Cancelation prior to transaction - Payment
Terminal and CSMS authorised -
EVConnectionTimeout
C P-0 and P-2.2 Integrated payment terminal
TC_C_113 Cancelation after start of transaction - stopped
by EV driver
C C CS: P-0 and P-2.2
CSMS: P-0
Integrated payment terminal
Settlement at end of transaction (2.1)
TC_C_114 Settled by CS, receipt by CS C P-0 and P-2.2 Integrated payment terminal
TC_C_115 Settled by CSMS C P-0 and P-2.2 Integrated payment terminal
TC_C_116 Settled by CS, receipt by CSMS C P-0 and P-2.2 Integrated payment terminal
TC_C_117 Settled by CSMS, receipt by CSMS C P-0
TC_C_118 Settled by CS, receipt by CSMS C P-0
Settlement (2.1)
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 44/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_C_119 Is rejected or fails - Failed C C CS: P-0 and P-2.2
CSMS: P-0
Integrated payment terminal
TC_C_120 Is rejected or fails - Rejected C C CS: P-0 and P-2.2
CSMS: P-0
Integrated payment terminal
Incremental authorization (2.1)
TC_C_121 increasing enabled C P-0 and P-2.2 Integrated payment terminal
TC_C_122 increasing disabled C P-0 and P-2.2 Integrated payment terminal
Ad hoc payment via stand-alone payment
terminal (2.1)
TC_C_123 Local cost calculation C P-0 and P-2.3 Stand alone payment terminal
TC_C_124 Central cost calculation C P-0 and P-2.3 Stand alone payment terminal
TC_C_125 Central cost calculation C P-0
TC_C_126 Local cost calculation C P-0
Ad hoc payment via static or dynamic QR code
(2.1)
TC_C_127 No URL parameters C P-0 and P-2.4 QR code payment
TC_C_128 URL parameter maxTime C P-0 and P-2.4 QR code payment
TC_C_129 URL parameter maxCost C P-0 and P-2.4 QR code payment
TC_C_130 URL parameter maxEnergy C P-0 and P-2.4 QR code payment
TC_C_131 Success C P-0
TC_C_132 Invalid URL parameters C P-0
TC_C_133 Invalid totp C P-0
Advanced Device Management
Get Custom Report
TC_B_16 with component criteria C DM-0
TC_B_17 with component/variable C DM-0
TC_B_18 with componentCriteria and
component/variables
C C DM-0
TC_B_54 with component/variable, but no instance C DM-0
TC_B_55 with component/variable/instance C DM-0
TC_B_56 with component/variable, but no evseId C DM-0
Get Monitoring Report
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 45/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_N_01 with monitoringCriteria C C DM-0
TC_N_02 with component/variable C C DM-0
TC_N_03 with component criteria and
component/variable
C C DM-0
TC_N_47 Report all C C DM-0
TC_N_60 with component criteria and list of
components/variables
C DM-0
TC_N_104
(2.1)
TargetDeltaMonitoring C C DM-0
Set Monitoring Base
TC_N_05 success C C DM-0
TC_N_06 test removal custom monitors C DM-0
Set Variable Monitoring
TC_N_08 One SetMonitoringData element C C DM-0
TC_N_09 Multiple elements on different component and
variable
C DM-0
TC_N_10 Multiple monitors on the same component and
variable
C DM-0
TC_N_11 Unknown component C DM-0
TC_N_12 Value out of range - Delta monitor C DM-0
TC_N_13 Value out of range - Threshold monitor C DM-0
TC_N_15 Duplicate Variable type/severity combination C DM-0
TC_N_24 Periodic event C C DM-0
TC_N_37 Unknown Variable C DM-0
TC_N_39 Component/Variable combination does NOT
correspond
C DM-0
TC_N_40 Replace Variable Monitor C DM-0
TC_N_41 Return to FactoryDefault C DM-0
TC_N_43 First SetMonitoringData and third
SetMonitoringData are valid, but the second
contains an out of range value
C DM-0
TC_N_51 Modifying a VariableMonitor and trigger C DM-0
TC_N_52 Removing a VariableMonitor C DM-0
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 46/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_N_105
(2.1)
Set Frequent Periodic Variable Monitoring -
Periodic
C C DM-0
TC_N_106
(2.1)
Set Frequent Periodic Variable Monitoring -
CSMS rejects stream
C DM-0
TC_N_107
(2.1)
Get Periodic Event Streams - Goodflow C DM-0
TC_N_108
(2.1)
Close Periodic Event Streams C DM-0
TC_N_109
(2.1)
Adjust Periodic Event Streams C DM-0
Set Monitoring Level
TC_N_16 Success C C DM-0
TC_N_17 Out of range C C DM-0
Clear Monitoring
TC_N_18 Success C C DM-0
TC_N_19 Not found C DM-0
TC_N_44 Rejected C C CS: If the Charging Station has at least one
hardWired monitor.
CS: DM-0 and AQ-5
CSMS: DM-0
Alert Event
TC_N_20 Threshold value exceeded C DM-0
TC_N_21 HardWiredMonitor C C CS: If the Charging Station has at least one
hardWired monitor.
CS: DM-0 and AQ-5
CSMS: DM-0
TC_N_45 Delta value exceeded C DM-0
TC_N_48 Variable monitoring on write only C CS: if the CS supports Delta monitoring on the
SecurityCtrlr.BasicAuthPassword
DM-0 and AQ-10
TC_N_49 LowerThreshold/UpperThreshold cleared after
reboot
C DM-0
TC_N_50 Periodic Triggered C DM-0
TC_N_53 Persistant over reboot C DM-0
TC_N_56 Delta value NOT numeric exceeded C DM-0
Offline Notification
TC_N_22 OfflineMonitoringEventQueuingSeverity set
equal or lower
C DM-0 and DM-3 OfflineMonitoringEventQueuingSev
erity
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 47/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_N_23 OfflineMonitoringEventQueuingSeverity set
higher
C DM-0 and DM-3 OfflineMonitoringEventQueuingSev
erity
Advanced User Interface
Show costs to EV Driver
TC_I_01 Show EV Driver running total cost during
charging
C C UI-0
TC_I_02 Show EV Driver Final Total Cost After Charging C C UI-0
TC_I_07 Show EV Driver running total cost during
charging - transactionEventResponse
C UI-0
Set Display Message
TC_O_01 Success C C UI-0 UI-1 and UI-2: Supported
MessagePriorities & Supported
MessageFormats
TC_O_26 Rejected C UI-0
TC_O_13 Display message at StartTime C C UI-0
TC_O_14 Remove message after EndTime C C UI-0
TC_O_17 NotSupportedPriority C C CS: In case it does not support 1 of the
MessagePriorities
CS: UI-0 and NOT (UI-
1.1 and UI-1.2 and UI-
1.3)
CSMS: UI-0
TC_O_18 NotSupportedState C UI-0
TC_O_19 NotSupportedMessageFormat C C CS: In case it does not support 1 of the
MessageFormats
CS: UI-0 and NOT (UI-
2.1 and UI-2.2 and UI-
2.3 and UI-2.4)
CSMS: UI-0
TC_O_20 Persistent over reboot C UI-0
TC_O_22 Multiple In front priority C If the Charging Station supports InFront priority UI-0 and UI-1.2 Supported MessagePriorities
InFront
TC_O_24 Second Alwaysfront priority C If the Charging Station supports AlwaysFront
priority
UI-0 and UI-1.1 Supported MessagePriorities
AlwaysFront
TC_O_36 State Charging C UI-0
TC_O_37
(2.1)
State Idle C UI-0 AND NOT HFS-13
TC_O_38 State Unavailable C UI-0
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 48/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature no. Feature
TC_O_12 Replace DisplayMessage C C UI-0
TC_O_100
(2.1)
unsupported language C C CS: UI-0 and AQ-19
CSMS: UI-0
TC_O_101
(2.1)
Language preference of the EV Driver C C CS: UI-0 and UI-3
CSMS: UI-0
Set Display Message - Specific transaction
TC_O_06 Success C C UI-0
TC_O_10 UnknownTransaction C C UI-0
TC_O_27 Display message at StartTime C C UI-0
TC_O_28 Remove message after EndTime C C UI-0
TC_O_30 Multiple In front priority C CS: If value "InFront" supported UI-0 and UI-1.2 Supported MessagePriorities
InFront
TC_O_32 Second Alwaysfront priority C CS: If value "AlwaysFront" supported UI-0 and UI-1.1 Supported MessagePriorities
AlwaysFront
Get all Display Messages
TC_O_02 Success C C UI-0
TC_O_03 No DisplayMessages configured C C UI-0
Get a Specific Display Message
TC_O_07 Id C C UI-0
TC_O_08 Priority C C UI-0 UI-1: Supported MessagePriorities
TC_O_09 State C C UI-0
TC_O_11 Unknown parameters C UI-0
TC_O_33 No DisplayMessages configured C UI-0
TC_O_34 Known Id, but not matching State C UI-0
TC_O_35 Known Id, but not matching Priority C Only if multiple messagePriorities supported UI-0 and ((UI-1.1 and
UI-1.2) or (UI-1.2 and
UI-1.3) or (UI-1.3 and
UI-1.1))
Supported MessagePriorities
Clear Display Message
TC_O_04 Success C C UI-0
TC_O_05 Unknown Key C C UI-0
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 49/68 Part 5 - Certification Profiles

4.3. Test Cases Advanced Security
TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature
no.
Feature
TLS - Client-side certificate
TC_A_07 valid certificate M M
TC_A_08 Invalid certificate M
Update Charging Station Certificate by request of
CSMS
TC_A_11 Success - Charging Station Certificate M M
TC_A_14 Invalid certificate M M
TC_A_15 SignCertificateRequest Rejected M
TC_A_23 CertificateSignedRequest Timeout C If the Charging Station supports
CertificateSignedRequest Timeout
AS-3
Upgrade Charging Station Security Profile
TC_A_21 No valid ChargingStationCertificate installed C If the last ChargingStationCertificate can be removed
(Via other means than OCPP).
AQ-3
Delete a certificate from a Charging Station
TC_M_23 Unable to delete the Charging Station Certificate M
Set new NetworkConnectionProfile (2.1)
TC_B_112
(2.1)
AllowSecurityDowngrade is false C AS-4.1 Support for disallowing security
downgrades from profile 3 to 2
TC_B_113
(2.1)
AllowSecurityDowngrade = false - DM C AS-4.1 Support for disallowing security
downgrades from profile 3 to 2
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 50/68 Part 5 - Certification Profiles

4.4. Test Cases Smart Charging
TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature
no.
Feature
Set Charging Profile
TC_K_01 TxDefaultProfile - Specific EVSE M M Remark: for CSMS TxDefaultProfile is mandatory,
specific values ValidFrom/ValidTo, Duration and limit
are tested.
TC_K_10 TxDefaultProfile - All EVSE M C SC-4 Support for TxDefaultProfile on
EVSEID #0
TC_K_60 TxProfile with ongoing transaction on the specified
EVSE
M M Remark for CSMS: combination TxProfile and Relative is
mandatory
TC_K_02 TxProfile without ongoing transaction on the specified
EVSE
M
TC_K_11 Unable to set TxProfile on all EVSE at once M
TC_K_03 ChargingStationMaxProfile M M Remark: for CSMS ChargingStationMaxProfile is
mandatory, specific values ValidFrom/ValidTo, Duration
and limit are tested.
TC_K_19 ChargingProfileKind is Recurring M M Remark for CSMS: TxDefaultProfile or
ChargingStationMaxProfile. Configurable
recurrencyKind: weekly and Daily are mandatory
TC_K_12 ChargerRateUnit Rejected C If only 1 of the 2 ChargingRateUnits is supported by the
Charging Station
NOT (SC-
2.1 and
SC-2.2)
TC_K_13 Persistent over reboot M
TC_K_14 Unexisting EVSEid M
TC_K_28 TxDefaultProfile with transaction ongoing M
TC_K_15 Not Supported M
TC_K_16 Unknown transactionId M
TC_K_21 ValidFrom M
TC_K_22 ValidTo M
TC_K_23 StartSchedule M
TC_K_70 Multiple Profiles M Remark for CSMS: at least two different stacklevels,
amount of ChargingProfiles: at least 2 different id’s
TC_K_100
(2.1)
maxOfflineDuration M M
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 51/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature
no.
Feature
TC_K_102
(2.1)
limitAtSoc C C SC-3 Support for limiting based on
SoC
TC_K_103
(2.1)
Local time - TimeOffset C SC-5.1 Support for using local time
(TimeOffset)
TC_K_136
(2.1)
Local time - TimeZone C SC-5.2 Support for using local time
(TimeZone)
TC_K_104
(2.1)
PriorityCharging C C SC-6 Support for using priority
charging
TC_K_105
(2.1)
ChargingStationMaxProfile persistent over reboot M
TC_K_106
(2.1)
randomizedDelay C M SC-7 Support for using randomized
delays
TC_K_107
(2.1)
randomizedDelay - validations C SC-7 Support for using randomized
delays
TC_K_108
(2.1)
randomizedDelay - random for each tx C SC-7 Support for using randomized
delays
TC_K_129
(2.1)
PriorityCharging persistent over reboot C SC-6 Support for using priority
charging
TC_K_130
(2.1)
PriorityCharging unsupported C NOT SC-6
TC_K_131
(2.1)
LocalGeneration unsupported C Not
supportin
g
Bidirectio
nal Power
Transfer
TC_K_132
(2.1)
useLocalTime unsupported C NOT SC-5
TC_K_133
(2.1)
RandomizedDelay unsupported C NOT SC-7
TC_K_134
(2.1)
LimitAtSoC unsupported C NOT SC-3
TC_E_117
(2.1)
Resuming transaction after interruption -
TxResumptionTimeout not expired -
TxAllowEnergyTransferResumption is true
M
Replace charging profile
TC_K_04 With chargingProfileId M M
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 52/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature
no.
Feature
Remote start transaction with charging profile
TC_K_37 Success M M
Get Composite Schedule
TC_K_39 No ChargingProfile installed on Charging Station M
TC_K_40 Stacking ChargingProfiles M Remark for Charging Station: At least max stacklevel 1 SC-2 ChargingRateUnit
TC_K_41 Combining chargingProfilePurposes M Remark for Charging Station: amount of
ChargingProfiles at least 3
TC_K_42 chargingRateUnit not supported C If one of the ChargingRateUnits is not supported. NOT (SC-
2.1 and
SC-2.2)
TC_K_43 Specific EVSE M
TC_K_44 Charging Station M
TC_K_47 Unknown EVSEId M
TC_K_112
(2.1)
randomizedDelay C SC-7 Support for using randomized
delays
Get Charging Profile
TC_K_29 EvseId 0 M M
TC_K_30 EvseId > 0 M M
TC_K_31 No EvseId M M
TC_K_32 chargingProfileId M M
TC_K_33 EvseId > 0 + stackLevel M M
TC_K_34 EvseId > 0 + chargingLimitSource M M
TC_K_35 EvseId > 0 + chargingProfilePurpose M M
TC_K_36 EvseId > 0 + chargingProfilePurpose + stackLevel M M
Clear Charging Profile
TC_K_05 With chargingProfileId M M
TC_K_06 With stackLevel/purpose combination for one profile M M
TC_K_07 With unknown stackLevel/purpose combination M
TC_K_08 Without previous charging profile M M
TC_K_09 Clearing a TxDefaultProfile - With ongoing transaction M
TC_K_24 With stackLevel/purpose combination for multiple
profiles
C Applicable if the Charging Station has more than one
EVSE.
HFS-8 > 1
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 53/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature
no.
Feature
Priority charging
TC_K_118 Requesting priority charging remotely C C SC-6 Support for using priority
charging
TC_K_119 Requesting priority charging locally C SC-6 Support for using priority
charging
Dynamic charging profiles from CSMS
TC_K_121
(2.1)
Pull C C SC-8 Support for dynamic charging
profiles
TC_K_122
(2.1)
Push C SC-8 Support for dynamic charging
profiles
TC_K_123
(2.1)
validations C SC-8 Support for dynamic charging
profiles
Idle operationMode
TC_Q_125
(2.1)
Idle, minimizing energy consumption - Idle with
EvseSleep
C M SC-9.1 Support for Idle operationMode
with EvseSleep
TC_Q_126
(2.1)
Idle, minimizing energy consumption - Idle with
EvseSleep unsupported
C SC-9 AND
NOT SC-
9.1
Support for Idle operationMode
TC_Q_127
(2.1)
Idle, minimizing energy consumption - Charging profile
validations
C SC-9 Support for Idle operationMode
TC_K_135
(2.1)
Set Charging Profile - EvseSleep unsupported C NOT SC-
9.1
EMS Control
TC_K_48 Set / Update External Charging Limit (not on a
transaction)
C SC-10 Support for EMS control
TC_K_50 Reset / release external charging limit - Without ongoing
transaction
C SC-10 Support for EMS control
TC_K_51 Reset / release external charging limit - With ongoing
transaction
C SC-10 Support for EMS control
TC_K_52 Set External Charging Limit (not on a transaction) -
ChargingStationExternalConstraints in report
C C SC-10 Support for EMS control
TC_K_120 Smart Charging with EMS and LocalGeneration C SC-10 Support for EMS control
TC_K_109 Set Charging Profile - MaxExternalConstraintsId C C SC-10 Support for EMS control
TC_K_110 Set Charging Profile - MaxExternalConstraintsId -
validations
C SC-10 Support for EMS control
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 54/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature
no.
Feature
Dynamic charging profiles by external system
TC_K_124 No Dynamic charging profile configured C SC-10 Support for EMS control
TC_K_125 Dynamic charging profile configured C SC-10 Support for EMS control
External V2X control
TC_Q_114 With a charging profile from an External System -
Dynamic external limits control
C Bidirectional Power Transfer is supported SC-10
AND
Bidirectio
nal Power
Transfer
EMS Control
TC_Q_115 With a charging profile from an External System -
Dynamic setpoint control
C Bidirectional Power Transfer is supported SC-10
AND
Bidirectio
nal Power
Transfer
EMS Control
TC_Q_116 With a charging profile from an External System -
Scheduled external limits control
C Bidirectional Power Transfer is supported SC-10
AND
Bidirectio
nal Power
Transfer
EMS Control
TC_Q_111 With a charging profile from CSMS - setpoint C M Bidirectional Power Transfer is supported SC-10
AND
Bidirectio
nal Power
Transfer
EMS Control
TC_Q_112 With a charging profile from CSMS - limit C M Bidirectional Power Transfer is supported SC-10
AND
Bidirectio
nal Power
Transfer
EMS Control
TC_Q_113 With a charging profile from CSMS - Duration expired C Bidirectional Power Transfer is supported SC-10
AND
Bidirectio
nal Power
Transfer
EMS Control
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 55/68 Part 5 - Certification Profiles

4.5. Test Cases ISO 15118 Support
TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature
no.
Feature
Update Charging Station Certificate by request of CSMS
TC_A_12 Success - V2G Certificate M M
Authorization using Contract Certificates 15118
TC_C_50 Online - Local contract certificate validation - Accepted M M
TC_C_51 Online - Local contract certificate validation - Rejected M M
TC_C_52 Online - Central contract certificate validation - Accepted C M Applicable if the Charging Station supports
central contract validation.
ISO-5
TC_C_53 Online - Central contract validation fails C Applicable if the Charging Station supports
central contract validation.
ISO-5
TC_C_54 Offline - ContractValidationOffline is true M
TC_C_55 Offline - ContractValidationOffline is false M
End of charging process 15118
TC_E_46 End of charging process 15118 M
Set Charging Profile
TC_K_01 TxDefaultProfile - Specific EVSE M M
TC_K_10 TxDefaultProfile - All EVSE M M
TC_K_60 TxProfile with ongoing transaction on the specified EVSE M M
TC_K_02 TxProfile without ongoing transaction on the specified
EVSE
M
TC_K_11 Unable to set TxProfile on all EVSE at once M
TC_K_03 ChargingStationMaxProfile M M
TC_K_19 ChargingProfileKind is Recurring M M
TC_K_12 ChargerRateUnit Rejected C If only 1 of the 2 ChargingRateUnits is
supported by the Charging Station
NOT (SC-
2.1 and
SC-2.2)
TC_K_13 Persistent over reboot M
TC_K_14 Unexisting EVSEid M
TC_K_28 TxDefaultProfile with transaction ongoing M
TC_K_16 Unknown transactionId M
TC_K_21 ValidFrom M
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 56/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature
no.
Feature
TC_K_22 ValidTo M
TC_K_23 StartSchedule M
Replace charging profile
TC_K_04 With chargingProfileId M M
Get Composite Schedule
TC_K_39 No ChargingProfile installed on Charging Station M
TC_K_40 Stacking ChargingProfiles M
TC_K_41 Combining chargingProfilePurposes M
TC_K_42 chargingRateUnit not supported C If one of the ChargingRateUnits is not
supported.
NOT (SC-
2.1 and
SC-2.2)
TC_K_43 Specific EVSE M
TC_K_44 Charging Station M
TC_K_47 Unknown EVSEId M
Get Charging Profile
TC_K_29 EvseId 0 M M
TC_K_30 EvseId > 0 M M
TC_K_31 No EvseId M M
TC_K_32 chargingProfileId M M
TC_K_33 EvseId > 0 + stackLevel M M
TC_K_34 EvseId > 0 + chargingLimitSource M M
TC_K_35 EvseId > 0 + chargingProfilePurpose M M
TC_K_36 EvseId > 0 + chargingProfilePurpose + stackLevel M M
Clear Charging Profile
TC_K_05 With chargingProfileId M M
TC_K_06 With stackLevel/purpose combination for one profile M M
TC_K_24 With stackLevel/purpose combination for multiple profiles C Applicable if the Charging Station has more
than one EVSE.
HFS-8 > 1
TC_K_07 With unknown stackLevel/purpose combination M
TC_K_08 Without previous charging profile M M
TC_K_09 Clearing a TxDefaultProfile - With ongoing transaction M
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 57/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature
no.
Feature
Charging with load leveling based on High Level
Communication
TC_K_53 Success M M
TC_K_54 No SASchedule (rejected) M
TC_K_55 EV charging profile exceeds limits M
TC_K_56 Offline M
Renegotiating a Charging Schedule ISO 15118-2
TC_K_57 Initiated by EV C M ISO-1.1 ISO 15118-2
TC_K_58 Initiated by CSMS C M ISO-1.1 ISO 15118-2
TC_K_59 Initiated by CSMS - Send NotifyEVChargingNeeds M
Renegotiating a Charging Schedule ISO 15118-20
TC_K_113
(2.1)
Initiated by CSMS C M ISO-1.2 ISO 15118-20
TC_K_114
(2.1)
Initiated by EV C M ISO-1.2 ISO 15118-20
TC_K_116
(2.1)
Adjusting charging schedule when energy needs change C ISO-1.2 ISO 15118-20
ISO 15118-20 Dynamic Control Mode
TC_K_115
(2.1)
Success C M ISO-1.2 ISO 15118-20
TC_K_117
(2.1)
Adjusting charging schedule M
TC_K_126
(2.1)
Sets no charging profile M
Certificate Installation EV
TC_M_26 Success C M ISO-1.1 ISO 15118-2
TC_M_100
(2.1)
ISO 15118-20 - Success C M ISO-1.2 ISO 15118-20
TC_M_27 Failed M ISO-1.1 ISO 15118-2
Certificate Update EV
TC_M_28 Success C M ISO-1.1 ISO 15118-2
TC_M_29 Failed C ISO-1.1 ISO 15118-2
Retrieve certificates from Charging Station
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 58/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature
no.
Feature
TC_M_14 V2GRootCertificate M M
TC_M_15 V2GCertificateChain M M
TC_M_16 MORootCertificate M M
Install CA certificate
TC_M_03 V2GRootCertificate M M
TC_M_04 MORootCertificate M M
TC_M_101
(2.1)
OEMRootCertificate C M ISO-1.2 ISO 15118-20
Get Charging Station Certificate status
TC_M_24 Success M M
TC_M_25 Rejected M
Clear Customer Information
TC_N_63 Clear and report - customerCertificate C C If Support for retrieving / deleting
CustomerInformation - CustomerCertificate is
supported.
ISO-4
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 59/68 Part 5 - Certification Profiles

4.6. Test Cases Bidirectional Power Transfer (2.1)
TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature
no.
Feature
TC_Q_109 Central V2X control with dynamic CSMS setpoint - push M M
TC_Q_110 Central V2X control with dynamic CSMS setpoint - pull M M
TC_Q_122 Local V2X control for load balancing - threshold
validations
C BPT-2 Support for local loadbalancing
TC_Q_123 Local V2X control for load balancing - not supported C NOT BPT-
2
TC_Q_124 Local V2X control for load balancing - good flow M
TC_Q_128 Going offline during V2X operation -
invalidAfterOfflineDuration = true
M
TC_K_101 Set Charging Profile - Change operation mode M M
TC_Q_100 V2X Authorisation - V2X Tx Measurands defined M
TC_B_116 Reset ImmediateAndResume - With Ongoing
Transaction and SmartCharging - resuming
energytransfer
M
V2X Authorisation - ISO15118-20
TC_Q_101 Processing charging needs C ISO-1.2 ISO 15118-20
TC_Q_102 Charging only (V2X control) before starting V2X -
Allowed Energy Transfer modes omitted
C C CSMS: ISO 15118 is supported CS: ISO-
1.2
CS: ISO 15118-20
TC_Q_103 Charging needs rejected C M For CSMS: ISO 15118 is supported CS: ISO-
1.2 AND
NOT ISO-6
CS: ISO 15118-20
TC_Q_130 has ISO15118ServiceRenegotiationSupport - Charging
needs rejected
C ISO-1.2
AND ISO-6
ISO 15118-20
TC_Q_104 Scheduled Control C ISO-1.2 ISO 15118-20
TC_Q_107 Charging only (V2X control) before starting V2X C C For CSMS: ISO 15118 is supported CS: ISO-
1.2
CS: ISO 15118-20
TC_Q_108 Central V2X control with charging schedule C ISO 15118 is supported
Frequency support
TC_Q_117 Central V2X control - push C C BPT-1 Frequency support
TC_Q_118 Central V2X control - Duration expired C BPT-1 Frequency support
TC_Q_119 Local V2X control - Charging profile validations C BPT-1 Frequency support
TC_Q_120 Local V2X control - AFRR support C C BPT-1 Frequency support
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 60/68 Part 5 - Certification Profiles

TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature
no.
Feature
TC_Q_121 Local V2X control C BPT-1 Frequency support
4.7. Test Cases DER Control (2.1)
TC Id OCPP Compliance Testing Tool scenario Conf. Test
for
Charging
Station
Conf. test
for CSMS
Condition / remark Feature
no.
Feature
TC_R_100 Starting a V2X session with DER control in EVSE -
Persistent DERControls
M
TC_R_101 Starting a V2X session with DER control in EVSE -
Device model
M
TC_R_102 Configure DER control settings at CS - clearing
controlTypes
M
TC_R_103 Configure DER control settings at CS - validations M
TC_R_104 Configure DER control settings at CS - superseding
future DER control
M
TC_R_105 Configure DER control settings at CS - superseding
active DER control
M
TC_R_106 Configure DER control settings at CS - Active DER
control supersedes new DER control
M
TC_R_107 Configure DER control settings at CS M
TC_R_108 Charging station reporting a DER event C M ISO-1.2 AND HFS-4 ISO-1.2
AND HFS-
4
ISO 15118-20
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 61/68 Part 5 - Certification Profiles


### Chapter 5. OCPP 2.1 Mandatory Controller components per profile

*_Source: Page 64 - 64_*

Chapter 5. OCPP 2.1 Mandatory Controller components per
profile
Controller components contain variables that describe the supported features of a Charging Station and influence its behavior. In
OCPP 2.1 we have configuration variables that are required or optional, but these are contained by controller components.
Functionalities cannot be tested without the accompanying controller component, so for certification the following controller
components are mandatory:
Certification Profile / Feature Id Description
Core OCPPCommCtrlr
TxCtrlr
DeviceDataCtrlr
ClockCtrlr
SecurityCtrlr
SampledDataCtrlr
AlignedDataCtrlr
AuthCtrlr
BatterySwapCtrlr (for product subtype Battery Swapping Charging Station only)
Core: DM-0 MonitoringCtrlr
Core: LA-0 LocalAuthListCtrlr
Core: UI-0 TariffCostCtrlr
DisplayMessageCtrlr
Core: R-0 ReservationCtrlr
Core: P-0 (2.1) TariffCostCtrlr
WebPaymentsCtrlr
Advanced Security SecurityCtrlr (already part of Core)
Smart Charging SmartChargingCtrlr
ISO 15118 Support ISO15118Ctrlr
SmartChargingCtrlr
Bidirectional Power Transfer (2.1) V2XChargingCtrlr
DER Control (2.1) DCDERCtrlr (depending on AC vs DC Charging Station)
ACDERCtrlr (depending on AC vs DC Charging Station)
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 62/68 Part 5 - Certification Profiles


### Chapter 6. Appendix A: additional questions for the Protocol Implementation Conformance Statement

*_Source: Page 65 - 65_*

Chapter 6. Appendix A: additional questions for the Protocol
Implementation Conformance Statement
To perform the certification testing, the test lab need some additional information (for the test selection). This concerns the
following questions:
6.1. Questions for Charging Stations
Id Additional questions for lab testing
AQ-1 Can the last CSMSRootCertificate be removed? 
AQ-2 Does the Charging Station have a cable lock, which prevents the EV driver to connect the EV and EVSE
before authorization? 
AQ-3 Can the last ChargingStationCertificate be removed (via other means than OCPP)? 
AQ-4 Is there at least one unsupported NumberOfPhases? 
AQ-5 Does the Charging Station have at least one hardWired monitor? If yes, which hardWired monitor should
be used for the certification test
AQ-6 Does the Charging Station have a pre-configured monitor? If yes, which pre-configured monitor should
be used for the certification test
AQ-7 Is your Charging Station able to download firmware while there is an ongoing transaction?
AQ-8 Does your Charging Station enforce a selection of EVSE (by design) prior to authorization?
AQ-9 Does your Charging Station support charging an EV using IEC 61851-1 (Mode 3)?
AQ-10 Does your Charging Station support setting a Delta monitor on the WriteOnly component.variable
SecurityCtrlr.BasicAuthPassword?
AQ-11 Does your Charging Station support a combined charging station Certificate (for both OCPP and ISO
15118)
AQ-18 Does your Charging Station have at least one connector with an (automatic) mechanized locking
mechanism on Charging Station side? (this is always true for connectorTypes; sType2 and sType3)
AQ-19 Does your Charging Station have at least 1 unsupported language code?
AQ-20 Does your Charging Station support more than 1 language?
6.2. Questions for CSMSs
Id Additional questions for lab testing
AQ-12 Is a FullInventory requested during onboarding / booting test cases?
AQ-13 Does your CSMS support Absolute values for the following Charging Profiles:
AQ-13.1 TxDefaultProfile
AQ-13.2 ChargingStationMaxProfile
AQ-14 Does your CSMS support Recurring values for the following Charging Profiles:
AQ-14.1 TxDefaultProfile
AQ-14.2 ChargingStationMaxProfile
AQ-16 Does the CSMS reject unknown Charging Stations during websocket connection setup?
AQ-17 Can your CSMS be configured to first respond to a BootNotificationRequest with status Pending or
Rejected?
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 63/68 Part 5 - Certification Profiles


### Chapter 7. Appendix B: Hardware feature set

*_Source: Page 66 - 66_*

Chapter 7. Appendix B: Hardware feature set
The table below gives an overview of the hardware feature set Ids that are used for determining whether test cases are needed /
applicable for certification.
Table 4. Hardware features
Id Hardware Feature
HFS-1 Charging Station has a detachable cable
HFS-2 Charging Station has a fixed cable
HFS-3 Charging Station has AC support 
HFS-4 Charging Station has DC support 
HFS-5 Charging Station has 1 phase support
HFS-6 Charging Station has 2 phase support
HFS-7 Charging Station has 3 phase support
HFS-8 No. EVSEs of Charging Station
HFS-9 Communication technology  
HFS-10 RFID readers 
HFS-11 DC power level (kW)
HFS-12 Number of displays
HFS-13 Charging Station has Battery Swapping support
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 64/68 Part 5 - Certification Profiles


### Chapter 8. Appendix C: Features vs. OCPP use cases

*_Source: Page 67 - 70_*

Chapter 8. Appendix C: Features vs. OCPP use cases
The table below gives an overview of the use cases / configuration variables that the features are applicable for / referring to in the
OCPP 2.1 Specificiation Part 2.
Table 5. Optional features vs. related use cases
Id Feature Related use cases
Core
C-01 Support for offline authorization of transactions C15, C10, C11, C12
C-02 Support for allowing Offline Authorization for Unknown Ids
(OfflineTxForUnknownIdEnabled)
C15
C-03 Support for maximizing energy for invalid ids (MaxEnergyOnInvalidId) C15, E05
C-04 Support to limit StatusNotifications (MinimumStatusDuration) Configuration Variable for G01
C-06 Authorization status after cable disconnected on EV side
(StopTxOnEVSideDisconnect)
C-06.1 Support for maintaining authorization when cable disconnected on EV side E10
C-06.2 Support for not maintaining authorization when cable disconnected on EV side E09
C-07 Support for using a Master Pass for charging stations with UI (MasterPassGroupId) C16
C-08 Support for using a Master Pass for charging stations without UI (MasterPassGroupId) C16
C-09 Supported Transaction Start points (TxStartPoint) E01
C-09.1 Start transaction options - EVConnected E01-S2
C-09.2 Start transaction options - Authorized E01-S3
C-09.3 Start transaction options - DataSigned E01-S4
C-09.4 Start transaction options - PowerPathClosed E01-S5
C-09.5 Start transaction options - EnergyTransfer E01-S6
C-09.6 Start transaction options - ParkingBayOccupancy E01-S1
C-10 Supported Transaction Stop points (TxStopPoint) E06
C-10.1 Stop transaction options - EVConnected E06-S2
C-10.2 Stop transaction options - Authorized E06-S3
C-10.3 Stop transaction options - PowerPathClosed E06-S5
C-10.4 Stop transaction options - EnergyTransfer E06-S6
C-10.5 Stop transaction options - ParkingBayOccupancy E06-S1
C-12 Unlocking of connector when cable disconnected on EV side
(UnlockOnEVSideDisconnect)
E09, E10
C-12.1 Support for unlocking connector when cable disconnected on EV side E09, E10
C-12.2 Support for not unlocking when cable disconnected on EV side E09, E10
C-13 Support for Reset per EVSE (AllowReset) B11, B12
C-14 Support for retrieving / deleting CustomerInformation - CustomerIdentifier N09, N10
C-20 Allowing New Sessions Pending a FirmwareUpdate
(AllowNewSessionsPendingFirmwareUpdate)
Configuration Variable for L01
C-21 Support for queuing all or only Transaction related messages until they are delivered to
the CSMS (QueueAllMessages)
Optional
Time related settings
C-23 Supported time sources (TimeSource)
C-25 Support for setting a TimeOffset (TimeOffset) Configuration Variable (B05,
B06)
C-26 Support for setting the TimeZone (TimeZone) Configuration Variable (B05,
B06)
C-28 Toggle sending clock aligned meter values when a transaction is ongoing / Idle
(AlignedDataSendDuringIdle)
Configuration Variable for J01
C-29 TriggerMessage F06
C-29.1 Trigger message - MeterValues F06
C-29.2 Trigger message - TransactionEvent F06
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 65/68 Part 5 - Certification Profiles

Id Feature Related use cases
C-29.3 Trigger message - LogStatusNotification F06
C-29.4 Trigger message - FirmwareStatusNotification F06
C-29.5 Trigger message - StatusNotification F06
C-29.6 Trigger message - BootNotification F06
C-29.7
(2.1)
Trigger message - CustomTrigger F06
Authorization options for local start
C-30 Authorization - using RFID ISO14443 C01
C-31 Authorization - using RFID ISO15693 C01
C-32 Authorization - using KeyCode C04
C-33 Authorization - using locally generated id C06
C-34 Authorization - MacAddress C06
C-35 Authorization - NoAuthorization C02
Authorization options for remote start (mandatory to support at least one)
C-36 Authorization - using RFID ISO14443 C01
C-37 Authorization - using RFID ISO15693 C01
C-38 Authorization - using centrally, in the CSMS (or other server) generated id C05
C-39 Authorization - NoAuthorization C02
C-40 Supported MeterValue Measurands
(SampledDataTx{Started,Updated,Ended}Measurands,
AlignedDataMeasurands)
J01, J02
C-41 Supported Cipher Suites See requirement A00.FR.318,
A00.FR.319, A00.FR.421,
A00.FR.422
C-42 Signed Metervalues (SampledDataSignReadings) J01, J02
C-43 Install and activate Firmware with ongoing transaction(s)
(AllowNewSessionsPendingFirmwareUpdate)
Configuration Variable for L01
C-47 Support for falling back to default OCPP reconnection mechanism when
NetworkConnection profile connection has failed
B10 (FR.07)
C-48 Authorization of remote start (AuthorizeRemoteStart) F01, F02
C-48.1 Option for authorization in case of a remote start F01, F02
C-48.2 Option for no authorization in case of a remote start F01, F02
C-58 Option for disabling remote authorization (DisableRemoteAuthorization) Configuration Variable (B05,
B06)
C-49 Authorization Cache (AuthCacheEnabled) C10, C11, C12
C-59 Option for disabling remote authorization for cached invalid idTokens
(AuthCacheDisablePostAuthorize)
Configuration Variable for C10,
C12
C-51 Configurable TxStartPoint Configuration Variable for E01
C-52 Configurable TxStopPoint Configuration Variable for E06
C-53 Support for lifetime cached token (AuthCacheLifeTime) Configuration Variable for C10
C-54 Supported policies for replacing cached entries (AuthCachePolicy) Configuration Variable for C10,
C11, C12
C-56 Support for providing the SummaryInventory B07
C-57 Support for cancelling ongoing log file upload N01 (AcceptedCanceled)
C-60 Support for cancelling ongoing firmware update L01, L02 (AcceptedCanceled)
C-61 Security Profile 1 - Unsecured Transport with Basic Authentication A01
C-62 (2.1) Support for resuming transactions (ImmediateAndResume) B13
C-63 (2.1) Support for transaction limits E16
C-64 (2.1) Support for resuming transaction after interruption E17
C-65 (2.1) Support for DataCollectorLog N01
Authorization options for local stop
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 66/68 Part 5 - Certification Profiles

Id Feature Related use cases
C-70 Authorization - using RFID ISO14443 C01
C-71 Authorization - using RFID ISO15693 C01
C-72 Authorization - using KeyCode C04
C-75 Authorization - NoAuthorization C02
C-76 (2.1) Support for Battery Swapping Stations S01-S04
Reservation
R-0 Support for Reservation H01-H04
R-1 Support for reservations of connectorType H01.S3
R-2 Support for reservations of unspecified EVSE (ReservationNonEvseSpecific) H01.S1
R-3 Support for disabling Reservations (ReservationEnabled) Configuration Variable for H01
Advanced Device Management
DM-0 Support for Advanced Device Management Optional
DM-3 Queue notifyEventRequest messages for specific severities
(OfflineMonitoringEventQueuingSeverity)
Configuration Variable for N07
Local Authorization List Management
LA-0 Support for Local Authorization List Management D01-D02
LA-1 Authorization list support (LocalAuthListEnabled) Configuration Variable for C13
LA-2 Support for GetLocalListVersion D02
LA-3 Option for disabling remote authorization for invalid idTokens stored at the Local
Authorization List (LocalAuthListDisablePostAuthorize)
Configuration Variable for C14
Advanced User Interface
UI-0 Support for Advanced User Interface N02-N15
UI-1 Supported message priorities (DisplayMessageSupportedPriorities) O01
UI-2 Supported message formats (DisplayMessageSupportedFormats) O01
Payment (2.1)
P-0 Support for Payment C17-C-25
P-1 Support for Tariff conditions I07
P-2 Supported Payment options
P-2.1 Payment by prepaid card C17
P-2.2 Integrated payment terminal C18
P-2.3 Stand alone payment terminal C24
P-2.4 QR code payment C25
Advanced Security
AS-2 Additional root certificate check mechanism implemented
(AdditionalRootCertificateCheck)
Configuration Variable for M05
AS-3 Update Charging Station Certificate - CertificateSignedRequest Timeout
(CertSigningWaitMinimum,CertSigningRepeatTimes)
Configuration Variable for A02,
A03
Smart Charging
SC-2 Supported charging rate units (ChargingScheduleChargingRateUnit) K01
SC-3 (2.1) Support for limiting based on SoC (limitAtSoC) K01
SC-4 Support for TxDefaultProfile on EVSEID #0 K01
SC-5 (2.1) Support for using local time (useLocalTime)
SC-5.1 TimeOffset K01
SC-5.2 TimeZone K01
SC-6 (2.1) Support for using priority charging (PriorityCharging) K01
SC-7 (2.1) Support for using randomized delays (randomizedDelay) K01
SC-8 (2.1) Support for dynamic charging profiles K28-K29
SC-9 (2.1) Support for operationMode Idle Q10
SC-9.1
(2.1)
Support for operationMode Idle with EvseSleep Q10
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 67/68 Part 5 - Certification Profiles

Id Feature Related use cases
EMS Control (2.1)
SC-10
(2.1)
Support for EMS Control K23-K27
ISO 15118 support
ISO-1
(2.1)
Supported ISO 15118 version
ISO-1.1 ISO 15118-2 C01-C08, M01-M06
ISO-1.2 ISO 15118-20 C01-C08, M01-M07, Q01
ISO-4 Support for retrieving / deleting CustomerInformation - CustomerCertificate N09/N10
ISO-5 Charging Station can provide a contract certificate that it cannot validate to the CSMS Configuration Variable for C07
ISO-6
(2.1)
Support for ServiceRenegotiation Q01-Q02
Bidirectional Power Transfer (2.1)
BPT-1 Frequency support Q07-Q08
BPT-2 Support for local loadbalancing (LocalLoadBalancing) Q09
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 68/68 Part 5 - Certification Profiles



---

## Technical Analysis & Implementation Guide

> **Note**: This section provides technical analysis and implementation guidance for OCPP 2.1 certification profiles.

### Understanding OCPP Certification

#### What is Certification?

OCPP certification is a **formal verification process** that ensures an implementation correctly implements the OCPP 2.1 specification. Certification provides:

1. **Interoperability**: Certified implementations work together reliably
2. **Quality Assurance**: Validates correct protocol behavior
3. **Market Recognition**: Demonstrates compliance with industry standards
4. **Customer Confidence**: Reduces integration risk for buyers

**Key Concepts**:
- **Profile**: A set of related functionality (Core, Security, Smart Charging, etc.)
- **Feature**: A specific functionality tested with one or more test cases
- **Test Case**: A sequence of messages to verify a use case
- **System Under Test (SUT)**: The Charging Station or CSMS being tested

#### Certification Profile Structure

```
┌─────────────────────────────────────────────────────────┐
│                   OCPP 2.1 Certification                │
├─────────────────────────────────────────────────────────┤
│  Core Profile (REQUIRED)                                │
│  └──────────── Basic OCPP functionality                  │
├─────────────────────────────────────────────────────────┤
│  Optional Profiles                                      │
│  ├─ Advanced Security (TLS client certificates)         │
│  ├─ Smart Charging (load management)                   │
│  ├─ ISO 15118 Support (vehicle-to-grid)                 │
│  ├─ Bidirectional Power Transfer (V2G)                  │
│  └─ ... (other specialized profiles)                    │
└─────────────────────────────────────────────────────────┘
```

### Key Certification Profiles

#### 1. Core Profile (Required)

**Mandatory for ALL implementations**

Key features:
- Boot sequence and configuration
- Basic authorization (RFID, start button)
- Transaction handling
- Remote control (start/stop, unlock)
- Security events and certificate management
- Firmware updates
- Meter values

**Implementation Priority**: ⭐⭐⭐⭐⭐ (MUST implement)

#### 2. Advanced Security Profile

**Enhanced security with mutual TLS**

Additional features:
- Client-side certificates
- Security profile upgrades
- Full certificate chain validation

**Implementation Priority**: ⭐⭐⭐⭐ (Highly recommended for production)

#### 3. Smart Charging Profile

**Load management and optimization**

Key features:
- Charging profiles (SetChargingProfile)
- Composite schedule calculation
- External limits handling (EMS integration)
- Dynamic profiles (NEW in 2.1)

**Implementation Priority**: ⭐⭐⭐⭐ (Important for most deployments)

#### 4. ISO 15118 Support Profile

**Plug & Charge with V2G**

Key features:
- Contract certificate handling
- Automatic authorization
- V2G communication (ISO 15118-20)
- Service renegotiation (NEW in 2.1)

**Implementation Priority**: ⭐⭐⭐ (Specialized use cases)

#### 5. Payment Profile (OCPP 2.1)

**Integrated payment processing**

Key features:
- Prepaid card support
- Payment terminal integration
- QR code payment
- Tariff and cost calculation

**Implementation Priority**: ⭐⭐⭐ (Public charging stations)

### Certification Readiness Checklist

Before applying for certification:

**Core Requirements**:
- [ ] All Core Profile test cases passing
- [ ] Security Profile 3 implemented (mutual TLS)
- [ ] Device Model matches specification
- [ ] All mandatory variables present
- [ ] Proper certificate handling
- [ ] Firmware update support

**Operational**:
- [ ] Offline operation supported
- [ ] Error handling correct (proper CallError codes)
- [ ] Time synchronization (NTP or mobile network)
- [ ] Heartbeat and StatusNotification working
- [ ] Meter values accurate (within tolerance)
- [ ] Transaction state machine correct

### Implementation Recommendations

#### Start Simple, Build Up

```rust
pub struct OcppImplementation {
    pub core: CoreProfileImplementation,
    pub advanced_security: Option<AdvancedSecurityProfile>,
    pub smart_charging: Option<SmartChargingProfile>,
}

impl OcppImplementation {
    pub fn new_basic() -> Self {
        OcppImplementation {
            core: CoreProfileImplementation::new(),
            advanced_security: None,
            smart_charging: None,
        }
    }

    pub fn with_security(mut self) -> Self {
        self.advanced_security = Some(AdvancedSecurityProfile::new());
        self
    }

    pub fn with_smart_charging(mut self) -> Self {
        self.smart_charging = Some(SmartChargingProfile::new());
        self
    }
}
```

#### Use Feature Flags

```toml
[features]
default = ["core"]
core = []
advanced_security = ["tls", "certificates"]
smart_charging = []
iso15118 = ["advanced_security"]
payment = ["tariff"]
```

### Common Pitfalls to Avoid

1. ❌ **Using Security Profile 1 in production**
   - Profile 1 (Basic Auth only) is NOT secure
   - Use at minimum Profile 2 (TLS)
   - Profile 3 (mutual TLS) recommended for field deployments

2. ❌ **Partial feature implementation**
   - If you support a feature, implement ALL test cases
   - Partial implementation = failed certification

3. ❌ **Missing Device Model components**
   - Certification tests require specific components
   - Verify Controller, EVSE, Connector components exist

4. ❌ **Hardcoded configuration**
   - Use SetVariables/GetVariables for configuration
   - Support all mandatory variables

5. ❌ **Incorrect transaction state management**
   - Track transaction state carefully
   - Support all TransactionEventType values
   - Handle transaction resumption after reboot

### References to Other Parts

- **Part 0**: Introduction and basic implementation guidance
- **Part 1**: Architecture and Device Model details
- **Part 2**: Complete functional block specifications
- **Part 4**: OCPP-J WebSocket protocol implementation
- **Part 6**: Detailed test cases for certification

---

**Analysis completed**: 2025-01-25
**Ralph Loop Iteration**: 3
**Phase**: Technical Analysis - Part 5 Certification Profiles
