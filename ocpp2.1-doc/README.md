# OCPP 2.1 Edition 2 Documentation

> **Open Charge Point Protocol Version 2.1, Edition 2**
>
> Complete documentation extracted and structured from official OCPP 2.1 PDF specifications.

---

## Quick Navigation

| Part | Document | Description | Size |
|------|----------|-------------|------|
| **Part 0** | [Introduction](./part0-introduction.md) | Overview, terms, version history | 30 KB |
| **Part 1** | [Architecture & Topology](./part1-architecture-topology.md) | System architecture and communication topology | 50 KB |
| **Part 2** | [Specification](./part2-specification.md) | Core protocol specification (all functional blocks) | 1.8 MB |
| **Part 2** | [Appendices](./part2-appendices.md) | Detailed appendices and supplementary material | 144 KB |
| **Part 4** | [OCPP-J Specification](./part4-ocpp-j-specification.md) | JSON over WebSocket protocol specification | 64 KB |
| **Part 5** | [Certification Profiles](./part5-certification-profiles.md) | Certification requirements and compliance | 131 KB |
| **Part 6** | [Test Cases](./part6-testcases.md) | Complete test case collection | 2.1 MB |

**Total**: ~4.4 MB of documentation (107,000+ lines)

---

## About This Documentation

This repository contains the complete OCPP 2.1 Edition 2 specification, extracted from official PDF documents and converted to structured Markdown format for better accessibility and searchability.

### Source Documents

All content is sourced from the official OCPP 2.1 Edition 2 PDF files:

```
OCPP-2.1_Edition2_all_files/
├── OCPP-2.1_edition2_part0_introduction.pdf              (17 pages)
├── OCPP-2.1_edition2_part1_architecture_topology.pdf     (24 pages)
├── OCPP-2.1_edition2_part2_specification pdf拆分/        (25+ files)
├── OCPP-2.1_edition2_part2_appendices_v21.pdf           (54 pages)
├── OCPP-2.1_edition2_part4_ocpp-j-specification.pdf     (30 pages)
├── OCPP-2.1_edition2_part5_certification_profiles.pdf   (70 pages)
└── OCPP-2.1_edition2_part6-testcases.pdf                (1385 pages)
```

### Documentation Structure

```
ocpp2.1-doc/
├── README.md                           # This file - navigation and overview
├── part0-introduction.md               # Part 0: Introduction
├── part1-architecture-topology.md      # Part 1: Architecture & Topology
├── part2-specification.md              # Part 2: Core Specification
├── part2-appendices.md                 # Part 2: Appendices
├── part4-ocpp-j-specification.md       # Part 4: OCPP-J Protocol
├── part5-certification-profiles.md     # Part 5: Certification
├── part6-testcases.md                  # Part 6: Test Cases
├── images/                             # Extracted diagrams and figures
│   ├── part0/
│   ├── part1/
│   ├── part2/
│   ├── part4/
│   ├── part5/
│   └── part6/
└── sources/
    └── source-map.json                 # Chapter to PDF file mapping
```

---

## Document Parts Overview

### Part 0: Introduction

**[Read Document →](./part0-introduction.md)**

- OCPP 2.1 overview and objectives
- Document structure explanation
- Terms and abbreviations
- Version history
- Basic implementation guidance

**Key Sections**:
- Chapter 1: Introduction
- Chapter 2: New functionality in OCPP 2.1
- Chapter 3: Documentation structure
- Chapter 4: Basic implementation

**Source**: `OCPP-2.1_edition2_part0_introduction.pdf` (17 pages)

---

### Part 1: Architecture & Topology

**[Read Document →](./part1-architecture-topology.md)**

- System architecture overview
- Communication topology
- Component relationships
- Network architecture
- Security architecture

**Key Topics**:
- Charging Station architecture
- CSMS (Central System) architecture
- Communication patterns
- Network topologies

**Source**: `OCPP-2.1_edition2_part1_architecture_topology.pdf` (24 pages)

---

### Part 2: Specification (Core)

**[Read Document →](./part2-specification.md)**

The heart of OCPP 2.1 - complete protocol specification with all functional blocks.

**Functional Blocks** (A-S):

1. **A. Security** - Authentication, encryption, security policies
2. **B. Provisioning** - Configuration, provisioning workflows
3. **C. Authorization** - Authorization mechanisms
4. **D. Local Authorization List Management** - Local authorization management
5. **E. Transactions** - Transaction handling
6. **F. Remote Control** - Remote control operations
7. **G. Availability** - Availability management
8. **H. Reservation** - Reservation functionality
9. **I. Tariff and Cost** - Pricing and cost calculation
10. **J. Meter Values** - Metering and values
11. **K. Smart Charging** - Smart charging algorithms
12. **L. Firmware Management** - Firmware updates
13. **M. Certificate Management** - Certificate handling
14. **N. Diagnostics** - Diagnostic operations
15. **O. Display Message** - Display operations
16. **P. Data Transfer** - Data transfer mechanisms
17. **Q. Bidirectional Power Transfer** - V2G support
18. **R. DER Control** - Distributed Energy Resources
19. **S. Battery Swapping** - Battery swapping operations

**Additional Sections**:
- Generic specification
- Messages, datatypes, and enumerations
- Referenced components and variables

**Source**: 25+ PDF files from `OCPP-2.1_edition2_part2_specification pdf拆分/`

---

### Part 2: Appendices

**[Read Document →](./part2-appendices.md)**

- Appendices A-S detailed explanations
- Supplementary material
- Additional technical details
- Reference information

**Source**: `OCPP-2.1_edition2_part2_appendices_v21.pdf` (54 pages)

---

### Part 4: OCPP-J Specification

**[Read Document →](./part4-ocpp-j-specification.md)**

Complete specification for OCPP-J (JSON over WebSocket).

**Key Topics**:
- JSON message format
- WebSocket implementation
- Message routing
- Error handling
- Protocol versioning

**Source**: `OCPP-2.1_edition2_part4_ocpp-j-specification.pdf` (30 pages)

---

### Part 5: Certification Profiles

**[Read Document →](part5-certification-profiles.md)**

Requirements and procedures for OCPP 2.1 certification.

**Key Topics**:
- Certification profiles
- Compliance requirements
- Testing procedures
- Certification criteria

**Source**: `OCPP-2.1_edition2_part5_certification_profiles.pdf` (70 pages)

---

### Part 6: Test Cases

**[Read Document →](./part6-testcases.md)**

Comprehensive test case collection for OCPP 2.1 implementation verification.

**Contents**:
- Complete test scenarios
- Validation steps
- Expected results
- Test coverage for all functional blocks

**Source**: `OCPP-2.1_edition2_part6-testcases.pdf` (1385 pages)

---

## How to Use This Documentation

### For Implementers

If you're implementing an OCPP 2.1 charging station or CSMS:

1. **Start with Part 0** - Read the introduction for basic understanding
2. **Review Part 1** - Understand the architecture and topology
3. **Study Part 2** - Deep dive into the specification and relevant functional blocks
4. **Implement OCPP-J** - Follow Part 4 for the JSON over WebSocket protocol
5. **Test with Part 6** - Use test cases to verify your implementation

### For Certification

If you're seeking OCPP 2.1 certification:

1. **Part 5** - Review certification profiles and requirements
2. **Part 6** - Prepare for certification testing with test cases
3. **Part 2** - Ensure all mandatory functional blocks are implemented

### Quick Reference

- **Message Definitions**: Part 2 - Messages, Datatypes & Enumerations section
- **Data Types**: Part 2 - Referenced Components and Variables
- **Protocol Details**: Part 4 - OCPP-J Specification
- **Testing**: Part 6 - Test Cases

---

## Source Mapping

Every section in these markdown documents is traceable to its source PDF.

The `sources/source-map.json` file contains:
- Source PDF filenames
- Chapter/page mappings
- Markdown section references

**Example**:
```json
{
  "part0-introduction.md": {
    "source_pdf": "OCPP-2.1_edition2_part0_introduction.pdf",
    "chapters": [
      {
        "title": "Introduction",
        "page_range": "1-10",
        "md_section": "# 1. Introduction"
      }
    ]
  }
}
```

---

## Technical Analysis & Implementation Notes

> **Note**: This documentation is currently in **Phase 1: Content Extraction**.
>
> Future iterations (via Ralph Loop) will add:
> - Technical principle analysis
> - Rust implementation suggestions
> - Best practices and common pitfalls
> - Example scenarios
> - Cross-references between sections

---

## Contributing & Improvement

This documentation uses the **Ralph Wiggum technique** for iterative improvement.

Each iteration adds:
1. ✅ Content completeness verification
2. ✅ Technical accuracy validation
3. ✅ Readability improvements
4. ✅ Practical implementation examples
5. ✅ Cross-reference enhancements

---

## Version Information

- **OCPP Version**: 2.1 Edition 2
- **Documentation Date**: 2025-12-03
- **Extraction Date**: 2025-01-25
- **Format**: Markdown (converted from PDF)

---

## Official OCPP Resources

- [OCPP Website](https://www.openchargealliance.org/)
- [OCPP 2.1 Specification](https://www.openchargealliance.org/protocols/ocpp-2-1/)
- [Open Charge Alliance](https://www.openchargealliance.org/)

---

## License

This documentation is derived from the official OCPP 2.1 Edition 2 specification. Please refer to the Open Charge Alliance for licensing terms and conditions for the OCPP protocol.

The extraction tooling and markdown formatting are provided for ease of use and accessibility.

---

**Last Updated**: 2025-01-25
**Documentation Phase**: Phase 1 - Content Extraction Complete ✅
