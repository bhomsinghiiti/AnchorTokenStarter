---
title: Fix Anchor Program ID Mismatch and Verify Build
type: fix
date: 2026-02-02
---

## Enhancement Summary

**Deepened on:** 2026-02-02
**Sections enhanced:** 6
**Research agents used:** Security, Architecture, Simplicity, Performance, Deployment Verification, Pattern Recognition, Web Research

### Key Improvements
1. **CRITICAL:** Added keypair file permissions security fix (chmod 600)
2. **CRITICAL:** Added Rust toolchain prerequisite step (1.79.0 stable)
3. **IMPROVED:** Simpler approach using `anchor keys list` instead of manual keypair reading
4. **NEW:** Pre-deployment verification checklist
5. **NEW:** Program ID verification script for future prevention
6. **NEW:** Added cluster-specific configuration guidance

### New Considerations Discovered
- **4 different program IDs** found (not 3) - includes CLAUDE.md with typo
- **Keypair file is world-readable** (644 permissions) - security vulnerability
- **Rust toolchain mismatch** blocks build before program ID can be fixed
- **CI/CD keypair management** is a known Anchor issue (#3985)
- **Missing .so file** indicates program has never successfully built

---

# Fix Anchor Program ID Mismatch and Verify Build

## Overview

Fix a critical program ID mismatch across **four** configuration files that prevents the Anchor program from building and deploying correctly. Verify that the existing SPL token functionality works after the fix.

## Problem Statement

The codebase has **four different program IDs** across different files, causing build failures and deployment confusion:

| Location | Current ID | Line |
|----------|------------|------|
| `Anchor.toml` | `6K9YF1gjvNCZYCmMhKwuAtZ5tDMztrGTjNWAsbFttXpC` | 9 |
| `lib.rs` | `BkxesYkTpnK1Xev6PWcgf87ERappKnRPFxTaPw5E8xrU` | 7 |
| `CLAUDE.md` | `6K9YF1gjvNCZYCmhKwuAtZ5tDMztrGTjNWAsbFttXpC` | 9 (note typo: "mh" vs "Mh") |
| Actual keypair | `9UimV9m3EwgA5RdtQrgEiHeAPHr3gvSmrb1UJ8CupzLJ` | from `anchor keys list` |

**Impact:**
- `anchor build` may fail or use wrong program
- `anchor deploy` will deploy to wrong address
- Tests cannot find the correct program
- Deployment confusion between localnet/devnet/mainnet
- **SECURITY:** Keypair file has 644 permissions (world-readable)

### Good News

The SPL token functionality is **already fully implemented** in the codebase:
- `create_mint` instruction (`lib.rs:29-33`)
- `mint_tokens` instruction (`lib.rs:35-51`)
- `transfer_tokens` instruction (`lib.rs:53-70`)
- Complete test coverage (`tests/my-first-project.ts:70-183`)
- Dependencies already configured (`Cargo.toml:25`)

The brainstorm was based on outdated information - only the program ID needs fixing.

---

## Proposed Solution

Use the generated keypair from `target/deploy/my_first_project-keypair.json` as the single source of truth. Update all files to match this program ID using Anchor's built-in tooling.

### Research Insights

**Best Practices:**
- Use `anchor keys list` to display program IDs (simpler than reading keypair file)
- Use `anchor keys sync` to automatically synchronize IDs (Anchor 0.30+)
- Keypair file is the canonical source - follow cryptographic primacy pattern

**Security Considerations:**
- **CRITICAL:** Set keypair file permissions to 600 (owner read-only)
- Never commit keypair files to git (properly in .gitignore)
- Verify cluster identity before deployment to prevent wrong-network deployments

**Implementation Details:**
```bash
# Simpler approach using Anchor CLI
anchor keys list                    # Shows all program IDs
anchor keys sync                    # Auto-syncs IDs (if available)

# Get program ID from keypair (one-liner)
anchor keys list | grep my_first_project
```

**Edge Cases:**
- If program is already deployed to a cluster, changing ID will orphan all state
- CI/CD pipelines generate new keypairs each build (#3985) - need persistent keypair
- Missing `.so` file indicates build never succeeded - may need to fix toolchain first

**References:**
- [Anchor GitHub Issue #3985](https://github.com/solana-foundation/anchor/issues/3985) - Key sync before building
- [Solana StackExchange](https://solana.stackexchange.com/questions/17031) - Best way to generate program ID
- [Anchor Installation Guide](https://www.anchor-lang.com/docs/installation) - Toolchain setup

---

## Prerequisites

### Research Insights

**Performance Considerations:**
- Rust 1.79.0 stable is required for Anchor 0.32.1 compatibility
- Nightly versions (1.86.0) cause inconsistent compilation times
- Using exact toolchain version ensures reproducible builds

**Security Considerations:**
- Verify no existing deployments before changing program ID
- Backup keypair before any modifications
- Check wallet has funds for deployment fees

**Implementation Details:**
```bash
# Install correct Rust version
rustup install 1.79.0
rustup override set 1.79.0
rustc --version  # Should show 1.79.0

# Backup existing keypair
cp target/deploy/my_first_project-keypair.json ~/backup-$(date +%Y%m%d)-keypair.json
```

**Edge Cases:**
- If program is already deployed, use deployed ID instead of generating new one
- Check if upgrade authority is owned by current wallet
- Verify which cluster(s) program is deployed to

**References:**
- [Anchor 0.32.0 Release Notes](https://www.anchor-lang.com/docs/updates/release-notes/0-32-0) - Rust compatibility
- [Solana Deployment Guide](https://solana.com/docs/intro/quick-start/deploying-programs) - Deployment best practices

---

## Implementation Steps

### Phase 1: Pre-Flight Checks

### Research Insights

**Best Practices:**
- Verify current build state before making changes
- Check for existing deployments to avoid orphaning state
- Document all current program IDs for rollback

**Performance Considerations:**
- Check disk space for build artifacts (target/ can be >1GB)
- Verify CPU cores for parallel compilation

**Implementation Details:**
```bash
# 1. Verify Rust toolchain
rustc --version  # Should be 1.79.0

# 2. Check current program IDs
anchor keys list

# 3. Check if program is deployed anywhere
solana program show $(anchor keys list | grep my_first_project | awk '{print $2}') --url localhost 2>&1
solana program show $(anchor keys list | grep my_first_project | awk '{print $2}') --url devnet 2>&1

# 4. Verify wallet has funds
solana balance --url devnet

# 5. Document current IDs before changes
grep -r "declare_id!" programs/
grep "my_first_project =" Anchor.toml
```

**Edge Cases:**
- If program already deployed, ABORT and use deployed ID
- If wallet has no funds, request SOL airdrop for devnet
- If .so file doesn't exist, first build will take longer

---

### Phase 2: Fix Rust Toolchain (if needed)

### Research Insights

**Performance Considerations:**
- Rust 1.79.0 stable ensures consistent compilation times
- Nightly versions have unpredictable performance and compatibility

**Implementation Details:**
```bash
# Update rust-toolchain.toml to 1.79.0
cat > rust-toolchain.toml << 'EOF'
[toolchain]
channel = "1.79.0"
components = ["rustfmt", "clippy"]
profile = "minimal"
EOF

# Verify installation
rustup install 1.79.0
rustup override set 1.79.0
rustc --version  # Should show 1.79.0
```

**References:**
- [Anchor Installation](https://www.anchor-lang.com/docs/installation) - Rust version requirements

---

### Phase 3: Generate and Extract Program ID

### Research Insights

**Simplification:** Use `anchor keys list` instead of manually parsing keypair JSON.

**Best Practices:**
- Let Anchor generate keypair on first build
- Use `anchor keys sync` if available (Anchor 0.30+)

**Implementation Details:**
```bash
# Build to generate keypair (if not exists)
anchor build

# Extract program ID using Anchor CLI
PROGRAM_ID=$(anchor keys list | grep my_first_project | awk '{print $2}')
echo "Program ID: $PROGRAM_ID"

# Alternative: Use anchor keys sync (if available)
anchor keys sync
```

**Edge Cases:**
- If `anchor build` fails, fix Rust toolchain first
- If keypair already exists, decide whether to reuse or regenerate
- Regenerating creates NEW program ID - only if never deployed

---

### Phase 4: Update All Files (Atomic Commit)

### Research Insights

**Best Practices:**
- Update all files in ONE commit (atomic change)
- Use the extracted program ID from Phase 3
- Add verification comments to prevent future drift

**Security Considerations:**
- Set keypair file permissions to 600 after update
- Verify no hardcoded old IDs remain in codebase

**Implementation Details:**

**Update lib.rs:**
```rust
// programs/my-first-project/src/lib.rs:7
declare_id!("PROGRAM_ID_FROM_ABOVE");
```

**Update Anchor.toml:**
```toml
[programs.localnet]
# This MUST match declare_id! in programs/my-first-project/src/lib.rs
my_first_project = "PROGRAM_ID_FROM_ABOVE"

[programs.devnet]
my_first_project = "PROGRAM_ID_FROM_ABOVE"

[programs.mainnet]
my_first_project = "PRODUCTION_ID_WHEN_READY"
```

**Update CLAUDE.md:**
```markdown
**Program ID**: Run `anchor keys list` to see the current program ID
**Rust Toolchain**: 1.79.0 stable (see rust-toolchain.toml)
```

**Set keypair permissions:**
```bash
chmod 600 target/deploy/my_first_project-keypair.json
```

**Verify no old IDs remain:**
```bash
grep -r "OLD_PROGRAM_ID" . --exclude-dir=target
# Should return nothing
```

---

### Phase 5: Verification

### Research Insights

**Best Practices:**
- Build, test, verify in sequence
- Use `anchor keys list` to confirm ID consistency
- Run tests to ensure SPL functionality works

**Performance Considerations:**
- First build after toolchain change may be slower (rebuilding dependencies)
- Subsequent builds will use incremental compilation

**Implementation Details:**
```bash
# 1. Build verification
anchor build
ls -lh target/deploy/my_first_project.so  # Should exist and be >50KB

# 2. ID consistency check
LIB_RS_ID=$(grep "declare_id!" programs/my-first-project/src/lib.rs | grep -oP '(?<=")[^"]+')
ANCHOR_ID=$(grep "my_first_project =" Anchor.toml | grep -oP '(?<=")[^"]+')
KEYPAIR_ID=$(anchor keys list | grep my_first_project | awk '{print $2}')

[ "$LIB_RS_ID" = "$ANCHOR_ID" ] && [ "$ANCHOR_ID" = "$KEYPAIR_ID" ] && echo "✓ IDs match" || echo "✗ MISMATCH"

# 3. Deploy to localnet and verify
anchor deploy --provider.cluster localnet
solana program show "$KEYPAIR_ID" --url localhost

# 4. Run tests
anchor test --skip-local-validator
```

**References:**
- [Anchor Build/Deploy/Test Cycle](https://solana.stackexchange.com/questions/21364) - Best practices

---

## Future Prevention

### Research Insights

**Best Practices:**
- Add pre-commit hook to verify program ID consistency
- Add CI check for program ID synchronization
- Document program ID lifecycle in CLAUDE.md

**Implementation Details:**

**Add verification script** (`scripts/verify-program-id.sh`):
```bash
#!/bin/bash
set -euo pipefail

LIB_ID=$(grep -oP 'declare_id!\("\K[^"]+' programs/my-first-project/src/lib.rs)
TOML_ID=$(grep -A2 '\[programs\.localnet\]' Anchor.toml | grep -oP 'my_first_project = "\K[^"]+')
KEYPAIR_ID=$(anchor keys list | grep my_first_project | awk '{print $2}')

echo "Program ID Verification:"
echo "  Keypair:     $KEYPAIR_ID"
echo "  lib.rs:      $LIB_ID"
echo "  Anchor.toml: $TOML_ID"

if [[ "$KEYPAIR_ID" != "$LIB_ID" ]] || [[ "$KEYPAIR_ID" != "$TOML_ID" ]]; then
    echo "✗ FAIL: Program ID mismatch detected"
    exit 1
fi

echo "✓ PASS: All program IDs synchronized"
```

**Add to package.json:**
```json
{
  "scripts": {
    "verify:program-id": "bash scripts/verify-program-id.sh",
    "prebuild": "yarn run verify:program-id",
    "pretest": "yarn run verify:program-id"
  }
}
```

**Add to CLAUDE.md:**
```markdown
## Program ID Management

### Sources (Priority Order)
1. **Generated Keypair**: `target/deploy/my_first_project-keypair.json` (authoritative)
2. **Runtime Declaration**: `declare_id!` in `lib.rs` (must match keypair)
3. **Test Configuration**: `Anchor.toml` (must match keypair)
4. **Documentation**: `CLAUDE.md` (informational only)

### Verification
Always run `yarn run verify:program-id` before deployment.
```

---

## Acceptance Criteria

- [ ] Program ID matches in `lib.rs`, `Anchor.toml`, and keypair file
- [ ] `anchor build` completes without errors
- [ ] `anchor test` passes all tests (including SPL token tests)
- [ ] `CLAUDE.md` documents correct Rust version (1.79.0) and program ID reference
- [ ] Keypair file has `600` permissions (owner read-only)
- [ ] Verification script passes (`yarn run verify:program-id`)
- [ ] No references to old program IDs remain in codebase

---

## Files to Modify

| File | Change |
|------|--------|
| `rust-toolchain.toml` | Verify/ensure version is 1.79.0 |
| `programs/my-first-project/src/lib.rs` | Update `declare_id!` on line 7 |
| `Anchor.toml` | Update program ID on line 9, add devnet/mainnet sections |
| `CLAUDE.md` | Update Rust version and program ID reference |
| `scripts/verify-program-id.sh` | Create verification script (new) |
| `package.json` | Add verification script to prebuild/pretest hooks |
| `target/deploy/my_first_project-keypair.json` | Set permissions to 600 |

---

## Security Checklist

- [ ] Keypair file permissions set to 600
- [ ] Keypair backed up before modifications
- [ ] No existing deployments that would be orphaned
- [ ] Wallet identity verified before deployment
- [ ] Cluster identity verified (localnet vs devnet vs mainnet)
- [ ] `.gitignore` properly excludes `target/deploy/*.json`

---

## References

- Brainstorm: `docs/brainstorms/2026-02-02-anchor-build-fixes-brainstorm.md`
- Anchor docs: https://www.anchor-lang.com/docs/program-space
- Program ID patterns: `Anchor.toml:9`, `lib.rs:7`
- Key sync issue: https://github.com/solana-foundation/anchor/issues/3985
- Program security: https://medium.com/@rkmonarch/solana-program-security-best-practices-to-prevent-exploits-f88b4a427bce
- Deployment best practices: https://solana.stackexchange.com/questions/13875
- Anchor 0.32.1 compatibility: https://www.anchor-lang.com/docs/updates/release-notes/0-32-1
