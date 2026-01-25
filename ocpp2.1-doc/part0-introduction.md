# Part 0: Introduction
> **Source**: `OCPP-2.1_edition2_part0_introduction.pdf`
> **Total Pages**: 17

---

## Table of Contents

- OCPP 2.1: Part 0 - Introduction (Page 1)
- Table of Contents (Page 2)
- Disclaimer (Page 3)
- Version History (Page 4)
- Chapter 1. Introduction (Page 5)
- Chapter 2. New functionality in OCPP 2.1 (Page 8)
- Chapter 3. OCPP 2.1 Documentation Structure (Page 10)
- Chapter 4. Basic implementation of OCPP 2.1 (Page 17)

---

## Content


### OCPP 2.1: Part 0 - Introduction

*_Source: Page 1 - 1_*

OCPP 2.1
Part 0 - Introduction
Edition 2, 2025-12-03


### Table of Contents

*_Source: Page 2 - 2_*

Table of Contents
Disclaimer . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  1
Version History . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  2
1. Introduction. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  3
1.1. OCPP version 2.1 . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  3
1.2. Terms and abbreviations. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  3
1.3. References . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  5
2. New functionality in OCPP 2.1 . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  6
3. OCPP 2.1 Documentation Structure. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  8
3.1. Overview of Specification Parts . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  8
3.2. Functional Blocks . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  9
3.3. All Functional Blocks and use cases . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  10
4. Basic implementation of OCPP 2.1 . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  15


### Disclaimer

*_Source: Page 3 - 3_*

Disclaimer
Copyright © 2010 – 2025 Open Charge Alliance. All rights reserved.
This document is made available under the *Creative Commons Attribution-NoDerivatives 4.0 International Public License*
(https://creativecommons.org/licenses/by-nd/4.0/legalcode).
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 1/15 Part 0 - Introduction


### Version History

*_Source: Page 4 - 4_*

Version History
Version Date Description
2.1 Edition 2 2025-12-03 OCPP 2.1 Edition 2. All errata from OCPP 2.1 Part 0 until and including Errata 2025-
11 have been merged into this version of the specification.
2.1 Edition 1 2025-01-23 OCPP 2.1 Edition 1
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 2/15 Part 0 - Introduction


### Chapter 1. Introduction

*_Source: Page 5 - 7_*

Chapter 1. Introduction
Electric Vehicles (EVs) are becoming the new standard for mobility all over the world. This development is only possible with a
good coverage of Charging Stations. To advance the roll out of charging infrastructure, open communication standards play a key
role: to enable switching from charging network without necessarily replacing all the Charging Stations, to encourage innovation
and cost effectiveness and to allow many and diverse players participate in this new industry.
Additionally, the EV charging infrastructure is part of the Smart Grid, a larger and still evolving ecosystem of actors, devices and
protocols. In this Smart Grid ecosystem, open communication standards are key enablers for bidirectional power flows, real time
information exchange, demand control and eMobility services.
The Open Charge Point Protocol (OCPP) is the industry-supported de facto standard for communication between a Charging
Station and a Charging Station Management System (CSMS) and is designed to accommodate any type of charging technique.
OCPP is an open standard with no cost or licensing barriers for adoption.
1.1. OCPP version 2.1
This specification defines version 2.1 of OCPP.
Version 2.1 is an extension of OCPP 2.0.1. OCPP 2.1 has its own JSON schemas, but the schemas are OCPP 2.0.1 schemas that
have been extended with optional fields that are used by OCPP 2.1 functionality. With the minor exceptions mentioned below, all
application logic developed for OCPP 2.0.1 will continue to work in OCPP 2.1 without any changes. The new features of OCPP 2.1,
of course, require new application logic.
Use case A02 & A03
The application logic in a CSMS for OCPP 2.0.1 for use cases A02 & A03 requires a small change in order to work in OCPP
2.1.
The SignCertificateRequest message has been extended with a requestId field, such that the resulting
CertificateSignedRequest message can be accurately mapped to the request that initiated it. Use of requestId is optional for
Charging Station, but when present, CSMS will have to use it in the subsequent CertificateSignedRequest message.
Note, that the updated application logic remains valid to use in OCPP 2.0.1.
Use case N02
The application logic in a Charging Station for OCPP 2.0.1 for use case N02 requires a small change in order to work for
OCPP 2.1.
The message NotifyMonitoringReportRequest has been extended with a required field in VariableMonitoringType:
eventNotificationType. Charging Station has to provide this field. It provides essential information to CSMS about the type of
monitor (HardWiredMonitor, PreconfiguredMonitor, CustomMonitor) that was missing in OCPP 2.0.1. Existing OCPP 2.0.1
logic in a CSMS that is not aware of this new field, will continue to work.
1.2. Terms and abbreviations
This section contains the terminology and abbreviations that are used throughout this document.
1.2.1. Terms
Term Meaning
Charging Station The Charging Station is the physical system where an EV can be charged. A Charging Station has one or
more EVSEs.
Charging Station
Management
System (CSMS)
Charging Station Management System: manages Charging Stations and has the information for authorizing
Users for using its Charging Stations.
Electric Vehicle
Supply Equipment
(EVSE)
EVSE is considered as an independently operated and managed part of the Charging Station that can deliver
energy to one EV at a time.
Energy Management
System (EMS)
In this document this is defined as a device that manages the local loads (consumption and production)
based on local and/or contractual constraints and/or contractual incentives. It has additional inputs, such as
sensors and controls from e.g. PV, battery storage.
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 3/15 Part 0 - Introduction

1.2.2. Abbreviations
Term Meaning
CSO Charging Station Operator
CSMS Charging Station Management System
EMS Energy Management System.
EV Electric Vehicle
EVSE Electric Vehicle Supply Equipment
RFID Radio-Frequency Identification
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 4/15 Part 0 - Introduction

1.3. References
Table 1. References
Reference Description
[IEC61851-1] IEC 61851-1 2017: EV conductive charging system - Part 1: General requirements. https://webstore.iec.ch/
publication/33644
[IEC62559-2:2015] Definition of the templates for use cases, actor list and requirements list. https://webstore.iec.ch/
publication/22349
[ISO15118-1] ISO 15118-1 specifies terms and definitions, general requirements and use cases as the basis for the other
parts of ISO 15118. It provides a general overview and a common understanding of aspects influencing the
charge process, payment and load leveling. https://webstore.iec.ch/publication/9272
[OCPP1.5] http://www.openchargealliance.org/downloads/
[OCPP1.6] http://www.openchargealliance.org/downloads/
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 5/15 Part 0 - Introduction


### Chapter 2. New functionality in OCPP 2.1

*_Source: Page 8 - 9_*

Chapter 2. New functionality in OCPP 2.1
OCPP 2.1 introduces new functionality compared to OCPP 2.0.1.
The application logic for OCPP 2.0.1 remains valid, but will have to be extended to support the new features of OCPP 2.1.
Most important new features of OCPP 2.1 include support for ISO 15118-20 and extensive support for bidirectional power transfer
(V2X), and control of Charging Stations and EVs as Distributed Energy Resources (DER). New use cases have been added that
describe ad hoc payment, and Charging Stations can now do local cost calculation based on tariff information from CSMS.
Below is a list of sections of Part 2 of the specification that have new or updated functionality.
A Security
A02/A03 A requestId has been added to SignCertificateRequest. Added support for ISO 15118-20 certificates.
A05 Downgrading from security profile 3 to 2 is no longer prohibited.
B Provisioning
B09 SetNetworkProfileRequest has been extended with basicAuthPassword and identity.
B13 New use case to support resuming transaction after a reset.
C Authorization
Length of IdToken has been extended to 255 characters.
IdToken type is now a predefined list instead of enumeration to allow for easier extension.
C07/C08 ISO 15118 authorization use cases updated with ISO 15118-20 flows.
C10 Explicit requirement added about expiration in authorization cache.
C17 New use case for authorization with prepaid card.
C18-C23 New use cases for ad hoc payment with integrated payment terminal.
C24 New use case for ad hoc payment via stand-alone payment terminal.
C25 New use case for ad hoc payment via dynamic QR code.
E Transactions
E16 New use case for transactions with cost, energy, time, SoC limit.
E17 New use case for resuming a transaction after forced reboot.
F Remote Control
F06 Added CustomTrigger to TriggerMessageRequest.
F07 Net use case for remote start of transaction with limits.
G Availability
Availability notification using NotifyEventRequest for component Connector is now the preferred method, instead of
StatusNotification.
I Tariff and Cost
Introducing local cost calculation
I07-I11 New use cases to set default/user tariffs on charging station.
I12 New use case to report calculated cost during and at end of transaction.
J Metervalues
New metervalue location: Upstream.
New measurands for bidirectional charging.
K Smart Charging
New charging profile purposes PriorityCharging and LocalGeneration.
Added operationMode to ChargingSchedulePeriodType to facilitate bidirectional charging scenarios.
K01 Added dynamic charging profiles for frequent and unscheduled updates of limits.
K23-K27 New use cases for topologies with energy management systems.
K18-K20 New uses cases to support ISO 15118-20.
K21-K22 New uses cases for priority charging to allow user to overrule charging profile.
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 6/15 Part 0 - Introduction

M Certificate Management
M01 Updated use case for ISO 15118-20.
N Diagnostics
N01 Added support for data collector log on charging station.
N02 Added monitoring types TargetDelta and TargetDeltaRelative.
N07 Added severity to NotifyEventRequest.
N11-14 New use cases for optimized frequent periodic variable monitoring via an event stream. This utilizes the new
unconfirmed message type: SEND.
N15 Use case to set a frequent periodic monitoring via event stream.
O Display Message
O01 Added multi-language support.
Q Bidirectional Power Transfer
New section that describes control of bidirectional charging via charging profiles.
Q01-Q04 V2X control with centrally controlled charging profiles.
Q05-Q06 V2X control with externally controlled charging profiles.
Q07-Q08 Central can local frequency control.
Q09 Local load-balancing with V2X.
Q10-Q12 Idle state, offline and resuming after offline.
R DER Control
New section that describes grid control when EV and charging station are considered to be a Distributed Energy Resource
(DER).
U01 DER control in EVSE.
U02 DER control in EV.
U03 Hybrid DER control in both EVSE and EV.
U04 Configure DER controls in charging station.
U05 Charging station reporting a DER event.
S Battery Swapping
New section that describes how to control a battery swap station.
S01 Battery Swap Local Autorization
S02 Battery Swap Remote Start
S03 Battery Swap In/Out
S04 Battery Swap Charging
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 7/15 Part 0 - Introduction


### Chapter 3. OCPP 2.1 Documentation Structure

*_Source: Page 10 - 16_*

Chapter 3. OCPP 2.1 Documentation Structure
3.1. Overview of Specification Parts
For readability and implementation purposes, OCPP 2.1 is divided in seven parts.
Table 2. Parts
Part 0 Introduction (this document)
Part 1 Architecture & Topology
Part 2 Specification:
Use Cases and Requirements, Messages, Data Types and Referenced Components and Variables
Appendices:
Security Events, Standardized Units of Measure, Components and Variables
Part 3 Schemas
Part 4 Implementation Guide JSON
Part 5 Certification Profiles
Part 6 Test Cases
The OCPP 2.1 specification is written using a structure, based on [IEC62559-2:2015]: "Use case methodology - Part 2: Definition of
the template for use cases, actor list and requirements list".
Part 2, the specification, is divided into 'Functional Blocks'. These Functional Blocks contain use cases and requirements.
Messages, Data Types and Referenced Components and Variables are described at the end of the document. The Appendices can
be found in the separate document: Part 2 - Appendices.
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 8/15 Part 0 - Introduction

3.2. Functional Blocks
OCPP 2.1 consists of the following Functional Blocks.
Table 3. Functional Blocks
Clause Functional Block Title Description
A. Security This Functional Block describes a security specification for the OCPP
protocol.
B. Provisioning This Functional Block describes all the functionalities that help a CSO
provision their Charging Stations, allowing them to be registered and
accepted on their network and retrieving basic configuration information
from these Charging Stations.
C. Authorization This Functional Block describes all the authorization related functionality:
AuthorizeRequest message handling/behavior and Authorization Cache
functionality.
D. Local Authorization List Management This Functional Block describes functionality for managing the Local
Authorization List.
E. Transactions This Functional Block describes the basic OCPP Transaction related
functionality for transactions that are started/stopped on the Charging
Station.
F. Remote Control This Functional Block describes three types of use cases for remote control
management from the CSMS: Remote Transaction Control, Unlocking a
Connector and Remote Trigger.
G. Availability This functional Block describes the functionality of sending status
notification messages.
H. Reservation This Functional Block describes the reservation functionality of a Charging
Station.
I. Tariff and Cost This Functional Block provides tariff and cost information to an EV Driver,
when a Charging Station is capable of showing this on a display. Before a
driver starts charging tariff information needs to be given, detailed prices
for all the components that make up the tariff plan applicable to this driver
at this Charging Station. During charging the EV Driver needs to be shown
the running total cost, updated at a regular, fitting interval. When the EV
Driver stops charging the total cost of this transaction needs to be shown.
J. Metering This Functional Block describes the functionality for sending meter values,
on a periodic sampling and/or clock-aligned timing basis.
K. Smart Charging This Functional Block describes all the functionality that enables the CSO
(or indirectly a third party) to influence the charging current/power of a
charging session, or set limits to the amount of power/current a Charging
Station can offer to an EV.
L. Firmware Management This Functional Block describes the functionality that enables a CSO to
update the firmware of a Charging Station.
M. Certificate Management This Functional Block provides the installation and update of certificates.
N. Diagnostics This Functional Block describes the functionality that enables a CSO to
request and track the upload of a diagnostics file from a Charging Station,
and to manage the monitoring of Charging Station data.
O. Display Message With the DisplayMessage feature OCPP enables a CSO to display a
message on a Charging Station, that is not part of the firmware of the
Charging Station. The CSO gets control over these messages: the CSO can
set, retrieve (get), replace and clear messages.
P. Data Transfer This Functional Block describes the functionality that enables a party to add
custom commands to OCPP, enabling custom extension to OCPP.
Q Bidirectional Power Transfer This Functional block extends Smart Charging with bidirectional power
transfer (V2X).
R DER Control This Functional Block describes how charging stations and EVs can be
controlled as Distributed Energy Resources. It provides functions to
configure grid code parameters on a charging station via CSMS. It is
designed to support DER settings from IEC 61850 and IEEE 2030.5 on the
grid side, and ISO 15118-20 Amendment 1 on the EV side.
S Battery Swapping This Functional block describes how to deal with battery swap stations in
OCPP and adds the BatterySwap message.
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 9/15 Part 0 - Introduction

3.3. All Functional Blocks and use cases
The following table shows the full list of use cases supported by OCPP 2.1 and which use cases were already supported by OCPP
1.6 [OCPP1.6] and OCPP 2.1 .
Clause Functional Block UC ID Use case name OCPP 1.6 New in
OCPP
2.0.1
New in
OCPP 2.1
A Security A01 Update Charging Station Password for HTTP
Basic Authentication
o
A02 Update Charging Station Certificate by
request of CSMS
o
A03 Update Charging Station Certificate initiated
by the Charging Station
o
A04 Security Event Notification o
A05 Upgrade Charging Station Security Profile o
B Provisioning B01 Cold Boot Charging Station o
B02 Cold Boot Charging Station - Pending o
B03 Cold Boot Charging Station - Rejected o
B04 Offline Behavior Idle Charging Station o
B05 Set Variables o
B06 Get Variables o
B07 Get Base Report o
B08 Get Custom Report o
B09 Setting a new NetworkConnectionProfile o
B10 Migrate to new CSMS o
B11 Reset - Without Ongoing Transaction o
B12 Reset - With Ongoing Transaction o
B13 Reset - With Ongoing Transaction - Resuming
Transaction
o
C Authorization C01 EV Driver Authorization using RFID o
C02 Authorization using a start button o
C03 Authorization using credit/debit card o
C04 Authorization using PIN-code o
C05 Authorization for CSMS initiated transactions o
C06 Authorization using local id type o
C07 Authorization using Contract Certificates o
C08 Authorization at EVSE using ISO 15118
External Identification Means (EIM)
o
C09 Authorization by GroupId o
C10 Store Authorization Data in the Authorization
Cache
o
C11 Clear Authorization Data in Authorization
Cache
o
C12 Start Transaction - Cached Id o
C13 Offline Authorization through Local
Authorization List
o
C14 Online Authorization through Local
Authorization List
o
C15 Offline Authorization of unknown Id o
C16 Stop Transaction with a Master Pass o
C17 Authorization with prepaid card o
C18 Authorization using locally connected
payment terminal
o
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 10/15 Part 0 - Introduction

Clause Functional Block UC ID Use case name OCPP 1.6 New in
OCPP
2.0.1
New in
OCPP 2.1
C19 Cancelation prior to transaction o
C20 Cancelation after start of transaction, before
costs have been incurred.
o
C21 Settlement at end of transaction o
C22 Settlement is rejected or fails o
C23 Increasing authorization amount o
C24 Ad hoc payment via stand-alone payment
terminal
o
C25 Ad hoc payment via static or dynamic QR
code
o
D LocalAuthorizationList D01 Send Local Authorization List o
D02 Get Local List Version o
E Transactions E01 Start Transaction Options o
E02 Start Transaction - Cable Plugin First o
E03 Start Transaction - IdToken First o
E04 Transaction started while Charging Station is
offline
o
E05 Start Transaction - Id not Accepted o
E06 Stop Transaction Options o
E07 Transaction locally stopped by IdToken o
E08 Transaction stopped while Charging Station
is offline
o
E09 When cable disconnected on EV-side: Stop
Transaction
o
E10 When cable disconnected on EV-side:
Suspend Transaction
o
E11 Connection Loss During Transaction o
E12 Inform CSMS of an Offline Occurred
Transaction
o
E13 Transaction related message not accepted
by CSMS
o
E14 Check transaction status o
E15 End of charging process o
E16 Transactions with fixed cost, energy, SoC or
time
o
E17 Resuming transaction after forced o
E18 Battery Swapping o
F RemoteControl F01 Remote Start Transaction - Cable Plugin First o
F02 Remote Start Transaction - Remote Start First o
F03 Remote Stop Transaction o
F04 Remote Stop ISO 15118 charging from CSMS o
F05 Remotely Unlock Connector o
F06 Trigger Message o
F07 Remote start with fixed cost, energy or time o
G Availability G01 Status Notification o
G02 Heartbeat o
G03 Change Availability EVSE o
G04 Change Availability Charging Station o
G05 Lock Failure o
H Reservation H01 Reservation o
H02 Cancel Reservation o
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 11/15 Part 0 - Introduction

Clause Functional Block UC ID Use case name OCPP 1.6 New in
OCPP
2.0.1
New in
OCPP 2.1
H03 Use a reserved EVSE o
H04 Reservation Ended, not used o
I Tariff and Costs I01 Show EV Driver-specific tariff information o
I02 Show EV Driver running total cost during
charging
o
I03 Show EV Driver final total cost after charging o
I04 Show fallback tariff information o
I05 Show fallback total cost message o
I06 Update Tariff Information During Transaction o
I07 Local Cost Calculation - Set Default Tariff o
I08 Local Cost Calculation - Receive User Tariff o
I09 Local Cost Calculation - Get Tariffs o
I10 Local Cost Calculation - Clear Tariffs o
I11 Local Cost Calculation - Change transaction
tariff
o
I12 Local Cost Calculation - Cost Details of
Transaction
o
J Metering J01 Sending Meter Values not related to a
transaction
o
J02 Sending transaction related Meter Values o
J03 Charging Loop with metering information
exchange
o
K SmartCharging K01 SetChargingProfile o
K02 Central Smart Charging o
K03 Local Smart Charging o
K04 Internal Load Balancing o
K05 Remote Start Transaction with Charging
Profile
o
K06 Offline Behavior Smart Charging During
Transaction
o
K07 Offline Behavior Smart Charging at Start of
Transaction
o
K08 Get Composite Schedule o
K09 Get Charging Profiles o
K10 Clear Charging Profile o
K11 Set / Update External Charging Limit With
Ongoing Transaction
o
K12 Set / Update External Charging Limit Without
Ongoing Transaction
o
K13 Reset / release external charging limit o
K14 External Charging Limit with Local Controller o
K15 Charging with load leveling based on High
Level Communication
o
K16 Optimized charging with scheduling to the
CSMS
o
K17 Renegotiating a Charging Schedule o
K18 ISO 15118-20 Scheduled Control Mode o
K19 ISO 15118-20 Dynamic Control Mode o
K20 Adjusting charging schedule when energy
needs change
o
K21 Requesting priority charging remotely o
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 12/15 Part 0 - Introduction

Clause Functional Block UC ID Use case name OCPP 1.6 New in
OCPP
2.0.1
New in
OCPP 2.1
K22 Requesting priority charging locally o
K23 Smart Charging with EMS connected to
Charging Stations
o
K24 Smart Charging with EMS connected to Local
Controller
o
K25 Smart Charging with EMS acting as a Local
Controller
o
K26 Smart Charging with Hybrid Local & Cloud
EMS
o
K27 Smart Charging with EMS and
LocalGeneration
o
K28 Dynamic charging profiles o
L Firmware Management L01 Secure Firmware Update o
L02 Non-Secure Firmware Update o
L03 Publish Firmware file on Local Controller o
L04 Unpublish Firmware file on Local Controller o
M Certificate Management M01 Certificate Installation EV o
M02 Certificate Update EV o
M03 Retrieve list of available certificates from a
Charging Station
o
M04 Delete a specific certificate from a Charging
Station
o
M05 Install CA certificate in a Charging Station o
M06 Get Charging Station Certificate status o
N Diagnostics N01 Retrieve Log Information o
N02 Get Monitoring report o
N03 Set Monitoring Base o
N04 Set Variable Monitoring o
N05 Set Monitoring Level o
N06 Clear / Remove Monitoring o
N07 Alert Event o
N08 Periodic Event o
N09 Get Customer Information o
N10 Clear Customer Information o
N11 Set Frequent Periodic Variable Monitoring o
N12 Get Periodic Event Streams o
N13 Close Periodic Event Streams o
N14 Adjust Periodic Event Streams o
N15 Periodic Event Streams o
O Display Message O01 Set DisplayMessage o
OO2 Set DisplayMessage for Transaction o
O03 Get All DisplayMessages o
O04 Get Specific DisplayMessages o
O05 Clear a DisplayMessage o
O06 Replace DisplayMessage o
P DataTransfer P01 Data Transfer to the Charging Station o
P02 Data Transfer to the CSMS o
Q Bidirectional Power
Transfer
Q01 V2X Authorization o
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 13/15 Part 0 - Introduction

Clause Functional Block UC ID Use case name OCPP 1.6 New in
OCPP
2.0.1
New in
OCPP 2.1
Q02 Charging only (V2X control) before starting
V2X
o
Q03 Central V2X control with charging schedule o
Q04 Central V2X control with dynamic CSMS
setpoint
o
Q05 External V2X setpoint control with a charging
profile from CSMS
o
Q06 External V2X control with a charging profile
from an External System
o
Q07 Central V2X control for frequency support o
Q08 Local V2X control for frequency support o
Q09 Local V2X control for load balancing o
Q10 Idle, minimizing energy consumption o
Q11 Going offline during V2X operation o
Q12 Resuming a V2X operation after an offline
period
o
R DER Control R01 Starting a V2X session with DER control in
EVSE
o
R02 Starting a V2X session with DER control in EV o
R03 Starting a V2X session with hybrid DER
control in both EV and EVSE
o
R04 Configure DER control settings at Charging
Station
o
R05 Charging station reporting a DER event o
S Battery Swapping S01 Battery swap local authorization o
S02 Battery swap remote start o
S03 Battery swap in/out o
S04 Battery swap charging o
NOTE
OCPP is used in many different regions and for many different charging solutions. Not all functionalities offered
by OCPP will be applicable to all implementations. Implementers can decide what specific functionalities apply to
their charging solution.
For interoperability purposes, the Open Charge Alliance introduces Certification Profiles in Part 5 of the
specification.
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 14/15 Part 0 - Introduction


### Chapter 4. Basic implementation of OCPP 2.1

*_Source: Page 17 - 17_*

Chapter 4. Basic implementation of OCPP 2.1
This section is informative.
The OCPP protocol describes a large number of use cases and messages, which are not all needed to implement a basic Charging
Station or CSMS. The table below lists messages that are typically implemented to deliver basic functionality for an OCPP managed
Charging Station. The purpose of this list is to guide developers that are new to OCPP.
The basic implementation set for OCPP 2.1 is the same as for OCPP 2.0.1.
NOTE this table does not define what needs to be done to become OCPP 2.1 "certified". The functionality that is to be
implemented to become OCPP 2.1 certified is described in Part 5 of the specification, "Certification Profiles".
Table 4. OCPP 2.1 Basic Implementation
Functionality Use cases Messages
Booting a Charging Station B01-B04 BootNotification
Configuring a Charging Station B05-B07 SetVariables, GetVariables and GetReportBase (respond
correctly to requests with reportBase = ConfigurationInventory,
FullInventory and SummaryInventory).
Resetting a Charging Station B11-B12 Reset
Authorization options One of C01, C02 and C04 Authorize
Transaction mechanism E01 (one of S1-S6), E02-E03,
E05, E06 (one of S1-S6), E07-
E08, One of E09-E10, E11-E13
TransactionEvent
Availability G01, G03-G04 Only ChangeAvailability and StatusNotification.
Monitoring Events G05, N07 A basic implementation of the NotifyEvent message to be used
to report operational state changes and problem/error
conditions of the Charging Station, e.g. for Lock Failure. Also
used for reporting built-in monitoring events.
Sending transaction related
Meter values
J02 TransactionEvent
DataTransfer P01-P02 Any OCPP implementations should at least be able to reject any
request for DataTransfer if no (special) functionality is
implemented.
NOTE Please also refer to the section on Minimum Device Model in part 1.
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 15/15 Part 0 - Introduction


---

## Technical Analysis & Implementation Guide

> **Note**: This section provides technical analysis and implementation guidance for OCPP 2.1, supplementing the official specification.

### Design Intent & Architecture

#### Why OCPP 2.1?

OCPP 2.1 represents a significant evolution from OCPP 2.0.1, addressing emerging requirements in the EV charging ecosystem:

**Key Design Goals**:
1. **ISO 15118-20 Support**: Full support for bidirectional charging (V2X) and advanced EV communication
2. **Smart Grid Integration**: Enhanced control for Distributed Energy Resources (DER)
3. **Payment Flexibility**: Ad hoc payment scenarios with integrated terminals and QR codes
4. **Local Intelligence**: Cost calculation and decision-making at the Charging Station edge

**Backward Compatibility**:
- OCPP 2.1 is designed to be backward compatible with OCPP 2.0.1
- Existing OCPP 2.0.1 application logic continues to work
- JSON schemas are extended with optional fields for new features
- Minor breaking changes only in specific use cases (A02/A03, N02)

#### Architecture Overview

OCPP 2.1 follows a **3-tier model** (detailed in Part 1):

```
┌─────────────────────────────────────────────────────────────┐
│                     CSMS (Central System)                    │
│            (Charging Station Management System)              │
│  - Authorization & transaction management                   │
│  - Smart charging & DER control                             │
│  - Tariff & cost management                                 │
└───────────────────────┬─────────────────────────────────────┘
                        │ OCPP over JSON/WebSocket
                        │ (Part 4: OCPP-J Specification)
┌───────────────────────┴─────────────────────────────────────┐
│                   Charging Station                           │
│  - Edge intelligence & local decision making                │
│  - Transaction & authorization handling                     │
│  - Smart charging & DER control implementation              │
└───────────────────────┬─────────────────────────────────────┘
                        │ ISO 15118 / IEC 61851
┌───────────────────────┴─────────────────────────────────────┐
│                       EVSE + EV                              │
│  - Electric Vehicle Supply Equipment                        │
│  - Electric Vehicle with bidirectional capability           │
└─────────────────────────────────────────────────────────────┘
```

### Key Concepts for Implementers

#### 1. Functional Block Architecture

OCPP 2.1 is organized into **19 Functional Blocks** (A-S), each containing related use cases:

**Critical Blocks for All Implementations**:
- **A. Security**: Required for all production deployments
- **B. Provisioning**: Boot process and configuration
- **C. Authorization**: How users are identified
- **E. Transactions**: Core charging session management

**Important but Optional**:
- **K. Smart Charging**: Load management and grid integration
- **I. Tariff and Cost**: Pricing information display
- **G. Availability**: Status notifications

**Advanced Features**:
- **Q. Bidirectional Power Transfer**: V2G/V2H/V2L scenarios
- **R. DER Control**: Grid services and frequency support
- **S. Battery Swapping**: Battery swap station management

#### 2. Message Flow Patterns

OCPP 2.1 uses several communication patterns:

**Request/Response (Call)**
```
CSMS → Charging Station: Request
Charging Station → CSMS: Response (+ optional Confirmed/Rejected)
```

**Notifications**
```
Charging Station → CSMS: NotifyEvent (status changes)
Charging Station → CSMS: TransactionEvent (transaction lifecycle)
```

**Unconfirmed Messages (New in 2.1)**
```
Charging Station → CSMS: SEND type message (e.g., event streams)
No response expected - fire-and-forget for high-frequency updates
```

#### 3. Device Model vs. Information Model

OCPP 2.1 distinguishes between:

- **Device Model**: Physical components and variables in the Charging Station
  - Components: Controller, EVSE, Connector, etc.
  - Variables: Configurable settings and monitored values
  - Hierarchy reflects physical structure

- **Information Model**: Logical data exchanged via OCPP messages
  - Message payloads and data structures
  - Business logic and use case implementation

### Rust Implementation Considerations

#### Recommended Data Structures

```rust
/// OCPP 2.1 Message (all messages share this structure)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcppMessage {
    #[serde(rename = "messageId")]
    pub message_id: String, // UUID v4
    pub action: Option<String>, // For Call messages
    pub payload: Option<Value>, // Message-specific payload
    pub message_type: MessageType, // Call, CallResult, CallError, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MessageType {
    Call,
    CallResult,
    CallError,
    Send, // New in OCPP 2.1 - unconfirmed messages
}

/// Charging Station component hierarchy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub name: String,
    pub instance: Option<String>,
    pub EVSE: Option<Box<EVSE>>, // Recursive nesting
}

/// Variable reference for GetVariables/SetVariables
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Variable {
    pub name: String,
    pub instance: Option<String>,
    pub component: Component,
}

/// Transaction lifecycle event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionEvent {
    pub event_type: TransactionEventType,
    pub timestamp: DateTime<Utc>,
    pub transaction_info: Transaction,
    pub seq_no: u32, // Sequence number for ordering
    pub trigger_reason: EventTriggerReason,
}
```

#### Error Handling Strategy

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OcppError {
    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Property constraint violation: {field} = {value}")]
    PropertyConstraint {
        field: String,
        value: String,
    },

    #[error("Unknown action: {0}")]
    UnknownAction(String),

    #[error("Security profile not supported: {0}")]
    SecurityProfileNotSupported(u8),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

// OCPP-defined error codes for CallError responses
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum OcppErrorCode {
    NotImplemented,
    NotSupported,
    InternalError,
    MessageFormatError,
    MessageTypeNotSupported,
    RpcError,
}
```

#### WebSocket Implementation (tokio-tungstenite)

```rust
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{StreamExt, SinkExt};

pub async fn connect_to_csms(url: &str) -> Result<(), OcppError> {
    let (ws_stream, _) = connect_async(url).await?;
    let (mut write, mut read) = ws_stream.split();

    // Send BootNotification
    let boot_msg = OcppMessage::boot_notification(/* ... */);
    let boot_json = serde_json::to_string(&boot_msg)?;
    write.send(Message::Text(boot_json)).await?;

    // Message handling loop
    while let Some(message) = read.next().await {
        match message? {
            Message::Text(text) => {
                let ocpp_msg: OcppMessage = serde_json::from_str(&text)?;
                handle_ocpp_message(ocpp_msg).await?;
            }
            Message::Close(_) => break,
            _ => {}
        }
    }

    Ok(())
}
```

### Best Practices

#### ✅ Recommended Approaches

1. **Start with Basic Implementation**
   - Implement functional blocks B, C, E, G first
   - Add other blocks incrementally
   - Refer to Part 5 for certification requirements

2. **Use Message IDs Correctly**
   - Generate UUID v4 for each `messageId`
   - Track pending requests by `messageId`
   - Match responses to requests using `messageId`

3. **Handle Offline Operation**
   - Cache authorizations locally (Functional Block C)
   - Support offline transactions (E04, E08)
   - Sync with CSMS when connection restored

4. **Implement Smart Charging Early**
   - Even basic implementations benefit from load limiting
   - Use `SetChargingProfile` for central control
   - Support at least `ChargingProfilePurposeType::ChargingStationMaxProfile`

5. **Security First**
   - Start with Security Profile 2 (TLS + certificate authentication)
   - Support Security Profile 3 for production (full certificate chain)
   - Implement `SecurityEventNotification` (A04)

#### ❌ Common Pitfalls to Avoid

1. **Ignoring Version Compatibility**
   - Don't assume OCPP 2.0.1 implementations work without testing
   - Check for breaking changes in use cases A02/A03 and N02
   - Validate JSON schemas match the protocol version

2. **Poor Error Handling**
   - Don't use `GenericError` for all failures
   - Return specific OCPP error codes
   - Include error descriptions in `CallError` responses

3. **Missing Transaction State Management**
   - Track transaction state transitions carefully
   - Handle all `TransactionEventType` values
   - Support transaction resumption after reboot (E17)

4. **Ignoring Component-Variable Hierarchy**
   - Don't hardcode component paths
   - Use the Device Model from Part 1
   - Support `GetVariables` / `SetVariables` for configuration

5. **Skipping Heartbeats**
   - Don't rely only on application-level messages
   - Implement `HeartbeatRequest` (G02) for connection monitoring
   - Configure reasonable interval (default: 60 seconds)

### Integration with Existing Systems

#### CSMS Integration Points

When integrating OCPP 2.1 with existing CSMS:

1. **Authorization Backend**
   - Implement `AuthorizeRequest` handler
   - Support multiple `IdToken` types (RFID, ISO 15118, PIN)
   - Handle authorization cache (C10)

2. **Tariff Management**
   - Implement `SetDisplayMessage` for tariff display (I07-I11)
   - Support cost calculation during transactions (I12)
   - Handle multiple tariff structures

3. **Smart Charging Backend**
   - Integrate with load management systems
   - Implement `SetChargingProfile` and `ClearChargingProfile`
   - Support dynamic profiles (K28, new in 2.1)

#### Charging Station Hardware Integration

When implementing OCPP 2.1 on Charging Station hardware:

1. **Hardware Abstraction Layer**
   - Abstract hardware-specific components (EVSE, connectors)
   - Map hardware variables to OCPP `Variable` type
   - Implement monitoring for all relevant `Variable` types

2. **Local Controller Support**
   - Support local decision-making for smart charging
   - Implement `GetCompositeSchedule` (K08) for visibility
   - Handle external control signals

3. **Payment Terminal Integration**
   - Support integrated payment terminals (C18-C23)
   - Handle payment authorization flows
   - Implement transaction cancellation (C19)

### Example Scenarios

#### Scenario 1: Basic Charging Session

```
1. Charging Station boots → BootNotification (B01)
2. CSMS accepts → BootNotificationResponse
3. Driver plugs in → StatusNotification (G01)
4. Driver presents RFID → AuthorizeRequest (C01)
5. CSMS validates → AuthorizeResponse (Accepted)
6. Transaction starts → TransactionEvent (Started, E03)
7. Meter values → TransactionEvent (Updated, J02)
8. Driver unplugs → TransactionEvent (Ended, E07)
9. CSMS stops transaction → TransactionEvent (Ended confirmation)
```

#### Scenario 2: Smart Charging with Load Limiting

```
1. CSMS detects grid constraint
2. CSMS → SetChargingProfile (K01)
   - Profile: ChargingStationMaxProfile
   - Limit: 22kW (reduced from 43kW)
3. Charging Station applies limit
4. Transaction continues at reduced power
5. Grid constraint clears
6. CSMS → ClearChargingProfile (K10)
7. Charging Station returns to normal power
```

#### Scenario 3: ISO 15118-20 Bidirectional Charging (New in 2.1)

```
1. EV connects with ISO 15118-20
2. EV negotiates V2X session → ISO 15118-20 messages
3. Charging Station → AuthorizeRequest (C07/C08)
   - IdToken: ISO 15118 certificate
4. Transaction starts → TransactionEvent (Started, Q01)
5. CSMS → SetChargingProfile (K18/K19)
   - Purpose: V2X charging schedule
   - Includes power limits for charging/discharging
6. EV discharges to grid → meter values negative
7. Transaction ends → TransactionEvent (Ended)
```

### References to Other Parts

- **Part 1**: Architecture, Device Model, topologies
- **Part 2**: Detailed functional block specifications
- **Part 4**: OCPP-J (JSON/WebSocket) protocol implementation
- **Part 5**: Certification profiles and requirements
- **Part 6**: Test cases for verification

### Next Steps for Implementers

1. **Read Part 1** - Understand the architecture and Device Model
2. **Review Part 5** - Identify required certification profile
3. **Choose Functional Blocks** - Select blocks relevant to your use case
4. **Implement Messages** - Start with basic implementation messages
5. **Test with Part 6** - Use test cases to verify compliance
6. **Consider Advanced Features** - Plan for V2X, DER control, etc.

---

**Analysis completed**: 2025-01-25
**Ralph Loop Iteration**: 1
**Phase**: Technical Analysis - Part 0 Foundation