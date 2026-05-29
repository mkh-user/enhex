"""Command-line interface for EnhEx."""

import sys
import argparse
from enhex import compile, compile_file, __version__


def main():
    parser = argparse.ArgumentParser(
        prog="enhex",
        description="EnhEx — Enhanced Expression. Compile readable patterns to Regex."
    )

    subparsers = parser.add_subparsers(dest="command", required=True)

    # enhex compile
    compile_parser = subparsers.add_parser("compile", help="Compile an EnhEx pattern")
    compile_parser.add_argument("input", help="Pattern string or path to .enhex file")

    # enhex version
    subparsers.add_parser("version", help="Show version")

    args = parser.parse_args()

    if args.command == "version":
        print(f"enhex v{__version__}")
        sys.exit(0)

    if args.command == "compile":
        if str(args.input).endswith('.enhex'):
            result = compile_file(args.input)
        else:
            result = compile(args.input)
        print(result)
        sys.exit(0)


if __name__ == "__main__":
    main()
