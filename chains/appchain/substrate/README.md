# Secure IoT Ledger – Substrate Sovereign Chain (Devnet)

This is a minimal FRAME workspace with stubs for the runtime and pallets:
- devices, firmware, vulns, scoring, bounties, certifications, anchors

> Goal: compile soon with standard pallets added; pallets here are skeletons to iterate quickly.

## Build
- Install Rust (nightly + wasm32 target) and Substrate deps.
- `cargo build -p node`

## Next steps
- Wire standard pallets (system, timestamp, balances, sudo) into `runtime/src/lib.rs`.
- Implement storage/events/extrinsics in each custom pallet.
- Add benchmarking features & weights before production.