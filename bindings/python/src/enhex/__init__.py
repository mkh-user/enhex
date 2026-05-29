"""
EnhEx — Enhanced Expression.
Regex, Enhanced for Readability.
"""
from enhex._enhex import compile as _native_compile

__version__ = "0.3.0"

def compile(pattern: str) -> str:
    return _native_compile(pattern)

def compile_file(path: str) -> str:
    with open(path, 'r', encoding='utf-8') as f:
        return compile(f.read())
