# GYLD Solidity → Anchor Conversion Brainstorm

**Date:** 2026-02-03
**Status:** Active Discussion
**Related Plan:** `docs/plans/2026-02-02-feat-solidity-dev-learning-path-cookbook-plan.md`

---

## What We're Building

Convert GYLD Foundry Solidity contracts to Anchor/Rust for Solana. Starting with **AccessRegistry.sol** as a proof-of-concept, then systematically converting remaining contracts.

**Why This Project:**
- Educational: Learn Solana patterns by converting production Solidity code
- Reference: Create a side-by-side comparison for Solidity developers
- Real Protocol: GYLD is a live DeFi protocol (tokenized ETH staking vaults)

---

## Phase 1: AccessRegistry Conversion (First Contract)

### Contract Overview

**AccessRegistry.sol** (~297 lines) - Centralized blacklist registry

**Solidity Features:**
1. **Internal blacklist** - `mapping(address => bool)` owner-controlled
2. **Chainalysis Oracle** - External call to check OFAC SDN list
3. **Owner auto-approval** - Bypass for AccessRegistry owner + PoolFactory owner
4. **Batch operations** - `setBlacklistedBatch()` up to 100 addresses
5. **Fail-closed design** - Oracle call failures revert transaction

**Functions to Convert:**
| Solidity Function | Anchor Equivalent |
|-------------------|-------------------|
| `constructor(chainalysisOracle, initialBlacklist)` | `initialize()` |
| `setBlacklisted(account, blacklisted)` | `set_blacklisted()` |
| `setBlacklistedBatch(accounts[], blacklisted)` | `set_blacklisted_batch()` |
| `isApproved(account) → bool` | `is_approved()` |
| `isBlacklisted(account) → bool` | `is_blacklisted()` |
| `isSanctionedByChainalysis(account) → bool` | `is_sanctioned_by_chainalysis()` |
| `getApprovedBatch(accounts[]) → bool[]` | `get_approved_batch()` |

### Key Architecture Decisions

**1. Storage Model: PDA-based Account**

```rust
// Anchor Program Account (PDA)
#[account]
pub struct AccessRegistry {
    pub owner: Pubkey,           // 32 bytes
    pub chainalysis_oracle: Pubkey, // 32 bytes
    pub pool_factory: Pubkey,    // 32 bytes
    pub blacklist_count: u32,    // 4 bytes
    pub bump: u8,                // 1 byte
    // Blacklist stored in separate accounts for rent efficiency
}
```

**Why PDAs:**
- Deterministic address via seeds
- Upgradeable via BPFLoaderUpgradeable
- Rent-exemption with SOL

**2. Blacklist Storage Pattern**

*Option A: Single Account with HashMap* ❌
```rust
pub struct BlacklistAccount {
    pub entries: HashMap<Pubkey, bool>, // Not anchor serializable
}
```

*Option B: Separate PDA per Blacklisted Address* ✅
```rust
#[account]
pub struct BlacklistEntry {
    pub account: Pubkey,
    pub blacklisted: bool,
    pub timestamp: i64,
    pub reason: String, // Optional
    pub bump: u8,
}
// Seeds: ["blacklist", account.key().as_ref()]
```

**Decision: Option B (Separate PDAs)**
- Rent-efficient (only pay for blacklisted addresses)
- Easy iteration with `get_program_account`
- Matches Solana's account model

**3. Chainalysis Oracle Integration**

Solana has no direct "oracle" like Ethereum's contract calls. Two approaches:

*Option A: CPI to Solana Oracle Program* ✅
```rust
// Mock oracle program for testing
pub fn is_sanctioned(ctx: Context<CheckSanctioned>, account: Pubkey) -> Result<bool> {
    // CPI to chainalysis_oracle program
    chainalysis::is_sanctioned(CpiContext::new(ctx.accounts.chainalysis_program, ...))
}
```

*Option B: Off-chain with Sysvar*
- Use `Clock` sysvar for timestamp
- Maintainer updates oracle data periodically

**Decision: Option A (CPI to Mock Oracle)**
- Matches Solidity's on-chain call pattern
- For testing: deploy mock oracle program
- For production: real Chainalysis could build Solana program

**4. Upgradeability**

```toml
# Anchor.toml
[programs.localnet]
access_registry = "AcceR3g1sTry..."

[tool.upgrade]
# Add upgrade authority management
```

Use `solana-program-upgrade` or delegate to upgradeable loader.

---

## Phase 2: Conversion Roadmap

### Contract Order (Simple → Complex)

| Phase | Contract | Complexity | Key Learning Outcomes |
|-------|----------|------------|----------------------|
| **1** | AccessRegistry.sol | Low-Medium | PDAs, external CPI, account validation |
| **2** | StripTokenUpgradeable.sol | High | SPL Token extensions, role systems, compliance |
| **3** | Vault3Upgradeable.sol | High | Token mint/burn CPIs, maturity logic, batch ops |
| **4** | PoolFactoryUpgradeable.sol | Very High | Factory pattern, proxy deployments, batch upgrades |

### Success Criteria for Phase 1

1. **All tests pass**: `anchor test` on localnet
2. **Feature parity**: All AccessRegistry functions work identically
3. **Test coverage**: ≥90% line coverage (matching Foundry tests)
4. **Gas/CU efficiency**: Reasonable compute unit usage
5. **Documentation**: Inline comments explaining Solana patterns

---

## Open Questions

**Q1: Chainalysis Oracle on Solana?**
- Chainalysis doesn't currently have a Solana program
- For PoC: Deploy mock oracle that returns hardcoded sanctions list
- For production: Would need Chainalysis to build Solana integration

**Q2: Blacklist Enumeration?**
- Solidity: Private mapping (cannot enumerate)
- Solana: Can use `get_program_account` to find all blacklist PDAs
- Should we add enumeration function for convenience?

**Q3: Batch Size Limits?**
- Solana transaction size limits (~1232 bytes)
- Anchor account limits (max 10 accounts per transaction by default)
- Current: MAX_BATCH_SIZE = 100 in Solidity
- Propose: Reduce to 20-30 for Solana

**Q4: Event Logging?**
- Solidity: `emit AccessUpdated(account, approved)`
- Solana: `msg!()` macro for logging
- Should we emit both for indexer compatibility?

---

## Next Steps

**Immediate:**
1. ✅ Create this brainstorm document
2. ⏭️ Create implementation plan via `/workflows:plan`
3. ⏭️ Set up project structure for AccessRegistry
4. ⏭️ Write first Anchor test (initialize)

**After AccessRegistry Success:**
5. Document lessons learned in learning path
6. Create comparison guide: AccessRegistry (Solidity vs Anchor)
7. Begin StripTokenUpgradeable conversion

---

## Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Chainalysis has no Solana oracle | Medium | Use mock oracle for tests, document production gap |
| Account limits reduce batch size | Low | Implement multi-transaction batching for large batches |
| Rent costs higher than Ethereum gas | Medium | Document rent-exemption requirements, consider rent rebates |
| Time/resource constraints | Medium | Start with minimal viable conversion, add features incrementally |

---

## References

- **Source:** `/mnt/d/GYLD/Foundry/gyld-contracts/src/contracts/AccessRegistry.sol`
- **Tests:** `/mnt/d/GYLD/Foundry/gyld-contracts/test/VaultUpgradeableTest.t.sol` (AccessRegistry tests)
- **Learning Path:** `docs/plans/2026-02-02-feat-solidity-dev-learning-path-cookbook-plan.md`
- **Anchor Docs:** https://www.anchor-lang.com/
