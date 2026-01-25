#!/usr/bin/env python3
"""
Extract text from OCPP 2.1 Part 0 Introduction PDF and convert to Markdown.
Improved version with better section detection.
"""
import sys
from pathlib import Path
from pypdf import PdfReader
import re

def extract_text_from_pdf(pdf_path):
    """Extract all text from PDF with page numbers."""
    reader = PdfReader(pdf_path)
    total_pages = len(reader.pages)

    content = []
    for i, page in enumerate(reader.pages, 1):
        text = page.extract_text()
        content.append({
            'page': i,
            'text': text
        })

    return content, total_pages

def create_simple_markdown(content, output_dir, source_pdf):
    """Create a simple markdown structure by pages/chunks."""
    output_path = Path(output_dir)
    output_path.mkdir(parents=True, exist_ok=True)

    files_created = []

    # For Part 0, let's create logical sections based on typical intro document structure
    # We'll create files for major content blocks

    all_text = ""
    for item in content:
        all_text += f"\n\n--- Page {item['page']} ---\n\n"
        all_text += item['text']

    # Create a comprehensive single file for Part 0 (it's only 17 pages)
    filename = "00-introduction-complete.md"

    md_content = f"""# OCPP 2.1 Edition 2 - Part 0: Introduction

## Source Information
- **Source File:** `{source_pdf}`
- **Date:** 2025-12-03
- **Edition:** OCPP 2.1 Edition 2
- **Total Pages:** 17

## Complete Content

{all_text}

## Related Documents
- [Back to Main Index](../INDEX.md)
"""

    file_path = output_path / filename
    with open(file_path, 'w', encoding='utf-8') as f:
        f.write(md_content)

    files_created.append(str(file_path))
    print(f"Created: {filename}")

    return files_created, all_text

def main():
    pdf_path = "OCPP-2.1_Edition2_all_files/OCPP-2.1_edition2_part0_introduction.pdf"
    output_dir = "ocpp2.1-doc/part0-introduction"

    print(f"Processing: {pdf_path}")
    print(f"Output directory: {output_dir}")
    print("-" * 60)

    # Extract text
    print("Extracting text from PDF...")
    content, total_pages = extract_text_from_pdf(pdf_path)
    print(f"Total pages: {total_pages}")

    # Create markdown files
    print("\nCreating Markdown files...")
    files, text = create_simple_markdown(content, output_dir, pdf_path)

    print(f"\n✓ Complete! Created {len(files)} Markdown files")

    # Create index
    index_path = Path(output_dir) / "INDEX.md"
    with open(index_path, 'w', encoding='utf-8') as f:
        f.write("# OCPP 2.1 Edition 2 - Part 0: Introduction\n\n")
        f.write("## Source Information\n")
        f.write(f"- **Source File:** `{pdf_path}`\n")
        f.write("- **Date:** 2025-12-03\n")
        f.write("- **Edition:** OCPP 2.1 Edition 2\n")
        f.write(f"- **Total Pages:** {total_pages}\n\n")
        f.write("## Documents\n\n")
        f.write("1. [Complete Introduction Document](00-introduction-complete.md)\n")

    print(f"Created: INDEX.md")

    # Show preview
    print("\n" + "="*60)
    print("PREVIEW (first 1000 chars):")
    print("="*60)
    print(text[:1000])
    print("="*60)

if __name__ == "__main__":
    main()
