#!/usr/bin/env python3
"""
Extract text from OCPP 2.1 Part 1 Architecture & Topology PDF.
"""
from pathlib import Path
from pypdf import PdfReader

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

def create_architecture_markdown(content, output_dir, source_pdf, total_pages):
    """Create markdown for Part 1."""
    output_path = Path(output_dir)
    output_path.mkdir(parents=True, exist_ok=True)

    # Combine all text
    all_text = ""
    for item in content:
        all_text += f"\n\n--- Page {item['page']} ---\n\n"
        all_text += item['text']

    # Create comprehensive file
    filename = "00-architecture-topology-complete.md"

    md_content = f"""# OCPP 2.1 Edition 2 - Part 1: Architecture & Topology

## Source Information
- **Source File:** `{source_pdf}`
- **Date:** 2025-12-03
- **Edition:** OCPP 2.1 Edition 2
- **Total Pages:** {total_pages}

## Complete Content

{all_text}

## Related Documents
- [Part 0: Introduction](../part0-introduction/INDEX.md)
- [Back to Main Index](../INDEX.md)
"""

    file_path = output_path / filename
    with open(file_path, 'w', encoding='utf-8') as f:
        f.write(md_content)

    print(f"Created: {filename}")
    return str(file_path)

def main():
    pdf_path = "OCPP-2.1_Edition2_all_files/OCPP-2.1_edition2_part1_architecture_topology.pdf"
    output_dir = "ocpp2.1-doc/part1-architecture"

    print(f"Processing: {pdf_path}")
    print(f"Output directory: {output_dir}")
    print("-" * 60)

    # Extract text
    print("Extracting text from PDF...")
    content, total_pages = extract_text_from_pdf(pdf_path)
    print(f"Total pages: {total_pages}")

    # Create markdown
    print("\nCreating Markdown file...")
    create_architecture_markdown(content, output_dir, pdf_path, total_pages)

    print(f"\n✓ Complete!")

    # Create index
    index_path = Path(output_dir) / "INDEX.md"
    with open(index_path, 'w', encoding='utf-8') as f:
        f.write("# OCPP 2.1 Edition 2 - Part 1: Architecture & Topology\n\n")
        f.write("## Source Information\n")
        f.write(f"- **Source File:** `{pdf_path}`\n")
        f.write("- **Date:** 2025-12-03\n")
        f.write("- **Edition:** OCPP 2.1 Edition 2\n")
        f.write(f"- **Total Pages:** {total_pages}\n\n")
        f.write("## Documents\n\n")
        f.write("1. [Complete Architecture & Topology Document](00-architecture-topology-complete.md)\n")

    print(f"Created: INDEX.md")

if __name__ == "__main__":
    main()
