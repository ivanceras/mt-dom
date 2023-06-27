#!/bin/bash
set -v

RUST_LOG=trace cargo test --no-fail-fast
