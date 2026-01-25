# Part 1: Architecture & Topology
> **Source**: `OCPP-2.1_edition2_part1_architecture_topology.pdf`
> **Total Pages**: 24

---

## Table of Contents

- OCPP 2.1: Part 1 - Architecture & Topology (Page 1)
- Table of Contents (Page 2)
- Disclaimer (Page 3)
- Version History (Page 4)
- Chapter 1. Introduction (Page 5)
- Chapter 2. 3-tier model (Page 6)
- Chapter 3. Device Model: Addressing Components and Variables (Page 7)
- Chapter 4. Device Model hierarchy (Page 13)
- Chapter 5. Information Model vs. Device Model (Page 14)
- Chapter 6. Using OCPP for other purposes than EV charging (Page 15)
- Chapter 7. Numbering (Page 16)
- Chapter 8. Topologies supported by OCPP (Page 17)
- Chapter 9. Energy management topologies supported by OCPP (Page 20)

---

## Content


### OCPP 2.1: Part 1 - Architecture & Topology

*_Source: Page 1 - 1_*

OCPP 2.1
Part 1 - Architecture & Topology
Edition 2, 2025-12-03


### Table of Contents

*_Source: Page 2 - 2_*

Table of Contents
Disclaimer . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  1
Version History . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  2
1. Introduction. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  3
1.1. Goal of this document . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  3
1.2. Terms and abbreviations. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  3
2. 3-tier model . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  4
3. Device Model: Addressing Components and Variables. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  5
3.1. Components . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  5
3.2. Variables . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  6
3.3. Characteristics and Attributes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  6
3.4. Monitoring . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  8
3.5. Standardized lists of Components and Variables . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  9
3.6. Minimum Device Model . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  9
4. Device Model hierarchy . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  11
5. Information Model vs. Device Model . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  12
6. Using OCPP for other purposes than EV charging. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  13
7. Numbering . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  14
7.1. EVSE numbering . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  14
7.2. Connector numbering . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  14
7.3. Transaction IDs. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  14
8. Topologies supported by OCPP . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  15
8.1. Charging Station(s) directly connected to CSMS . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  15
8.2. Multiple Charging Stations connected to CSMS via Local Proxy . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  15
8.3. Multiple Charging Stations connected to CSMS via Local Controller . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  16
8.4. Non-OCPP Charging Stations connected to CSMS via OCPP Local Controller. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  16
8.5. DSO control signals to CSMS . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  16
9. Energy management topologies supported by OCPP . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  18
9.1. Parallel control of charging station by CSMS and smart meter . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  18
9.2. Parallel control of charging location by CSMS and EMS . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  18
9.3. EMS via Local Controller . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  19
9.4. EMS as man-in-the-middle . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  20
9.5. Hybrid local & cloud EMS . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  20
9.6. Parallel control by CSMS and EMS. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  21


### Disclaimer

*_Source: Page 3 - 3_*

Disclaimer
Copyright © 2010 – 2025 Open Charge Alliance. All rights reserved.
This document is made available under the *Creative Commons Attribution-NoDerivatives 4.0 International Public License*
(https://creativecommons.org/licenses/by-nd/4.0/legalcode).
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 1/22 Part 1 - Architecture & Topology


### Version History

*_Source: Page 4 - 4_*

Version History
Version Date Description
2.1 Edition 2 2025-12-03 OCPP 2.1 Edition 2. All errata from OCPP 2.1 Part 1 until and including Errata 2025-
11 have been merged into this version of the specification.
2.1 Edition 1 2025-01-23 OCPP 2.1 Edition 1
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 2/22 Part 1 - Architecture & Topology


### Chapter 1. Introduction

*_Source: Page 5 - 5_*

Chapter 1. Introduction
1.1. Goal of this document
The goal of this document is to describe a number of architecture related topics for OCPP 2.1 . It is not fundamentally different
from the version for OCPP 2.0.1.
OCPP was originally intended for two way communication between a backoffice, in OCPP the Charging Station Management System
(in this document: CSMS) and a Charging Station. The protocol has become more advanced and with every new revision new
functionalities and options are added. It has evolved into a protocol that can be used in different architectures for different types of
Charging Stations.
This document describes, in addition to the original "simple" setup CSMS <> Charging Station, a number of topologies as an
additional explanation for using OCPP. Furthermore, the Device Management concept to configure and monitor any type of
Charging Station, the OCPP Information Model and the 3-tier model are explained.
This document is partially informative and partially normative and is not intended to limit the use of OCPP. However, it does add an
explanation what kind of use of OCPP the creators of OCPP had in mind when creating this version of the specification. This
document is therefore also intended to support the reader of the protocol specification in Part 2 of OCPP to understand how it can
be used.
1.2. Terms and abbreviations
This section contains the terminology and abbreviations that are used throughout this document.
1.2.1. Terms
Term Meaning
Charging Location A group of one or more Charging Stations that belong together geographically or spatially.
Charging Station The Charging Station is the physical system where EVs can be charged. A Charging Station
has one or more EVSEs.
Connector The term Connector, as used in this specification, refers to an independently operated and
managed electrical outlet on a Charging Station. In other words, this corresponds to a single
physical Connector. In some cases an EVSE may have multiple physical socket types and/or
tethered cable/Connector arrangements(i.e. Connectors) to facilitate different vehicle types
(e.g. four-wheeled EVs and electric scooters).
EVSE An EVSE is considered as an independently operated and managed part of the Charging
Station that can deliver energy to one EV at a time.
Local port Smart Meter The local port on a Smart Meter is a port (for example serial) on a digital electricity meter
that provides access to information about meter readings and usage.
1.2.2. Abbreviations
Abbreviation Meaning
DSO Distribution System Operator
CSO Charging Station Operator
CSMS Charging Station Management System
EMS Energy Management System. In this document this is defined as a device that manages the local loads
(consumption an production) based on local and/or contractual constraints and/or contractual incentives. It
has additional inputs, such as sensors and controls from e.g. PV, battery storage.
EVSE Electric Vehicle Supply Equipment
LC Local Controller. In this document this is defined as a device that can send messages to its Charging
Stations, independently of the CSMS. A typical usage for this is the local smart charging case described in
the Smart Charging chapter of Part 2 of OCPP, where a Local Controller can impose charge limits on its
Charging Stations.
LP Local Proxy. Acts as a message router.
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 3/22 Part 1 - Architecture & Topology


### Chapter 2. 3-tier model

*_Source: Page 6 - 6_*

Chapter 2. 3-tier model
This section is informative.
To understand the terminology in the OCPP specification, it is important to understand the starting point of this specification. The
OCPP specification uses the term Charging Station as the physical system where EVs can be charged. A Charging Station can have
one or more EVSEs (Electric Vehicle Supply Equipment). An EVSE is considered as a part of the Charging Station that can deliver
energy to one EV at a time. The term Connector, as used in this specification, refers to an independently operated and managed
electrical outlet on a Charging Station, in other words, this corresponds to a single physical Connector. In some cases an EVSE may
have multiple physical socket types and/or tethered cable/connector arrangements to facilitate different vehicle types (e.g. four-
wheeled EVs and electric scooters). This setup is referred to as the 3-tier model and visualized in the figure below.
Figure 1. 3-tier model as used in OCPP
A Charging Location is a group of Charging Stations at the same place or building. This concept has no meaning in OCPP, since
OCPP is about CSMS to Charging Station communication, but a Charging Location may exist as a concept in a CSMS for
management and reporting purposes.
NOTE
This section describes the charging infrastructure on a logical level for communication purposes. We do not wish
to impose a mapping onto physical hardware. This is a manufacturer’s choice. For example, the EVSE might be
integrated into a Charging Station and to look as just a part of that device, but it might just as well have its own
casing and live outside of the physical entity Charging Station, for example a charging plaza with 20 EVSEs and
Connectors which communicates via 1 modem as 1 Charging Station to the CSMS is seen by OCPP as 1 Charging
Station.
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 4/22 Part 1 - Architecture & Topology


### Chapter 3. Device Model: Addressing Components and Variables

*_Source: Page 7 - 12_*

Chapter 3. Device Model: Addressing Components and Variables
The Device Model refers to a generalized mechanism within OCPP to enable any model of Charging Station to report how it is build
up, so it can be managed from any CSMS. To manage a Charging Station with the Device Model (i.e. "to manage a device") a
number of messages and use cases is defined to configure and monitor a Charging Station in detail, without defining the structure
of the Charging Station in advance. To be able do do this, OCPP provides a generalized mechanism to allow the exchange of a wide
range of information about Charging Station. This version of the Device Model has the 3-tier model (Charging Station, EVSE,
Connector) as its starting point, which means that any description created with the Device Model follows these three tiers. The
remainder of this chapter describes how the data (and associated meta-data) looks like that can be exchanged between a Charging
Station and a CSMS. The use cases and messages that are used to manage a device are not described here, but in Part 2 of the
specification. This chapter only focuses on the data model.
3.1. Components
In OCPP 2.1 , a Charging Station is modelled as a set of "Components", typically representing physical devices (including any
external equipment to which it is connected for data gathering and/or control), logical functionality, or logical data entities.
Components of different types are primarily identified by a ComponentName, that is either the name of a standardized component
(see OCPP part 2c), or a custom/non-standardized component name, for new, pre-standardized equipment, vendor specific
extensions, etc.
ChargingStation (TopLevel), EVSE, and Connector represent the three major "tiers" of a Charging Station, and constitute an implicit
"location-based" addressing scheme that is widely used in many OCPP data structures. Each "tier" has a component of the same
name, which represents the tier.
For example, EVSE 1 on a Charging Station is represented by the component named "EVSE" (no instance name) with "evseId = 1". In
the same manner, Connector 1 on EVSE 1 is represented by the component named "Connector" (no instance name) with "evseId = 1,
connectorId = 1".
By default, all components are located at the ChargingStation tier, but individual instances of any component can be associated with
a specific EVSE, or a specific Connector (on a specific EVSE) by including EVSE or EVSE and Connector identification numbers as
part of a component addressing reference.
Additionally, there can be more than one instance of a component (in the functional dimension), representing multi-occurrence
physical or logical components (e.g. power converter modules, fan banks, resident firmware images, etc.).
Each distinct component instance is uniquely identified by an (optional) componentInstance addressing key. It is allowed for a
component to exist without an instance and at the same time also exist with one of more instances. When no componentInstance is
provided, then the component without an instance is referenced.
Components do not in themselves hold data: all externally accessible data associated with each component instance is
represented by a set of variables that can be read, set, and/or monitored for changes. The relationship of a Component with one or
more Variables is illustrated in below.
ComponentType
Name
Instance [0..1]
VariableType
Name
Instance [0..1]
EVSEType
Id
ConnectorId [0..1]
1
0..1
1
*
Figure 2. Component and variables
The table below illustrates some common components (by their standardized component-names), and examples of the hierarchical
location levels at which they typically occur for a basic home charger and a typical public Charging Station.
Basic home charger example configuration
ChargingStation tier EVSE tier Connector tier
ChargingStation (itself, as a whole) EVSE (itself, as a whole) Connector (itself, as a whole)
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 5/22 Part 1 - Architecture & Topology

Basic home charger example configuration
RadioLink ControlMetering PlugRetentionLock
TokenReader OverCurrentBreaker
Controller RCD
ChargingStatusIndicator
Public Charging Station example configuration
ChargingStation tier EVSE tier Connector tier
ChargingStation (itself, as a whole) EVSE (itself, as a whole) Connector (itself, as a whole)
ElectricalFeed ElectricalFeed AccessProtection
TokenReader TokenReader PlugRetentionLock
Display Display
FiscalMetering FiscalMetering
Clock ControlMetering
Controller OverCurrentBreaker
RCD
ChargingStatusIndicator
3.2. Variables
Every component has a number of variables, that can, as appropriate, be used to hold, set, read, and/or report on all (externally
visible) data applicable to that component, including configuration parameters, measured values (e.g. a current or a temperature)
and/or monitored changes to variable values.
Although many components can have associated variables that are, by their nature, specific to the component type (e.g.
ConnectorType for a Connector component), there is a minimal set of standardized variables that is used to provide standardized
high level event notification and state/status reporting (e.g. Problem, Active) on a global and/or selective basis, and also to report
component presence, availability, etc. during the inventorying/discovery process (e.g. Available, Enabled).
A Charging Station is not required to report the base variables: Present, Available and Enabled when they are readonly and set to
true. When a Charging Station does not report: Present, Available and/or Enabled the Central System SHALL assume them to be
readonly and set to true
Variables can be any of a range of common general-purpose data types (boolean, integer, decimal, date-time, string), but also can
have their allowable values constrained to particular ranges, enumeration lists, sets, or ordered lists.
To support complex components, there can be more than one instance of any given variable name associated with any component
(e.g. power converter modules reporting temperature, current, or voltage at multiple points).
Each distinct variable instance is uniquely identified by an (optional) variableInstance addressing key string value. It is allowed for a
variable to exist without an instance and at same time also with one or more instances. When no variableInstance is provided, then
the variable without an instance is referenced.
3.3. Characteristics and Attributes
Each variable, in addition to its primary ("Actual") value, can have a set of associated secondary data that is linked to the same
primary variable name and variableInstance.
This greatly avoids cluttering the variables namespace with confusing clusters of ancillary variable names (e.g. FanSpeed,
FanSpeedUnits, MinimumFanSpeed, BaseFanSpeed) that lack consistence and discoverability.
The ancillary variable data includes:
• Variable characteristics meta-data (read-only)
◦ Unit of measure (V,W,kW,kWh, etc.)
◦ Data type (Integer, Decimal, String, Date, OptionList, etc.)
◦ Lower limit
◦ Upper limit
◦ List of allowed values for enumerated variables
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 6/22 Part 1 - Architecture & Topology

• Variable attributes (read-write):
◦ Actual value
◦ Target value
◦ Configured lower limit
◦ Configured upper limit
◦ Mutability (whether the value can be altered or not, e.g. ReadOnly or ReadWrite)
◦ Persistence (whether the value is preserved in case of a reboot or power loss)
The relationship of a Variable with one or more VariableAttributes is illustrated in the figure below.
ComponentType
Name
Instance [0..1]
VariableType
Name
Instance [0..1]
EVSEType
Id
ConnectorId [0..1]
VariableAttributeType
Type [0..1]
Value
Mutability [0..1]
Persistent
Constant
VariableCharacteristicsType
Unit [0..1]
DataType
MinLimit [0..1]
MaxLimit [0..1]
ValuesList [0..1]
SupportsMonitoring
1
0..1
1
*
1
1..*
1
0..1
Figure 3. Variable attributes and characteristics
There is a difference between how to implement (physical) devices and (virtual) controller components, using the DeviceModel. A
(virtual) controller component has to be implemented as described in part 2 chapter the "Referenced Components and Variables".
These kind of components/variables are only using the variableAttribute type 'Actual'. Depending on if this variableAttribute is
writable, the CSMS can use this to set a new value.
(Physical) devices are a bit more complex to implement. For example, there is a fan with a fan speed, that has a (physical) limit with
a range of 0 - 1000. But it should not be allowed to set the value below 200, because the fan can stop functioning. And it should not
be set above 500, because that would be bad for the fan on the long run. When implementing this device using the DeviceModel, it
can be defined as follows:
Component name Fan
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 7/22 Part 1 - Architecture & Topology

Variable name FanSpeed
variableAttribute 1 type Actual
value <The current fan speed value of the fan.>
mutability ReadOnly
variableAttribute 2 type Target
value <The CSMS can use this value to adjust the fan speed. The Charging Station
SHALL try to keep the actual value at the target value.>
mutability ReadWrite
variableAttribute 3 type MaxSet
value <The value '500' from the example. The target may not be set above this
value.>
variableAttribute 4 type MinSet
value <The value '200' from the example. The target may not be set below this
value.>
variableCharacteristics maxLimit <The value '1000' from the example. This could be the physical max limit of
the fan.>
minLimit <The value '0' from the example. This could be the physical min limit of the
fan. This could also be -1000, if the fan is also able to rotate in the other
direction.>
Description This is an example of how a fan could be defined using the DeviceModel.
When trying to set the target with value 600, the Charging Station will first check the allowed min and max values/limits and reject
the set. If the target value is set to 500, the value is within range and the Charging Station will allow the set and start to adjust the
actual fan speed. If the actual fan speed is measured to be 502, it’s out of range. But it should be reported to the CSMS, so the
actual value of a physical component should be updated without checking the min and max values/limits.
3.4. Monitoring
(Updated in OCPP 2.1)
Optional monitoring settings can be associated with a variable, that allow changes to variable (Actual) values are to be reported to
the CSMS as event notifications.
These include:
• Monitoring value
• Monitoring type: upper threshold, lower threshold, delta, periodic
• Severity level when reporting the event
The following table show which MonitorType/dataType combinations are possible.
string decimal integer dateTime boolean OptionList SequenceList MemberList
UpperThresh
old
X X
LowerThresh
old
X X
Delta X X X X X X X X
Periodic X X X X X X X
PeriodicCloc
kAligned
X X X X X X X
TargetDelta X X X X X X X
TargetDeltaR
elative
X X X X X X X
• For UpperThreshold and LowerThreshold the value represents the to be exceeded value by the actual value of the variable.
• For Delta this value represents the change in value compared to the actual value from the moment the monitor was set.
◦ When the dataType of the variable is integer or decimal, this value represents the absolute difference to be reached
to trigger the monitor.
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 8/22 Part 1 - Architecture & Topology

◦ When the dataType of the variable is dateTime the unit of measure will be in seconds.
◦ When the dataType of the variable is string, boolean, OptionList, SequenceList or MemberList, this value is ignored.
The monitor will be triggered by every change in the actual value.
• When a delta monitor is triggered OR when the Charging Station has rebooted, the Charging Station shall set a new
momentary value.
• For Periodic and PeriodicClockAligned the value represents the interval in seconds.
• For TargetDelta this value represents the absolute difference between the variableAttributes "Actual" and "Target"
(calculated as Actual - Target).
• For TargetDeltaRelative this value represents the relative deviation of the "Actual" variableAttribute with respect to the
"Target" variableAttribute (calculated as the absolute value of (Actual - Target) / Target).
The relationship between a Variable and one or more VariableMonitoring elements is illustrated in the figure below.
ComponentType
Name
Instance [0..1]
VariableType
Name
Instance [0..1]
EVSEType
Id
ConnectorId [0..1]
VariableAttributeType
Type [0..1]
Value
Mutability [0..1]
Persistent
Constant
VariableCharacteristicsType
Unit [0..1]
DataType
MinLimit [0..1]
MaxLimit [0..1]
ValuesList [0..1]
SupportsMonitoring
VariableMonitoringType
Id
Severity
Transaction
Type
Value
1
0..1
1
*
1
1..*
1
0..1
1
0..*
Figure 4. Variables and monitoring
3.5. Standardized lists of Components and Variables
To provide some level of interoperability between different Charging Stations and CSMSs, besides the above defined model of
Components and Variables, part 2 - appendices of the OCPP specification provides a list of standardized names for Components
and Variables. The idea of this lists is to make sure that if a Charging Station and CSMS want to exchange information about a
component, they both use the same name and description if it is listed in the OCPP specification. For names of a Components or
Variables that are not listed in the specification, bilateral appointments between Charging Station manufacturer and CSMS are to be
made. In these cases it is advised to provide feedback to the Open Charge Alliance to be able to include new/additional
Components and Variables in new versions of OCPP.
3.6. Minimum Device Model
Since the Device Model is a generalized mechanism which can be applied to any model of Charging Station, the complexity of
different implementations can vary. It consists of a number of use cases and messages that are not all required. This section
describes the minimum part of the Device Model that needs to be implemented to create a working implementation of OCPP 2.1.
The Device Model introduces Components and Variables that can be used for configuring and monitoring a Charging Station. A
number of these Components and Variables are included in the list of Referenced Components and Variables (grouped by
Functional Block) in Part 2 of the specification. When implementing a Functional Block, ALL required Configuration Variables that
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 9/22 Part 1 - Architecture & Topology

belong to a Functional Block SHALL be implemented. The required Configuration Variables from the General section SHALL also be
implemented for all implementations of OCPP 2.1.
The following table describes which messages are required to implement for use cases that are part of the Device Model
implementation.
Use cases / messages that are part of a minimium Device Model implementation
Use case Messages
B05 Set Variables SetVariables message MUST be implemented
B06 Get Variables GetVariables message MUST be implemented.
B07 Get Base Report GetBaseReport message MUST be implemented and MUST support ConfigurationInventory
and FullInventory. The content of these reports depends on the implementation of the
Charging Station. It is up to the implementer to decide which components and variables exist
in the implementation.
Additional use cases / messages that are not part of a minimium Device Model implementation
Use case Messages
B08 Get Custom Report GetCustomReport message is optional.
N02 Get Monitoring Report GetMonitoringReportRequest message is optional.
N03 Set Monitoring Base SetMonitoringBaseRequest message is optional.
N04 Set Variable Monitoring SetVariableMonitoringRequest message is optional.
N05 Set Monitoring Level SetMonitoringLevelRequest message is optional.
N06 Clear/Remove Monitoring ClearVariableMonitoringRequest message is optional.
N07 Alert Event it is RECOMMENDED that NotifyEventRequest is implemented in the Charging Station even
when monitoring is not implemented, so that this can be used to report built-in monitoring
events.
N08 Periodic Event see N07.
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 10/22 Part 1 - Architecture & Topology


### Chapter 4. Device Model hierarchy

*_Source: Page 13 - 13_*

Chapter 4. Device Model hierarchy
(New in OCPP 2.1)
The 3-tier model of the Device Model does not suffice to represent the hierarchy of Charging Stations with a lot of components. If
there is a need to represent a hierarchy between components, then a set of standard variables can be used for this. To allow
comprehensive rendering of its components in a UI, a Charging Station may describe the hierarchy of its components using the
following read-only variables:
• CommunicationParent (data flow source),
• ElectricalParent (power flow source),
• LogicalParent (for a comprehensive overview),
• PhysicalParent (container).
These variables point to one or more (using multiple instances of these variables) parent components. Since the Device Model
does not permit duplicate component names and instances, which might occur in a hierarchy, the optional read-only variable
"Label" permits specifying a non-unique label to use instead of the component name and instance in a hierarchical rendering.
See Part 2 of this specification for details on these variables.
ElectricalFeed
ComponentName = "ElectricalFeed"
PowerBank#1
ComponentName: "PowerBank"ComponentInstance: "1"ElectricalParent = "ElectricalFeed"
PowerBank#2
ComponentName: "PowerBank"ComponentInstance: "2"ElectricalParent = "ElectricalFeed"
Fan#1
ComponentName: "Fan"ComponentInstance: "1"PhysicalParent: "PowerBank#1"
Fan#2
ComponentName: "Fan"ComponentInstance: "2"PhysicalParent: "PowerBank#1"
Fan#3
ComponentName: "Fan"ComponentInstance: "3"PhysicalParent: "PowerBank#2"Label: "Fan#1"
Fan#4
ComponentName: "Fan"ComponentInstance: "4"PhysicalParent: "PowerBank#2"Label: "Fan#2"
wiring wiringpart ofpart of part of part of
Figure 5. Example of hierarchy in device model
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 11/22 Part 1 - Architecture & Topology


### Chapter 5. Information Model vs. Device Model

*_Source: Page 14 - 14_*

Chapter 5. Information Model vs. Device Model
As described above, the terms Information Model and Device Model refer to different concepts. The Information Model refers to a
model of the information structure upon which the messages and datatypes in OCPP are based, whereas the Device Model refers
to a generalized mechanism within OCPP to enable any model of Charging Station to report how it is build up so, it can be managed
from any CSMS without defining the structure of the Charging Station in advance.
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 12/22 Part 1 - Architecture & Topology


### Chapter 6. Using OCPP for other purposes than EV charging

*_Source: Page 15 - 15_*

Chapter 6. Using OCPP for other purposes than EV charging
As indicated in the introduction of this document, OCPP is primarily intended for two way communication between a CSMS and a
Charging Station. However, with the addition of the Device Model as described in the chapter Device Model, OCPP can additionally
be used for other purposes. For example, the reporting of Events or Status changes in transformers or stand-alone battery packs
might also be useful for companies that are rolling out EV charging infrastructure. In this example, a BootNotification could be used
to connect these devices to a management system. In the device model a device that is not a Charging Station, can be recognized
by the fact that the component Charging Station is not present at the top level. At the moment the OCPP specification does not
provide use cases for non Charging Station devices. However, they may be added in a future version of OCPP.
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 13/22 Part 1 - Architecture & Topology


### Chapter 7. Numbering

*_Source: Page 16 - 16_*

Chapter 7. Numbering
This section is normative.
7.1. EVSE numbering
To enable the CSMS to address all the EVSEs of a Charging Station, EVSEs MUST always be numbered in the same way.
EVSEs numbering (evseIds) MUST be as follows:
• The EVSEs MUST be sequentially numbered, starting from 1 at every Charging Station (no numbers may be skipped).
• evseIds MUST never be higher than the total number of EVSEs of a Charging Station
• For operations initiated by the CSMS, evseId 0 is reserved for addressing the entire Charging Station.
• For operations initiated by the Charging Station (when reporting), evseId 0 is reserved for the Charging Station main
controller.
Example: A Charging Station with 3 EVSEs: All EVSEs MUST be numbered with the IDs: 1, 2 and 3. It is advisable to number the
EVSEs of a Charging Station in a logical way: from left to right, top to bottom incrementing.
7.2. Connector numbering
To enable the CSMS to address all the Connectors of a Charging Station, Connectors MUST always be numbered in the same way.
Connector numbering (connectorIds) MUST be as follows:
• The connectors are numbered (increasing) starting at connectorId 1 on every EVSE
• Every connector per EVSE has a unique number
• ID of the first Connector of an EVSE MUST be 1
• Additional Connectors of the same EVSE MUST be sequentially numbered (no numbers may be skipped)
• connectorIds MUST never be higher than the total number of connectors on that EVSE
Example: A Charging Station with 3 EVSEs that each have 2 connectors, is numbered as follows:
• EVSE 1 has connectors with connectorId 1 and 2
• EVSE 2 has connectors with connectorId 1 and 2
• EVSE 3 has connectors with connectorId 1 and 2
7.3. Transaction IDs
TransactionIds are now generated by the Charging Station and MUST be unique on this Charging Station for every started
transaction.
In OCPP 1.x this was done by the CSMS.
The format of the transaction ID is left to implementation. This MAY for example be an incremental number or an UUID.
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 14/22 Part 1 - Architecture & Topology


### Chapter 8. Topologies supported by OCPP

*_Source: Page 17 - 19_*

Chapter 8. Topologies supported by OCPP
This chapter shows a number of topologies for using OCPP. As indicated in the introduction, OCPP was originally used for a setup
where each Charging Station communicates directly with the CSMS. It is important to keep in mind that OCPP has no knowledge of
the topology of the Charging Station network. The following figure shows an example of a more complex topology where OCPP is
used between CSMS, Local Controller and Charging Station, and other protocols are being used between EMS (Energy Management
System) and Local Controller, and the smart grid meter and the Charging Station.
Site
Local Controller
Charging
Station 1
Charging
Station 2
EMS
e.g. PV, meter, battery
Smart Meter
CSMS
Third Parties
e.g. aggregator
<other>
<other> OCPP
OCPP OCPP
Figure 6. Example of a topology with OCPP and non-OCPP components
8.1. Charging Station(s) directly connected to CSMS
Description
This is the basic setup for using OCPP.
CSMS Charging
Station
OCPP
Figure 7. Charging Station directly connected to CSMS
8.2. Multiple Charging Stations connected to CSMS via Local Proxy
Description
In some situations it is desirable to route all communications for a group of Charging Stations through a single network node (i.e.
modem, router, etc.). A typical example is the situation where a number of a Charging Stations are located in an underground
parking garage with little or no access to the mobile network. In order to provide access to mobile data the Charging Stations are
linked to a central data communications unit over a LAN. This central unit connects to the mobile network and acts as a proxy
between CSMS and Charging Stations. Such a unit is called a "local proxy" (LP) in OCPP. A local proxy acts as a message router.
Neither the CSMS nor the Charging Stations are aware of the topology of the network. For the Charging Stations in the group the
local proxy "is" the CSMS. Similarly, for the CSMS the local proxy "is" the Charging Station. The diagram below illustrates this
configuration.
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 15/22 Part 1 - Architecture & Topology

CSMS LP
ChargingStation 1
ChargingStation n
OCPP OCPP
OCPP
Figure 8. Multiple Charging Stations connected to CSMS via Local Proxy
8.3. Multiple Charging Stations connected to CSMS via Local Controller
Description
Whereas a local proxy does little more than route OCPP messages, a Local Controller can send messages to its Charging Stations,
independently of the CSMS. A typical usage for this is the local smart charging case described in the Smart Charging chapter of
Part 2 of OCPP, where a Local Controller can impose charge limits on its Charging Stations. In order for a Local Controller to be
addressed by the CSMS, it needs to have its own Charging Station identity. From the point of view from OCPP, the Local Controller
will just be a Charging Station (without any EVSEs/Connectors). The CSMS will possess the logic to deal with the Local Controller in
order to support, for example, local smart charging. It is up to the implementation of the CSMS, whether the group topology is
manually configured or deduced from the network based on IP addresses and information in BootNotifications. The diagram below
illustrate this configuration.
CSMS LC
Charging
Station 1
Charging
Station n
OCPP
OCPP
OCPP
Figure 9. Multiple Charging Stations connected to CSMS via Local Controller
NOTE
When a Charging Station connects to the Local Controller, the Local Controller must open a websocket
connection with the same address to the CSMS. The advantage of this approach is that CSMS does not require
any modification, because it does not notice that a Local Controller is in between. Still, a Local Contoller can read
all messages to a Charging Stations, and can act on it, for example to perform local load-balancing. It will,
however, in large installations lead to a lot of websocket connections between CSMS and LC.
For further information, please refer to OCPP implementation guide in Part 4.
8.4. Non-OCPP Charging Stations connected to CSMS via OCPP Local
Controller
This setup has multiple non-OCPP Charging Stations that are abstracted away using a OCPP enabled Local Controller. When
applying OCPP in this situation, the LC should be considered as a Charging Station with many EVSEs or the LC should act as
multiple OCPP Charging Stations (having their own Charging Station Identity).
CSMS LC
ChargingStation 1
ChargingStation n
OCPP non-OCPP
non-OCPP
Figure 10. Multiple non-OCPP Charging Stations connected to CSMS via Local Controller
8.5. DSO control signals to CSMS
This is a set-up in which the CSMS is the only application sending signals to a its Charging Stations, but the CSMS receives smart
charging signals from a DSO based on (most likely) grid constraints. This means that a non-OCPP signal such as OpenADR or
OSCP is received and based on this signal, the CSMS limits charging on its Charging Stations. CSOs that want full control over their
Charging Station use this architecture, this way they are in control of the amount of energy being used by their Charging Stations.
This can be done by sending charging profiles / charging schedules to Charging Stations.
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 16/22 Part 1 - Architecture & Topology

CSMSDSO ChargingStationNon-OCPP (e.g. OpenADR or OSCP)OCPP
Figure 11. Smart Charging - DSO control signals to CSMS
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 17/22 Part 1 - Architecture & Topology


### Chapter 9. Energy management topologies supported by OCPP

*_Source: Page 20 - 24_*

Chapter 9. Energy management topologies supported by OCPP
(New in OCPP 2.1)
This chapter describes various topologies that can be used when combining an external actor for energy management with
Charging Stations. The external actor can be a full-fledged (home) energy management system, often abbreviated as EMS or HEMS,
but it can also be a smart meter that provides a maximum power limit. It is not meant to be an exhaustive list of possibilities, and in
the future other topologies may become possible.
In the diagrams the following convention is used for the connectors between components:
Legend
a b
measurements
control
ocpp
9.1. Parallel control of charging station by CSMS and smart meter
In this setup a Charging Station is connected to a smart meter of the grid connection for the premise. The smart meter provides a
charging limit to the Charging Station, such that the power consumption of the Charging Station will be reduced if the capacity of
the grid connection is about to be reached.
Site
Charging
Station
Smart Meter
Grid
Connection
CSMS
Third Parties
e.g. aggregator
OCPP
Figure 12. Parallel control by CSMS and smart meter
9.2. Parallel control of charging location by CSMS and EMS
In this setup a Charging Location with one or more Charging Stations is equipped with an Energy Management System (EMS).
CSMS controls the Charging Stations via OCPP, but local load-balancing on-site is controlled by the EMS. EMS will have its own
connection to a Charging Station, using its own protocol, for example Modbus. If a Charging Station receives a charging constraint
from EMS, then it will represent this constraint internally as an OCPP charging profile with purpose
ChargingStationExternalConstraints. This charging profile is combined with other charging profiles that it might receive
from CSMS. When such an external constraint is received by the Charging Station, it will immediately report this constraint to CSMS
via the NotifyChargingLimitRequest message. A limitation of this topology is that EMS is not aware of OCPP information that is
exchanged between Charging Station and CSMS. EMS can therefore not know who is charging at a Charging Station, or what the
specific charging needs of a user are. Local balancing based on user needs (e.g. time of departure) or priorities is not possible in
this topology.
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 18/22 Part 1 - Architecture & Topology

Site
Other loadsLocal Generation
Charging
Station 1
Charging
Station 2
EMS
Grid Connection
Utility/DSO
Aggregator
CSMS
control
TxProfileNotifyChargingLimitNotifyChargingLimit
Figure 13. Parallel control by CSMS and EMS
9.3. EMS via Local Controller
The limitation of Parallel control of charging location by CSMS and EMS can be overcome with help of a Local Controller
component. A Local Controller is a kind of "local CSMS" (see Multiple Charging Stations connected to CSMS via Local Controller),
that uses OCPP messages to perform local load-balancing. In this topology the Energy Management System (EMS) is connected to
the Local Controller. EMS treats all Charging Stations at Charging Location as a single load, and provides its constraint to the Local
Controller. The Local Controller will represent this constraint internally as a ChargingStationExternalConstraints charging
profile for the cluster of Charging Stations at the Charging Location. It is up to the Local Controller to divide the available capacity
among the Charging Stations in the cluster. Because all OCPP traffic between Charging Station and CSMS passes through the
Local Controller, it can be made aware of user needs and priorities, and use this information for intelligent scheduling. An added
advantage of an on-site Local Controller is, that it can continue to function and support local load-balancing, even when connection
to CSMS is lost.
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 19/22 Part 1 - Architecture & Topology

Site
Other loadsLocal PV Generation
Charging
Station 1
Charging
Station 2
Local ControllerEMS
Grid Connection
DSO
Aggregator
CSMS
ExternalConstraints,
LocalGeneration
ExternalConstraints 1,
LocalGeneration 1,
TxProfile 1
ExternalConstraints 2,
LocalGeneration 2,
TxProfile 2
TxProfile (optional)
Figure 14. EMS via Local Controller
9.4. EMS as man-in-the-middle
In the topology sketched above in EMS via Local Controller it is a logical step to combine the EMS and Local Controller functionality
in one box. EMS is acting as a Local Controller and is placed as a "man-in-the-middle" between CSMS and Charging Stations. An
advantage of this setup is, that EMS (as part of a Local Controller) is aware of instructions coming from CSMS. This enables EMS
to know about a charging limitation set by CSMS to the (cluster of) Charging Stations. Having this knowledge allows for more
sophisticated energy management, because EMS can now differentiate between the situation where an EV is charging at low power
because it does not need more power, and the situation where it is not allowed more power by CSMS or Local Controller.
Site
Other loadsLocal PV Generation
ChargingStation 1ChargingStation 2
EMS asLocal Controller
Grid Connection
DSO
Aggregator
CSMS
measurements
TxProfile 1TxProfile 2
control TxProfile (optional)
Figure 15. EMS as man-in-the-middle
9.5. Hybrid local & cloud EMS
The hybrid local & cloud EMS topology describes the situation where an advanced EMS is running in the cloud. This cloud EMS can
have advanced scheduling algorithms and has access to external information, like weather forecasts and control signals from a
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 20/22 Part 1 - Architecture & Topology

DSO. Since it is running in the cloud it can even optimize across multiple sites. An EMS in the cloud will likely not be able to react
fast enough to protect the local grid connection, and it will not be able to control the site when the internet connection is lost. This
topology therefore adds a local EMS, whose main task is to protect the fuse of the local grid connection, to pass data with local
load measurements on to the cloud EMS, and to act as a fallback when data connection to the cloud is lost. Charging Stations can
connect directly to the Cloud EMS acting as a Local Controller, as shown below.
Site
Other loadsLocal PV Generation
ChargingStation 1 ChargingStation 2
Local EMS
Grid Connection
DSO
Aggregator
CSMS
Cloud EMS asLocal Controller
Figure 16. Hybrid topology with cloud EMS as Local Controller
Alternatively, the Local Controller function can be performed by the Local EMS. The Cloud EMS scheduling will be suboptimal,
however, because in this case it is not aware of the state of ongoing transactions, unless this information is explicitly passed by
Local EMS to Cloud EMS.
Site
Other loadsLocal PV Generation
ChargingStation 1ChargingStation 2
Local EMS asLocal Controller
Grid Connection
DSO
Aggregator
CSMS
Cloud EMS asScheduler
Figure 17. Hybrid topology with local EMS as Local Controller
9.6. Parallel control by CSMS and EMS
(Updated in OCPP 2.1 and moved from chapter 8 to 9)
Description
In a (semi-)private situation where a Charging Station is not only connected to the CSMS, but also to an Energy Management
System, some form of parallel control is possible. OCPP is then used for transaction handling and management of the Charging
Station, and the Energy Management System provides smart charging controls. OCPP 2.1 supports reporting external smart
charging control limits. Control limits that EMS provides via its own protocol, are represented (and reported as)
"ExternalConstraints" charging profiles by the Charging Station.
When the Energy Management System decides to delay charging, the Energy Management System can impose an external limit
(e.g. 0) to a Charging Station, which the Charging Station in turn can report to the CSMS via OCPP. The Energy Management System
might get input from e.g. Local port of a Smart Meter to prevent overloading the grid connection, but can also have other reasons
for not charging (e.g. weather conditions).
The protocol between Charging Station and EMS is not specified. Charging limits or schedules can be provided by any means that
Charging Station supports. This can also be implemented using OCPP messages. See also Part 2, section K 2.4 for some topology
examples with an EMS.
NOTE
An OCPP message exchange between Charging Station and EMS is not a full-fledged OCPP connection in which
EMS acts as the server. It is a limited solution consisting of a websocket connection over which EMS sends, for
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 21/22 Part 1 - Architecture & Topology

example, a SetChargingProfile message, and Charging Station sends MeterValue messages.
CSMS EMSCharging
Station
OCPP OCPP or other
Figure 18. Parallel control by CSMS and EMS
Edition 2, 2025-12-03
OCPP 2.1 Edition 2 - © Open Charge Alliance 2025 22/22 Part 1 - Architecture & Topology


---

## Technical Analysis & Implementation Guide

> **Note**: This section provides technical analysis and implementation guidance for OCPP 2.1 architecture, Device Model, and topologies.

### Design Intent & Architecture Principles

#### The 3-Tier Model: Foundation of OCPP 2.1

The 3-tier model is the cornerstone of OCPP 2.1's architecture:

```
┌────────────────────────────────────────────────────────────┐
│ Tier 1: Charging Station (Top Level)                       │
│ - Physical enclosure with communication capability        │
│ - Contains one or more EVSEs                               │
│ - Single OCPP connection to CSMS                           │
└────────────────────┬───────────────────────────────────────┘
                     │
┌────────────────────┴───────────────────────────────────────┐
│ Tier 2: EVSE (Electric Vehicle Supply Equipment)           │
│ - Independently operated charging point                    │
│ - Can deliver energy to one EV at a time                   │
│ - May have multiple connectors                              │
└────────────────────┬───────────────────────────────────────┘
                     │
┌────────────────────┴───────────────────────────────────────┐
│ Tier 3: Connector                                          │
│ - Physical outlet (socket or cable)                        │
│ - Point of electrical connection to EV                     │
│ - Identified by connectorId within an EVSE                 │
└────────────────────────────────────────────────────────────┘
```

**Key Design Principles**:

1. **Logical, Not Physical**: The 3 tiers describe logical structure, not physical packaging
   - Example: A charging plaza with 20 EVSEs communicating via 1 modem = 1 Charging Station
   - Manufacturers choose how to map logical tiers to physical hardware

2. **Hierarchical Addressing**: Every component is addressable within this hierarchy
   - Charging Station level: `Controller`, `RadioLink`, etc.
   - EVSE level: `EVSE` (identified by `evseId`)
   - Connector level: `Connector` (identified by `evseId` + `connectorId`)

3. **Flexible Composition**: Components can exist at any tier
   - Some components are Charging Station-wide (e.g., `Controller`)
   - Some are per-EVSE (e.g., `TxCtrlCtrl`)
   - Some are per-Connector (e.g., `ConnectorLock`)

#### Device Model vs. Information Model

OCPP 2.1 explicitly separates two models:

**Device Model** (Physical Structure):
- Describes how the Charging Station is built
- Components and Variables organized by 3-tier hierarchy
- Managed via `GetVariables`, `SetVariables`, `GetReport`
- Standardized in Part 2 Appendices

**Information Model** (Message Payloads):
- Describes data exchanged in OCPP messages
- Transaction-related data (e.g., `TransactionEvent`)
- Business logic and use case implementation
- Defined in Part 2 functional blocks

**Why This Separation Matters**:
- Device Model enables universal management without prior knowledge
- Information Model carries business logic for specific use cases
- CSMS can discover Device Model dynamically
- Information Model remains stable across different hardware

### Deep Dive: Components and Variables

#### Component Structure

A **Component** represents a physical device, logical functionality, or data entity:

```rust
/// Component reference in OCPP 2.1 Device Model
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    /// Component name (e.g., "Controller", "EVSE", "Connector")
    pub name: String,

    /// Optional instance identifier for multiple occurrences
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,

    /// EVSE ID (if component is at EVSE or Connector level)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i32>,

    /// Connector ID (if component is at Connector level)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<i32>,
}

impl Component {
    /// Create Charging Station level component
    pub fn charging_station_level(name: &str) -> Self {
        Component {
            name: name.to_string(),
            instance: None,
            evse_id: None,
            connector_id: None,
        }
    }

    /// Create EVSE level component
    pub fn evse_level(name: &str, evse_id: i32) -> Self {
        Component {
            name: name.to_string(),
            instance: None,
            evse_id: Some(evse_id),
            connector_id: None,
        }
    }

    /// Create Connector level component
    pub fn connector_level(name: &str, evse_id: i32, connector_id: i32) -> Self {
        Component {
            name: name.to_string(),
            instance: None,
            evse_id: Some(evse_id),
            connector_id: Some(connector_id),
        }
    }

    /// Create component with instance (for multiple occurrences)
    pub fn with_instance(mut self, instance: &str) -> Self {
        self.instance = Some(instance.to_string());
        self
    }
}
```

#### Variable Structure

A **Variable** holds data associated with a Component:

```rust
/// Variable in OCPP 2.1 Device Model
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Variable {
    /// Variable name
    pub name: String,

    /// Optional instance identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,

    /// Component this variable belongs to
    pub component: Component,
}

/// Variable characteristics (from GetVariables response)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariableCharacteristics {
    /// Data type (e.g., "string", "decimal", "integer")
    pub data_type: String,

    /// Minimum value (for numeric types)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_limit: Option<BigDecimal>,

    /// Maximum value (for numeric types)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_limit: Option<BigDecimal>,

    /// Allowed values (for enum types)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values_list: Option<Vec<String>>,

    /// Unit of measurement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

/// Variable monitoring criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariableMonitoring {
    /// Variable being monitored
    pub variable: Variable,

    /// Monitoring type (Periodic, Delta, etc.)
    pub monitoring_type: MonitoringType,

    /// Threshold value (for Delta monitoring)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<BigDecimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MonitoringType {
    Periodic,
    Delta,
    PeriodicClockAligned,
    TriggeredByMutation,
}
```

### Common Standardized Components

OCPP 2.1 defines standardized components in Part 2 Appendices. Here are the most important ones:

#### Charging Station Level Components

| Component | Description | Typical Variables |
|-----------|-------------|-------------------|
| `Controller` | Main controller | `HeartbeatInterval`, `Clock` |
| `RadioLink` | Communication module | `SignalStrength`, `APN` |
| `LocalAuthList` | Authorization cache | `Enabled`, `MaxEntries` |
| `ChargingStation` | Station-wide settings | `Alignment`, `Freeze` |

#### EVSE Level Components

| Component | Description | Typical Variables |
|-----------|-------------|-------------------|
| `EVSE` | EVSE itself | `Availability`, `MaxCurrent` |
| `TxCtrlCtrl` | Transaction controller | `TxStartPoint`, `TxStopPoint` |
| `ACPhaseSwitching` | 3-phase switching | `MaxCurrent`, `Mode` |

#### Connector Level Components

| Component | Description | Typical Variables |
|-----------|-------------|-------------------|
| `Connector` | Connector itself | `Availability`, `PhaseRotation` |
| `ConnectorLock` | Cable lock | `Available`, `UnlockedOnEVSide` |
| `PlugRetention` | Plug retention mechanism | `Available` |

### Rust Implementation: Device Model Registry

For a production OCPP 2.1 implementation, create a Device Model registry:

```rust
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::RwLock as TokioRwLock;

/// Device Model registry for a Charging Station
pub struct DeviceModel {
    /// All components in this Charging Station
    components: Arc<TokioRwLock<HashMap<ComponentKey, ComponentData>>>,

    /// Variable values
    variables: Arc<TokioRwLock<HashMap<VariableKey, VariableData>>>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ComponentKey {
    pub name: String,
    pub instance: Option<String>,
    pub evse_id: Option<i32>,
    pub connector_id: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct ComponentData {
    pub component: Component,
    pub variables: Vec<String>, // Variable names
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct VariableKey {
    pub component: ComponentKey,
    pub name: String,
    pub instance: Option<String>,
}

#[derive(Debug, Clone)]
pub struct VariableData {
    pub variable: Variable,
    pub characteristics: VariableCharacteristics,
    pub current_value: Option<VariableDataType>,
    pub monitoring: Vec<VariableMonitoring>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VariableDataType {
    String(String),
    Decimal(BigDecimal),
    Integer(i64),
    Boolean(bool),
}

impl DeviceModel {
    pub fn new() -> Self {
        DeviceModel {
            components: Arc::new(TokioRwLock::new(HashMap::new())),
            variables: Arc::new(TokioRwLock::new(HashMap::new())),
        }
    }

    /// Get a variable value
    pub async fn get_variable(&self, variable: &Variable) -> Option<VariableDataType> {
        let variables = self.variables.read().await;
        let key = Self::variable_key(variable);
        variables.get(&key).and_then(|v| v.current_value.clone())
    }

    /// Set a variable value
    pub async fn set_variable(&self, variable: &Variable, value: VariableDataType) -> Result<(), OcppError> {
        let mut variables = self.variables.write().await;
        let key = Self::variable_key(variable);

        if let Some(var_data) = variables.get_mut(&key) {
            // Validate against characteristics
            Self::validate_value(&value, &var_data.characteristics)?;

            // Update value
            var_data.current_value = Some(value);
            Ok(())
        } else {
            Err(OcppError::Protocol(format!("Variable not found: {:?}", key)))
        }
    }

    /// Add monitoring for a variable
    pub async fn add_monitoring(&self, monitoring: VariableMonitoring) {
        let mut variables = self.variables.write().await;
        let key = Self::variable_key(&monitoring.variable);

        if let Some(var_data) = variables.get_mut(&key) {
            var_data.monitoring.push(monitoring);
        }
    }

    fn variable_key(variable: &Variable) -> VariableKey {
        VariableKey {
            component: ComponentKey {
                name: variable.component.name.clone(),
                instance: variable.component.instance.clone(),
                evse_id: variable.component.evse_id,
                connector_id: variable.component.connector_id,
            },
            name: variable.name.clone(),
            instance: variable.instance.clone(),
        }
    }

    fn validate_value(value: &VariableDataType, characteristics: &VariableCharacteristics) -> Result<(), OcppError> {
        match value {
            VariableDataType::Decimal(d) => {
                if let Some(min) = &characteristics.min_limit {
                    if d < min {
                        return Err(OcppError::PropertyConstraint {
                            field: "value".to_string(),
                            value: d.to_string(),
                        });
                    }
                }
                if let Some(max) = &characteristics.max_limit {
                    if d > max {
                        return Err(OcppError::PropertyConstraint {
                            field: "value".to_string(),
                            value: d.to_string(),
                        });
                    }
                }
            }
            VariableDataType::String(s) => {
                if let Some(ref values) = characteristics.values_list {
                    if !values.contains(s) {
                        return Err(OcppError::PropertyConstraint {
                            field: "value".to_string(),
                            value: s.clone(),
                        });
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}
```

### Topology Analysis & Implementation Guidance

#### Topology 1: Direct Connection (Simplest)

```
CSMS ←→ Charging Station
```

**Implementation**: Standard OCPP-J WebSocket connection
**Use Case**: Simple deployments, home chargers, small public sites
**Complexity**: Low

#### Topology 2: Local Proxy (Message Routing)

```
CSMS ←→ Local Proxy ←→ Charging Station 1
                       ←→ Charging Station 2
```

**Purpose**: Reduce number of concurrent connections to CSMS
**Implementation**: Local Proxy acts as WebSocket router
**Key Point**: Charging Stations think they're talking to CSMS directly

```rust
// Local Proxy routes messages between CSMS and Charging Stations
pub struct LocalProxy {
    csms_connection: WebSocketConnection,
    charging_stations: HashMap<ChargingStationId, WebSocketConnection>,
}

impl LocalProxy {
    pub async fn route_message(&self, source: Source, message: OcppMessage) {
        match source {
            Source::Csms => {
                // Route to appropriate Charging Station
                if let Some(cs_conn) = self.get_destination_station(&message) {
                    cs_conn.send(message).await;
                }
            }
            Source::ChargingStation(id) => {
                // Route to CSMS
                self.csms_connection.send(message).await;
            }
        }
    }
}
```

#### Topology 3: Local Controller (Smart Charging)

```
CSMS ←→ Local Controller ←→ Charging Station 1
       ↓                   ←→ Charging Station 2
    (sends charging profiles)
```

**Purpose**: Local smart charging decision-making
**Key Difference from Local Proxy**: Local Controller sends its own messages
**Critical**: Local Controller can impose charging limits independently

**Implementation**:
```rust
// Local Controller can send SetChargingProfile to Charging Stations
pub struct LocalController {
    csms_connection: WebSocketConnection,
    charging_stations: HashMap<ChargingStationId, WebSocketConnection>,
    smart_charging_engine: SmartChargingEngine,
}

impl LocalController {
    // Local Controller decides to limit power
    pub async fn impose_local_limit(&self, evse_id: i32, max_power: f64) {
        let profile = ChargingProfile {
            purpose: ChargingProfilePurposeType::ChargingStationMaxProfile,
            schedule: vec![ChargingSchedulePeriod {
                start_period: 0,
                limit: max_power,
                number_phases: 3,
            }],
        };

        for (id, conn) in &self.charging_stations {
            let msg = OcppMessage::set_charging_profile(evse_id, profile);
            conn.send(msg).await;
        }
    }
}
```

#### Topology 4: Local Controller with Non-OCPP Charging Stations

```
CSMS ←→ Local Controller ←→ Non-OCPP Charging Station 1
                          ←→ Non-OCPP Charging Station 2
```

**Purpose**: Integrate legacy charging stations via OCPP
**Implementation**: Local Controller translates between protocols
**Critical**: Local Controller presents OCPP interface to CSMS

#### Topology 5: Parallel Control (CSMS + EMS)

```
         ←→ CSMS (OCPP)
Charging Station
         ←→ EMS (proprietary)
```

**Purpose**: Smart charging via local Energy Management System
**Key Feature (OCPP 2.1)**: Charging Station reports "ExternalConstraints"
**Implementation**: Charging Station receives limits from both CSMS and EMS

```rust
// Charging Station handles two control sources
pub struct ChargingStationController {
    csms_limit: Option<f64>,
    ems_limit: Option<f64>,
    current_limit: f64, // Minimum of CSMS and EMS limits
}

impl ChargingStationController {
    pub fn update_csms_limit(&mut self, limit: f64) {
        self.csms_limit = Some(limit);
        self.recalculate_limit();
    }

    pub fn update_ems_limit(&mut self, limit: f64) {
        self.ems_limit = Some(limit);
        self.recalculate_limit();
    }

    fn recalculate_limit(&mut self) {
        self.current_limit = match (self.csms_limit, self.ems_limit) {
            (Some(csms), Some(ems)) => csms.min(ems),
            (Some(csms), None) => csms,
            (None, Some(ems)) => ems,
            (None, None) => MAX_POWER,
        };

        // Report ExternalConstraints to CSMS (if EMS limit is active)
        if let Some(ems) = self.ems_limit {
            self.report_external_constraints(ems);
        }
    }

    fn report_external_constraints(&self, limit: f64) {
        // Send NotifyReportRequest with ExternalConstraints profile
    }
}
```

### Best Practices for Topology Implementation

#### ✅ Recommended Approaches

1. **Start Simple, Add Complexity as Needed**
   - Begin with direct CSMS ↔ Charging Station connection
   - Add Local Proxy only if connection count is an issue
   - Add Local Controller only for local smart charging

2. **Maintain Device Model Consistency**
   - Device Model must reflect actual hardware structure
   - Keep Component-Variable hierarchy synchronized
   - Support dynamic discovery via `GetBaseReport`

3. **Handle Multi-EVSE Charging Stations Correctly**
   - Each EVSE must have unique `evseId`
   - Transactions are per-EVSE, not per-Connector
   - Support concurrent transactions on different EVSEs

4. **Support Offline Operation**
   - Cache authorized IdTokens locally
   - Continue charging when CSMS unreachable
   - Sync when connection restored

#### ❌ Common Pitfalls to Avoid

1. **Confusing EVSE and Connector**
   - An EVSE can have multiple Connectors
   - Only one transaction per EVSE at a time
   - Don't assume 1:1 EVSE:Connector mapping

2. **Ignoring Component Instance**
   - Multiple components can have the same name
   - Use `instance` field to distinguish them
   - Example: Multiple power converter modules

3. **Hardcoding Component Paths**
   - Don't assume fixed Device Model structure
   - Always discover Device Model dynamically
   - Support vendor-specific components

4. **Misunderstanding Local Controller Role**
   - Local Controller is NOT a simple message router
   - Local Controller sends its own OCPP messages
   - Local Controller can act independently of CSMS

5. **Neglecting ExternalConstraints Reporting**
   - When EMS limits power, report it to CSMS
   - Use `ExternalConstraints` charging profile type
   - Enables CSMS visibility into local control decisions

### Integration Example: Building a Charging Station

Here's a complete example of setting up a Charging Station with proper Device Model:

```rust
use ocpp_2_1::device_model::{DeviceModel, Component, Variable};
use ocpp_2_1::messages::BootNotificationRequest;

pub struct ChargingStation {
    device_model: DeviceModel,
    ocpp_client: OcppClient,
    evses: Vec<EvseController>,
}

impl ChargingStation {
    pub async fn new(config: StationConfig) -> Result<Self, OcppError> {
        let mut device_model = DeviceModel::new();

        // Initialize Charging Station level components
        device_model.add_component(Component::charging_station_level("Controller"));
        device_model.add_component(Component::charging_station_level("RadioLink"));

        // Initialize EVSEs
        let mut evses = Vec::new();
        for evse_id in 1..=config.num_evses {
            // Add EVSE component
            device_model.add_component(Component::evse_level("EVSE", evse_id));
            device_model.add_component(Component::evse_level("TxCtrlCtrl", evse_id));

            // Add Connectors for this EVSE
            for connector_id in 1..=config.connectors_per_evse {
                device_model.add_component(
                    Component::connector_level("Connector", evse_id, connector_id)
                );

                device_model.add_component(
                    Component::connector_level("ConnectorLock", evse_id, connector_id)
                );
            }

            evses.push(EvseController::new(evse_id, &config));
        }

        // Connect to CSMS
        let ocpp_client = OcppClient::connect(&config.csms_url).await?;

        Ok(ChargingStation {
            device_model,
            ocpp_client,
            evses,
        })
    }

    pub async fn boot(&self) -> Result<BootNotificationResponse, OcppError> {
        let request = BootNotificationRequest {
            reason: BootReasonType::PowerUp,
            charging_station: ChargingStationType {
                model: self.device_model.get_variable("Controller", "Model").await?,
                serial_number: self.device_model.get_variable("Controller", "SerialNumber").await?,
                vendor_name: self.device_model.get_variable("Controller", "VendorName").await?,
                firmware_version: self.device_model.get_variable("Controller", "FirmwareVersion").await?,
                ..Default::default()
            },
        };

        self.ocpp_client.call(request).await
    }
}
```

### References to Other Parts

- **Part 0**: Introduction and basic implementation guidance
- **Part 2**: Detailed functional blocks and use cases
  - B. Provisioning: BootNotification, configuration
  - C. Authorization: Local authorization list
  - K. Smart Charging: Charging profiles, Local Controller
- **Part 4**: OCPP-J WebSocket protocol
- **Part 2 Appendices**: Standardized components and variables

---

**Analysis completed**: 2025-01-25
**Ralph Loop Iteration**: 1
**Phase**: Technical Analysis - Part 1 Architecture Foundation

