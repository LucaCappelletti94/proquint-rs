# Fuzz harness for Proquint

This directory contains an honggfuzz-based fuzzing harness for the Proquint library. The harness is designed to test the Proquint encoding and decoding functions for various edge cases, trying to find bugs or unexpected behavior in the implementation.

## Fuzzing Process

After having installed [`honggfuzz`](https://docs.rs/honggfuzz/latest/honggfuzz/), select one of the harnesses described below and run it with the following command (for instance for the `uuid` harness):

```bash
cargo hfuzz run uuid
```
