#!/bin/bash
set -v

RUST_LOG=trace cargo +stable test --features "with-measure"
