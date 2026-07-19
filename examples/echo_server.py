#!/usr/bin/env python3
"""A minimal stdio MCP server used for trying Astray Verify locally."""

import json
import sys


for line in sys.stdin:
    message = json.loads(line)
    if message.get("id") == 1:
        print(
            json.dumps(
                {
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": {
                        "protocolVersion": "2025-06-18",
                        "capabilities": {},
                        "serverInfo": {"name": "echo-demo", "version": "1.0.0"},
                    },
                }
            ),
            flush=True,
        )
    elif message.get("id") == 2:
        print(
            json.dumps(
                {
                    "jsonrpc": "2.0",
                    "id": 2,
                    "result": {
                        "tools": [
                            {
                                "name": "echo",
                                "description": "Returns the provided text.",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {"text": {"type": "string"}},
                                    "required": ["text"],
                                },
                            }
                        ]
                    },
                }
            ),
            flush=True,
        )
