
"""
Example usage of mdict-rs Python package
"""

import mdict_rs

def main():
    # Example 1: Parse from a file path
    try:
        print("Parsing MDX file...")
        mdx = mdict_rs.parse_mdx_file("resources/mdx/en/朗文当代4.mdx")
        
        print(f"Total entries: {mdx.get_entries_count()}")
        
        # Get the first 5 entries as example
        records = mdx.items()
        print("\nFirst 5 entries:")
        for i, record in enumerate(records[:5]):
            print(f"{i+1}. {record.text}")
            # Print the first 100 characters of definition
            definition = record.definition[:100].replace('\n', ' ')
            print(f"Definition: {definition}...")
            print()
            
    except Exception as e:
        print(f"Error parsing file: {e}")
        print("Make sure you have a valid MDX file in the resources directory")

    # Example 2: Parse from bytes
    try:
        print("\nParsing from bytes...")
        with open("resources/mdx/zh/汉语词典3.mdx", "rb") as f:
            data = f.read()
        
        mdx2 = mdict_rs.parse_mdx_bytes(data)
        print(f"Parsed {mdx2.get_entries_count()} entries from bytes")
        
    except Exception as e:
        print(f"Error parsing from bytes: {e}")

if __name__ == "__main__":
    main()