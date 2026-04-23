#!/usr/bin/env python3

import argparse
import json
import sys


def describe():
    description = {
        "name": "write_file",
        "description": "Writes a message to a file",
        "keywords": ["файл"],
        "args_schema": {
            "type": "object",
            "properties": {
                "type": {"type": "string", "const": "write_file"},
                "args": {
                    "type": "object",
                    "properties": {"message": {"type": "string"}},
                    "required": ["message"],
                    "additionalProperties": False,
                },
            },
            "required": ["type", "args"],
            "additionalProperties": False,
        },
    }
    print(json.dumps(description, indent=2))


def run(json_args):
    try:
        data = json.loads(json_args)
    except json.JSONDecodeError:
        print("Invalid JSON")
        sys.exit(1)

    message = data.get("message")
    if message is None:
        print("Missing 'message' field")
        sys.exit(1)

    with open("test.x", "w", encoding="utf-8") as f:
        f.write(message)

    print("File test.x created")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--describe", action="store_true")
    parser.add_argument("--run", type=str)

    args = parser.parse_args()

    if args.describe:
        describe()
    elif args.run:
        run(args.run)
    else:
        print("Use --describe or --run")


if __name__ == "__main__":
    main()
