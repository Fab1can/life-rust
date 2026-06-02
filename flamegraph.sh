#!/bin/bash

export RUSTFLAGS="-C debuginfo=2 -C force-frame-pointers=yes"
CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph --release --output flamegraph-$(date +%Y-%m-%d-%H-%M-%S).svg -- --no-buildid-mmap 