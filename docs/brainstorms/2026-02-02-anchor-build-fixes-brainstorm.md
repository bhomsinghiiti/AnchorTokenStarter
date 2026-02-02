# Anchor Build Fixes & SPL Token Implementation

**Date:** 2026-02-02
**Status:** Brainstormed, ready for planning

---

## What We're Building

Fix existing issues in the Solana/Anchor project and add SPL token functionality to create a functional token program that can be deployed and tested.

## Problems Identified

### 1. Program ID Mismatch (Critical)
- **lib.rs:** `6K9YF1gjvNCZYCmHhKwuAtZ5tDMztrGTjNWAsbFttXpC` (Hh)
- **Anchor.toml:** `6K9YF1gjvNCZYCmMhKwuAtZ5tDMztrGTjNWAsbFttXpC` (Mm)
- **Impact:** Deployment failures, confusion between source and config

### 2. Rust Toolchain Version Inconsistency
- **rust-toolchain.toml:** specifies 1.79.0
- **CLAUDE.md:** documents 1.89.0 (incorrect)
- **Actual installed:** 1.86.0-nightly
- **Impact:** Documentation is misleading, potential compatibility issues

### 3. Missing SPL Token Implementation
- **Current state:** Only basic `initialize` and `increment` instructions
- **package.json has:** `@solana/spl-token` dependency
- **Cargo.toml missing:** SPL token Rust dependencies
- **Impact:** Can't build SPL token functionality despite having TS dependencies

---

## Key Decisions

### Decision 1: Program ID Source of Truth
Use the program ID from `target/deploy/my_first_project-keypair.json` as the single source of truth. Update both `lib.rs` and `Anchor.toml` to match this generated keypair.

**Rationale:** Anchor generates this keypair during first build; it should be the canonical ID.

### Decision 2: Rust Toolchain Strategy
Pin to stable 1.79.0 (as specified in rust-toolchain.toml) for consistency. This is compatible with Anchor 0.32.1.

**Rationale:** Nightly versions (currently 1.86.0-nightly) can introduce instability. Pin to a known working stable version.

### Decision 3: SPL Token Implementation Approach
Add minimal SPL token functionality:
1. Add `anchor-spl` dependency to Cargo.toml
2. Create a `create_token` instruction that mints a new token
3. Keep it simple - no complex transfer/burn logic initially

**Rationale:** YAGNI principle - start with basic token creation, expand later if needed.

---

## Open Questions

1. **Token parameters:** Should the token supply be fixed at mint time, or allow minting more later?
2. **Token authority:** Who should be the mint authority? The program or the signer?
3. **Testing:** Do we want to test against localnet only, or also devnet?

---

## Files to Modify

| File | Change |
|------|--------|
| `programs/my-first-project/Cargo.toml` | Add `anchor-spl` dependency |
| `programs/my-first-project/src/lib.rs` | Add SPL token instructions, fix program ID |
| `Anchor.toml` | Fix program ID mismatch |
| `rust-toolchain.toml` | Verify/stabilize version |
| `CLAUDE.md` | Update Rust version documentation |
| `tests/my-first-project.ts` | Add SPL token tests |
