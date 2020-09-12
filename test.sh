#!/bin/bash
set -v

RUST_LOG=trace cargo test --features "with-measure"
