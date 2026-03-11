# WebSocket Echo Server — QA Engineering Assessment

## Overview

This repository contains a WebSocket echo server written in Rust. The server accepts WebSocket connections on port `8080`. When a client sends a text message, the server processes it through an internal handler and responds with the message prefixed by `echo: `.

## Getting Started

### Prerequisites

- Docker and Docker Compose

### Running the Server

```bash
git clone <repo-url>
cd assessment
docker compose up --build
```

The server will be available at `ws://localhost:8080`.

### Quick Smoke Test

You can verify the server is running with a quick manual test using any WebSocket client:

```bash
# Using websocat (cargo install websocat)
websocat ws://localhost:8080
> hello
echo: hello
```

## Your Task

Write a **test harness in Python** that evaluates the WebSocket echo server for correctness, performance, and reliability.

Your harness should:

1. **Connect** to the WebSocket server and verify the connection is established.
2. **Send messages** and validate that responses are correct.
3. **Measure performance** — assess response latency under various conditions.
4. **Test concurrency** — evaluate how the server behaves under concurrent load (multiple messages, multiple connections).
5. **Document findings** — produce a short report summarizing the server's behavior, any bugs or issues discovered, and their severity.

### Requirements

- Use Python and [uv](https://docs.astral.sh/uv/) for project and dependency management.
- You may use any libraries you see fit (e.g., `websockets`, `asyncio`, `pytest`).
- Your harness should be runnable with a single command (e.g., `uv run pytest`).

### What We're Looking For

- **Thoroughness**: Do your tests cover edge cases, not just the happy path?
- **Correctness validation**: Are you verifying that every response matches what was sent, or just that a response was received?
- **Performance awareness**: Can your harness identify bottlenecks or unexpected latency patterns?
- **Clear reporting**: When an issue is found, is it described in a way that a developer could reproduce and fix it?

## Deliverables

1. A Python test harness (source code + `requirements.txt`).
2. A short written report (`REPORT.md`) with:
   - A summary of your test approach.
   - Any bugs or issues found, with evidence.
   - Severity classification for each finding.
   - A performance analysis covering throughput, latency distribution, and any bottlenecks identified under load. Consider how latency behaves when a single client sends multiple messages in quick succession versus when messages are sent one at a time.
   - Recommendations.

## Time Expectation

This assessment is designed to be completed in **2–3 hours**. Focus on quality of findings over quantity of code.
