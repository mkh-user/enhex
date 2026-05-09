"""
Automatic test for all .enhex files inside examples/ folder
Each file must have a `# expect: <regex>` at first line
"""

import os
import sys

# Uses installed version
from enhex import compile


EXAMPLES_DIR = os.path.join(os.path.dirname(__file__), '..', 'examples')


def test_examples():
    """Test all .enhex files"""
    
    if not os.path.isdir(EXAMPLES_DIR):
        print(f"Examples folder not found: {EXAMPLES_DIR}")
        return
    
    enhex_files = [f for f in os.listdir(EXAMPLES_DIR) if f.endswith('.enhex')]
    
    if not enhex_files:
        print("Warn: No .enhex file found!")
        return
    
    passed = 0
    failed = 0
    
    for filename in sorted(enhex_files):
        filepath = os.path.join(EXAMPLES_DIR, filename)
        
        with open(filepath, 'r', encoding='utf-8') as f:
            lines = f.readlines()
        
        if not lines or not lines[0].startswith('# expect:'):
            print(f"Failed: {filename}: First line must start with '# expect:'")
            failed += 1
            continue
        
        expected = lines[0].replace('# expect:', '').strip()
        
        pattern_lines = [line for line in lines[1:] if line.strip() and not line.strip().startswith('#')]
        
        if not pattern_lines:
            print(f"Failed: {filename}: EnhEx code not found")
            failed += 1
            continue
        
        enhx_code = ' '.join(line.strip() for line in pattern_lines)
        
        try:
            result = compile(enhx_code)
        except Exception as e:
            print(f"Failed: {filename}: Compile error: {e}")
            failed += 1
            continue
        
        if result == expected:
            print(f"Passed: {filename}")
            passed += 1
        else:
            print(f"Failed: {filename}")
            print(f"   Expected: {expected}")
            print(f"   Got:      {result}")
            failed += 1
    
    print(f"\n{'='*50}")
    print(f"Total: {passed + failed} | Passed: {passed} | Failed: {failed}")
    
    if failed > 0:
        sys.exit(1)


if __name__ == "__main__":
    test_examples()