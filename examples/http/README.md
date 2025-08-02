# HTTP

A simple program that makes a GET request and prints the response status.

This example demonstrates:
- Making HTTP requests from within a Bubble Tea application
- Using native Rust HTTP client (reqwest) with timeout
- Handling async operations with commands
- Auto-quitting after completing the main task

The program automatically starts an HTTP request to `https://charm.sh/` when launched, displays the status code (e.g., "200 OK"), and then exits. You can press 'q' or Esc to quit early if needed.