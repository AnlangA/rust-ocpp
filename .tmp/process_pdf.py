#!/usr/bin/env python3
"""
Extract text from OCPP 2.1 Part 0 Introduction PDF and convert to Markdown.
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

def detect_sections(content):
    """Detect section breaks based on text patterns."""
    sections = []
    current_section = None
    current_content = []

    for item in content:
        text = item['text']

        # Look for section headers (typically numbered like "1.", "2.", etc.)
        lines = text.split('\n')
        for line in lines:
            # Pattern for main sections (1. Title, 2. Title, etc.)
            match = re.match(r'^(\d+)\.\s+(.+)$', line.strip())
            if match and len(line.strip()) < 100:  # Reasonable title length
                section_num = match.group(1)
                section_title = match.group(2)

                # Save previous section
                if current_section:
                    sections.append({
                        'number': section_num,
                        'title': section_title,
                        'content': '\n'.join(current_content)
                    })

                current_section = f"{section_num}. {section_title}"
                current_content = []
            else:
                current_content.append(line)

        current_content.append(f"\n--- Page {item['page']} ---\n")

    # Add last section
    if current_section:
        sections.append({
            'number': 'last',
            'title': current_section,
            'content': '\n'.join(current_content)
        })

    return sections

def create_markdown_files(sections, output_dir, source_pdf):
    """Create individual MD files for each section."""
    output_path = Path(output_dir)
    output_path.mkdir(parents=True, exist_ok=True)

    files_created = []

    for i, section in enumerate(sections, 1):
        # Create filename
        safe_title = section['title'].lower()
        safe_title = re.sub(r'[^\w\s-]', '', safe_title)
        safe_title = re.sub(r'[-\s]+', '-', safe_title)
        filename = f"{i:02d}-{safe_title}.md"

        # Create content
        md_content = f"""# {section['title']}

## Source Information
- **Source File:** `{source_pdf}`
- **Section:** {section['number']} - {section['title']}
- **Date:** 2025-12-03
- **Edition:** OCPP 2.1 Edition 2

## Content

{section['content']}

## Related Documents
- [Back to Index](../INDEX.md)
"""

        # Write file
        file_path = output_path / filename
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(md_content)

        files_created.append(str(file_path))
        print(f"Created: {filename}")

    return files_created

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

    # Detect sections
    print("\nDetecting sections...")
    sections = detect_sections(content)
    print(f"Found {len(sections)} sections")

    # Create markdown files
    print("\nCreating Markdown files...")
    files = create_markdown_files(sections, output_dir, pdf_path)

    print(f"\n✓ Complete! Created {len(files)} Markdown files")

    # Create index
    index_path = Path(output_dir) / "INDEX.md"
    with open(index_path, 'w', encoding='utf-8') as f:
        f.write("# OCPP 2.1 Edition 2 - Part 0: Introduction\n\n")
        f.write("## Source Information\n")
        f.write(f"- **Source File:** `{pdf_path}`\n")
        f.write("- **Date:** 2025-12-03\n")
        f.write("- **Edition:** OCPP 2.1 Edition 2\n\n")
        f.write("## Sections\n\n")

        for i, section in enumerate(sections, 1):
            safe_title = section['title'].lower()
            safe_title = re.sub(r'[^\w\s-]', '', safe_title)
            safe_title = re.sub(r'[-\s]+', '-', safe_title)
            filename = f"{i:02d}-{safe_title}.md"
            f.write(f"{i}. [{section['title']}]({filename})\n")

    print(f"Created: INDEX.md")

if __name__ == "__main__":
    main()
