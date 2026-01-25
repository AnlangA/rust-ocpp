# OCPP 2.1 Edition 2 Documentation Processing Plan

## Project Overview

Convert all OCPP 2.1 Edition 2 documentation files into organized Markdown format with proper source tracking.

### Source Materials (OCPP-2.1_Edition2_all_files/)

**PDF Documents:**
1. `OCPP-2.1_edition2_part0_introduction.pdf` (1.3M)
2. `OCPP-2.1_edition2_part1_architecture_topology.pdf` (1.2M)
3. `OCPP-2.1_edition2_part2_specification.pdf` (24M) - Requires chapter-based splitting
4. `OCPP-2.1_edition2_part2_appendices_v21.pdf` (2.6M)
5. `OCPP-2.1_edition2_part4_ocpp-j-specification.pdf` (1.2M)
6. `OCPP-2.1_edition2_part5_certification_profiles.pdf` (3.8M)
7. `OCPP-2.1_edition2_part6-testcases.pdf` (5.3M)

**JSON Schemas:** 100+ schema files in `OCPP-2.1_part3_JSON_schemas/`

**CSV Data:** `Appendices_CSV_v2.1/` directory

### Output Structure (ocpp2.1-doc/)

```
ocpp2.1-doc/
├── part0-introduction/
│   ├── 01-overview.md
│   ├── 02-scope.md
│   └── ...
├── part1-architecture/
│   ├── 01-network-architecture.md
│   └── ...
├── part2-specification/
│   ├── 01-introduction.md
│   ├── 02-message-format.md
│   └── ...
├── part2-appendices/
│   ├── 01-components.md
│   └── ...
├── part3-schemas/
│   ├── 01-schemas-overview.md
│   ├── AuthorizeRequest.md
│   ├── AuthorizeResponse.md
│   └── ...
├── part4-ocppj/
│   └── ...
├── part5-certification/
│   └── ...
├── part6-testcases/
│   └── ...
└── appendices-csv/
    └── ...
```

## Processing Strategy

### Phase 1: Large PDF Splitting (part2_specification.pdf)

**Tool:** `document-skills:pdf` skill

**Steps:**
1. Extract bookmark/chapter structure from `OCPP-2.1_edition2_part2_specification.pdf`
2. Split PDF into chapter-based sections
3. Save split files to temporary directory: `.tmp/part2-split/`
4. Each split file should be named by chapter number and title

### Phase 2: Ralph Loop Iterations

**Total Iterations:** 8 (one per PDF file)

**Completion Promise:** `<promise>OCPP_2.1_DOCUMENTATION_COMPLETE</promise>`

#### Iteration 1: Part 0 - Introduction
**Source:** `OCPP-2.1_edition2_part0_introduction.pdf`
**Output:** `ocpp2.1-doc/part0-introduction/`
**Tasks:**
- Extract all chapters/sections
- Create one MD file per section
- Include source path and chapter reference in header

#### Iteration 2: Part 1 - Architecture & Topology
**Source:** `OCPP-2.1_edition2_part1_architecture_topology.pdf`
**Output:** `ocpp2.1-doc/part1-architecture/`
**Tasks:**
- Extract network topology diagrams and descriptions
- Document architecture patterns
- Create MD files per section

#### Iteration 3: Part 2 - Specification (Split Files)
**Source:** `.tmp/part2-split/*.pdf` (from Phase 1)
**Output:** `ocpp2.1-doc/part2-specification/`
**Tasks:**
- Process each split PDF chapter
- Create MD file per chapter
- Extract message definitions, data types, enums
- Document all OCPP messages with structure

#### Iteration 4: Part 2 Appendices
**Source:** `OCPP-2.1_edition2_part2_appendices_v21.pdf`
**Output:** `ocpp2.1-doc/part2-appendices/`
**Tasks:**
- Extract component definitions
- Extract variable definitions
- Document device model hierarchy

#### Iteration 5: Part 3 - JSON Schemas
**Source:** `OCPP-2.1_part3_JSON_schemas/*.json`
**Output:** `ocpp2.1-doc/part3-schemas/`
**Tasks:**
- Parse each JSON schema
- Generate MD documentation with:
  - Schema purpose
  - Required/optional fields
  - Field types and constraints
  - Example values
- Create overview index file

#### Iteration 6: Part 4 - OCPP-J Specification
**Source:** `OCPP-2.1_edition2_part4_ocpp-j-specification.pdf`
**Output:** `ocpp2.1-doc/part4-ocppj/`
**Tasks:**
- Document WebSocket protocol details
- Extract message flow specifications
- Document security requirements

#### Iteration 7: Part 5 - Certification Profiles
**Source:** `OCPP-2.1_edition2_part5_certification_profiles.pdf`
**Output:** `ocpp2.1-doc/part5-certification/`
**Tasks:**
- Extract certification requirements
- Document test profiles
- Create compliance checklist MD files

#### Iteration 8: Part 6 - Test Cases
**Source:** `OCPP-2.1_edition2_part6-testcases.pdf`
**Output:** `ocpp2.1-doc/part6-testcases/`
**Tasks:**
- Extract all test case definitions
- Document test procedures
- Map test cases to requirements

#### Final Iteration: CSV Appendices + Index Generation
**Source:** `Appendices_CSV_v2.1/*`
**Output:** `ocpp2.1-doc/appendices-csv/` + root index files
**Tasks:**
- Convert CSV files to MD tables
- Create master INDEX.md for entire documentation set
- Generate cross-reference maps
- Final verification of all files

## Markdown File Format

Each generated MD file MUST include:

```markdown
# [Section Title]

## Source Information
- **Source File:** `OCPP-2.1_Edition2_all_files/[filename.pdf]`
- **Chapter/Section:** [Chapter number - Section title]
- **Date:** 2025-12-03
- **Edition:** OCPP 2.1 Edition 2

## Content
[Extracted and formatted content]

## Related Documents
- [Links to related sections]
```

## Tool Requirements

### Enabled Plugins (from .claude/settings.local.json)
- ✅ `superpowers` - For brainstorming and systematic debugging
- ✅ `rust-skills` - For Rust-specific analysis
- ✅ `rust-analyzer-lsp` - For code navigation

### Skills to Use
1. **document-skills:pdf** - PDF splitting and text extraction
2. **ralph-wiggum** - Iterative processing loop
3. **superpowers:brainstorming** - Plan refinement (already in use)

## Ralph Loop Configuration

**Command:**
```bash
/ralph-loop "Execute the current iteration's task as defined in AGENT.md. Process the designated source file(s), extract content following the format requirements, and generate organized Markdown files in ocpp2.1-doc/. When all 9 iterations are complete, output <promise>OCPP_2.1_DOCUMENTATION_COMPLETE</promise>" --completion-promise "OCPP_2.1_DOCUMENTATION_COMPLETE" --max-iterations 50
```

**Iteration Tracking:**
- Maintain `.claude/.ralph-iteration-count.md` to track current iteration number
- Each iteration should:
  1. Read AGENT.md to identify current iteration's task
  2. Process the designated source files
  3. Generate output MD files
  4. Update iteration progress
  5. Commit work with message: `docs: iteration N - [part name]`

## Success Criteria

✅ All 7 PDF files processed and converted to MD
✅ Large PDF (part2) properly split by chapters
✅ All 100+ JSON schemas converted to documentation
✅ CSV appendices converted to MD tables
✅ Each MD file includes source path and chapter reference
✅ Output organized in folder structure matching source parts
✅ Master INDEX.md created with cross-references
✅ All changes committed to git with descriptive messages

## Execution Order

1. **Setup:** Create directory structure
2. **Phase 1:** Split large PDF (part2_specification.pdf)
3. **Phase 2:** Start Ralph Loop with 9 iterations
4. **Verification:** Review generated documentation
5. **Finalization:** Generate master index and cross-references

---

**Created:** 2025-01-25
**Status:** Ready to execute
**Plugin Configuration:** superpowers, rust-skills, rust-analyzer-lsp enabled
