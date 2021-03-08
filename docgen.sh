#!/bin/bash
RUSTDOCFLAGS="--html-in-header katex_support.html" cargo doc --no-deps
