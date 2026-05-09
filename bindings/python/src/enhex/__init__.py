"""
EnhEx — Enhanced Expression.
Regex, Enhanced for Readability.
"""

from enhex._enhex import compile as _compile_native # pyright: ignore[reportMissingImports]

def compile(pattern: str) -> str:
    return _compile_native(pattern)

def compile_file(path: str) -> str:
    with open(path, 'r', encoding='utf-8') as f:
        return compile(f.read())

__version__ = "0.2.0"
