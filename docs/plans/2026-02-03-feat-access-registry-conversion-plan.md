---
title: Convert AccessRegistry.sol from GYLD Foundry to Anchor/Rust
type: feat
date: 2026-02-03
status: planned
related-brainstorm: ../brainstorms/2026-02-03-gyld-solidity-to-anchor-conversion.md
---

# Convert AccessRegistry.sol from GYLD Foundry to Anchor/Rust

## Overview

Convert GYLD's `AccessRegistry.sol` (~297 lines) from Solidity to Anchor/Rust as the first contract in the Solidity → Solana learning path. This centralized blacklist registry combines internal blacklist management with Chainalysis Oracle integration for OFAC SDN list compliance.

**Why Start Here:**
- Simplest GYLD contract (no token operations, simple state)
- Demonstrates core Solana patterns: PDAs, CPI, account validation
- Clear mapping from Solidity concepts to Anchor patterns
- Production-quality reference for learning

**Success Criteria:**
- [ ] All `anchor test` tests pass on localnet
- [ ] Feature parity with Solidity version (all functions work identically)
- [ ] ≥90% test coverage matching Foundry tests
- [ ] `anchor build` completes without errors
- [ ] Documented patterns for remaining conversions

---

## Problem Statement / Motivation

### Current State (Solidity)

The GYLD protocol uses a centralized blacklist registry deployed on Ethereum:

```solidity
contract AccessRegistry is Ownable {
    mapping(address => bool) private _blacklisted;
    address public chainalysisOracle;
    address public poolFactory;

    function setBlacklisted(address account, bool blacklisted) external onlyOwner
    function isApproved(address account) external view returns (bool)
    // Checks internal blacklist + Chainalysis Oracle
}
```

**Key Features:**
- Internal blacklist (owner-controlled mapping)
- Chainalysis Oracle integration for OFAC compliance
- Auto-approval for registry owner + pool factory owner
- Batch operations (up to 100 addresses)
- Fail-closed design (oracle failures revert transaction)

### Target State (Anchor/Solana)

Convert to Solana with architectural adaptations:

| Solidity Concept | Anchor/Solana Equivalent |
|-----------------|--------------------------|
| `mapping(address => bool)` | Separate PDA accounts per blacklisted address |
| `external view` functions | On-chain instructions + RPC optimization |
| External contract call | Cross-Program Invocation (CPI) |
| `onlyOwner` modifier | Account validation in `#[derive(Accounts)]` |
| Contract storage | `#[account]` struct with PDA |
| UUPS upgradeable | BPFLoaderUpgradeable |
| Event emission | `msg!()` + `emit!()` macros |

---

## Proposed Solution

### Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                  AccessRegistry Program                     │
│                                                              │
│  ┌────────────────────────────────────────────────────┐    │
│  │  AccessRegistry PDA                                │    │
│  │  Seeds: ["access_registry"]                        │    │
│  │  ┌──────────────────────────────────────────────┐ │    │
│  │  │ owner: Pubkey                                 │ │    │
│  │  │ pending_owner: Pubkey                         │ │    │
│  │  │ chainalysis_oracle: Pubkey                    │ │    │
│  │  │ pool_factory_owner: Pubkey                    │ │    │
│  │  │ blacklist_count: u32                          │ │    │
│  │  │ bump: u8                                      │ │    │
│  │  └──────────────────────────────────────────────┘ │    │
│  └────────────────────────────────────────────────────┘    │
│                           │                                  │
│                           │ CPI to                           │
│                           ▼                                  │
│  ┌────────────────────────────────────────────────────┐    │
│  │  Mock Chainalysis Oracle Program                   │    │
│  │  (separate program for testing)                    │    │
│  └────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
         │                                      │
         │ Creates/Reads                       │ Checks via CPI
         ▼                                      ▼
┌─────────────────────┐              ┌──────────────────┐
│ BlacklistEntry PDA  │              │  Chainalysis     │
│ (per address)       │◄─────────────┤  Oracle State    │
│                     │    CPI       │                  │
│ Seeds: ["blacklist",│              └──────────────────┘
│   account.as_ref()]  │
│                     │
│ - account: Pubkey   │
│ - blacklisted: bool │
│ - timestamp: i64    │
│ - bump: u8          │
└─────────────────────┘
```

### Storage Model

#### Main Registry Account
```rust
#[account]
#[derive(InitSpace)]
pub struct AccessRegistry {
    pub owner: Pubkey,              // 32 bytes - Registry owner (multisig)
    pub pending_owner: Pubkey,      // 32 bytes - For two-step ownership transfer
    pub chainalysis_oracle: Pubkey, // 32 bytes - Chainalysis Oracle program ID
    pub pool_factory_owner: Pubkey, // 32 bytes - PoolFactory owner (auto-approved)
    pub blacklist_count: u32,       // 4 bytes  - Count of blacklisted addresses
    pub bump: u8,                   // 1 byte   - PDA bump
}
// Space: 8 (discriminator) + 32 + 32 + 32 + 32 + 4 + 1 = 141 bytes
```

#### Per-Address Blacklist Entry
```rust
#[account]
#[derive(InitSpace)]
pub struct BlacklistEntry {
    pub account: Pubkey,   // 32 bytes - The address being blacklisted
    pub blacklisted: bool, // 1 byte   - Blacklist status
    pub timestamp: i64,    // 8 bytes  - When added to blacklist
    pub bump: u8,          // 1 byte   - PDA bump
}
// Space: 8 (discriminator) + 32 + 1 + 8 + 1 = 50 bytes
```

### Key Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| **Blacklist Storage** | Separate PDAs per address | Rent-efficient, enumerable via RPC |
| **Batch Size** | 15 addresses max | Solana transaction/account limits |
| **Oracle Strategy** | Mock program for testing | Chainalysis has no Solana program |
| **Read-Only Functions** | On-chain instructions | Required for oracle CPI; document RPC optimization |
| **Batch Atomicity** | All-or-nothing (revert on failure) | Matches Solidity semantics |
| **Ownership Transfer** | Two-step (pending → accept) | Prevents accidental transfers |
| **Re-init Protection** | Check `owner == default()` in init | Critical security guard |
| **Upgradeability** | BPFLoaderUpgradeable | Matches UUPS pattern from Solidity |

---

## Technical Approach

### Implementation Phases

#### Phase 1: Project Setup & Core Structure (Foundation)

**Tasks:**
1. Create new Anchor workspace: `access-registry`
2. Generate program ID: `anchor keys list`
3. Add `declare_id!()` macro to `lib.rs`
4. Configure `Anchor.toml` for localnet testing
5. Set up TypeScript test environment
6. Create account structs (`AccessRegistry`, `BlacklistEntry`)
7. Define error codes (`AccessRegistryError`)

**Deliverables:**
- Working Anchor workspace
- Program builds successfully: `anchor build`
- Basic test file structure

**Acceptance Criteria:**
- [ ] `anchor build` completes without errors
- [ ] `anchor keys list` shows program ID
- [ ] Account structs compile with `INIT_SPACE` derivation
- [ ] Error codes defined with descriptive messages

**Estimated Effort:** 1-2 hours

---

#### Phase 2: Initialize & Ownership Management

**Tasks:**
1. Implement `initialize` instruction
   - Create `AccessRegistry` PDA with seeds `["access_registry"]`
   - Set owner, oracle, pool_factory addresses
   - Accept optional initial blacklist (create `BlacklistEntry` PDAs)
   - **CRITICAL**: Re-initialization protection (check `owner == default()`)
2. Implement `transfer_ownership` instruction
   - Set `pending_owner` field
3. Implement `accept_ownership` instruction
   - Validate caller is `pending_owner`
   - Transfer ownership
4. Implement `renounce_ownership` instruction (optional)
   - Set owner to `Pubkey::default()`

**Account Validation:**
```rust
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + AccessRegistry::INIT_SPACE,
        seeds = [b"access_registry"],
        bump
    )]
    pub registry: Account<'info, AccessRegistry>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

**Acceptance Criteria:**
- [ ] `initialize` creates registry with correct owner
- [ ] Calling `initialize` twice returns `AlreadyInitialized` error
- [ ] Initial blacklist entries are created atomically
- [ ] Ownership transfer requires two steps
- [ ] Only pending owner can accept ownership
- [ ] All tests pass: `anchor test`

**Estimated Effort:** 2-3 hours

---

#### Phase 3: Blacklist Management

**Tasks:**
1. Implement `set_blacklisted` instruction (single address)
   - Validate caller is `owner`
   - Create or close `BlacklistEntry` PDA
   - Seeds: `["blacklist", account.as_ref()]`
   - Update `blacklist_count`
   - Emit event: `BlacklistUpdated`
2. Implement `set_blacklisted_batch` instruction
   - Validate batch size (1-15 addresses)
   - Process atomically (revert if any fails)
   - Prevent blacklisting special addresses (registry owner, pool factory owner)
3. Implement helper functions
   - `_check_not_special_address()` - Validate not owner/factory
   - `_create_blacklist_entry()` - Create PDA
   - `_close_blacklist_entry()` - Close PDA and return rent

**Account Validation:**
```rust
#[derive(Accounts)]
pub struct SetBlacklisted<'info> {
    #[account(
        mut,
        seeds = [b"access_registry"],
        bump = registry.bump
    )]
    pub registry: Account<'info, AccessRegistry>,
    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + BlacklistEntry::INIT_SPACE,
        seeds = [b"blacklist", account.key().as_ref()],
        bump
    )]
    pub blacklist_entry: Account<'info, BlacklistEntry>,
    /// CHECK: account is only used as seed
    pub account: UncheckedAccount<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

**Acceptance Criteria:**
- [ ] Owner can blacklist address (creates `BlacklistEntry` PDA)
- [ ] Owner can unblacklist address (closes PDA, returns rent)
- [ ] Non-owner cannot call `set_blacklisted` (Unauthorized error)
- [ ] Cannot blacklist registry owner or pool factory owner
- [ ] Batch operations process atomically
- [ ] Batch size > 15 returns `InvalidBatchSize` error
- [ ] Empty batch returns `InvalidBatchSize` error
- [ ] Duplicate addresses in batch handled correctly (idempotent)
- [ ] Events emitted for all state changes
- [ ] All tests pass

**Estimated Effort:** 3-4 hours

---

#### Phase 4: Mock Oracle Program

**Tasks:**
1. Create separate Anchor workspace: `mock-chainalysis-oracle`
2. Implement oracle state account
   ```rust
   #[account]
   pub struct OracleState {
       pub authority: Pubkey,
       pub bump: u8,
   }
   ```
3. Implement `is_sanctioned` instruction
   - Takes `account: Pubkey` parameter
   - Returns `bool` via custom error or account data
   - For testing: Return `true` for hardcoded test addresses
4. Implement `set_sanctioned` instruction (test configuration)
   - Allow adding/removing addresses from test sanctions list
5. Document oracle interface for production integration

**Oracle Interface:**
```rust
// Instruction
pub fn is_sanctioned(ctx: Context<IsSanctioned>, account: Pubkey) -> Result<bool>

// Error for sanctioned addresses
#[error_code]
pub enum OracleError {
    #[msg("Address is sanctioned")]
    AddressSanctioned,
}

// Usage in AccessRegistry:
chainalysis::is_sanctioned(cpi_context, account)
    .expect("Oracle call failed"); // Fail-closed
```

**Acceptance Criteria:**
- [ ] Mock oracle program builds successfully
- [ ] `is_sanctioned` returns `true` for test addresses
- [ ] `is_sanctioned` returns `false` for other addresses
- [ ] `set_sanctioned` allows runtime configuration
- [ ] Oracle program ID can be passed to `AccessRegistry.initialize`
- [ ] All tests pass

**Estimated Effort:** 2-3 hours

---

#### Phase 5: Oracle Integration & Approval Logic

**Tasks:**
1. Implement `is_approved` instruction
   - Check if caller is registry owner → return `true`
   - Check if caller is pool factory owner → return `true`
   - Check if `BlacklistEntry` PDA exists → return `false`
   - If `chainalysis_oracle != default()`: CPI to oracle
   - **Fail-closed**: If CPI fails, revert transaction
   - Return result via account data or custom error
2. Implement `is_sanctioned_by_chainalysis` instruction (oracle-only check)
3. Implement `get_approved_batch` instruction
   - Accept array of addresses (max 15)
   - Return array of booleans
   - Process oracle checks sequentially

**CPI Pattern:**
```rust
pub fn is_approved(ctx: Context<IsApproved>, account: Pubkey) -> Result<()> {
    let registry = &ctx.accounts.registry;

    // Auto-approve registry owner
    if account == registry.owner {
        msg!("Auto-approved: registry owner");
        return Ok(());
    }

    // Auto-approve pool factory owner
    if account == registry.pool_factory_owner {
        msg!("Auto-approved: pool factory owner");
        return Ok(());
    }

    // Check internal blacklist
    if ctx.accounts.blacklist_entry.is_some() {
        return Err(AccessRegistryError::AddressBlacklisted.into());
    }

    // Check Chainalysis Oracle
    if registry.chainalysis_oracle != Pubkey::default() {
        let cpi_program = ctx.accounts.chainalysis_program.to_account_info();
        let cpi_accounts = chainalysis::accounts::IsSanctioned {
            oracle_state: ctx.accounts.oracle_state.to_account_info(),
        };
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        match chainalysis::is_sanctioned(cpi_context, account) {
            Ok(_) => {}, // Not sanctioned
            Err(_) => {
                // Fail-closed: any oracle error reverts transaction
                return Err(AccessRegistryError::OracleFailure.into());
            }
        }
    }

    Ok(())
}
```

**Acceptance Criteria:**
- [ ] Registry owner is always approved
- [ ] Pool factory owner is always approved
- [ ] Blacklisted addresses return `false`
- [ ] Oracle-sanctioned addresses return `false`
- [ ] Clean addresses return `true`
- [ ] Oracle call failures revert transaction (fail-closed)
- [ ] When oracle not set, skip oracle check
- [ ] Batch approval checks process correctly
- [ ] All tests pass

**Estimated Effort:** 3-4 hours

---

#### Phase 6: Testing & Validation

**Tasks:**
1. Write comprehensive test suite
   - Initialization tests
   - Blacklist management tests
   - Oracle integration tests
   - Approval logic tests
   - Edge case tests (re-init, unauthorized, etc.)
2. Add test utilities
   - PD A derivation helpers
   - Test account generation
   - Mock oracle helpers
3. Verify test coverage ≥90%
4. Run `anchor test` repeatedly for stability

**Test Categories:**
```typescript
describe("AccessRegistry", () => {
  describe("Initialization", () => {
    it("creates registry with correct owner")
    it("prevents re-initialization attack")
    it("accepts initial blacklist")
    it("rejects invalid oracle address")
  })

  describe("Blacklist Management", () => {
    it("allows owner to blacklist address")
    it("allows owner to unblacklist address")
    it("rejects non-owner attempts")
    it("prevents blacklisting special addresses")
    it("handles batch operations atomically")
    it("rejects oversized batches")
    it("handles duplicate addresses")
    it("returns rent on unblacklist")
  })

  describe("Oracle Integration", () => {
    it("calls oracle successfully")
    it("reverts on oracle failure (fail-closed)")
    it("skips oracle when not configured")
    it("handles sanctioned addresses")
  })

  describe("Approval Logic", () => {
    it("auto-approves registry owner")
    it("auto-approves pool factory owner")
    it("rejects blacklisted addresses")
    it("rejects oracle-sanctioned addresses")
    it("approves clean addresses")
    it("handles batch approval checks")
  })

  describe("Ownership Transfer", () => {
    it("initiates two-step transfer")
    it("allows pending owner to accept")
    it("prevents non-pending owner acceptance")
  })
})
```

**Acceptance Criteria:**
- [ ] All tests pass: `anchor test`
- [ ] Test coverage ≥90% (including error paths)
- [ ] No flaky tests (run 10+ times successfully)
- [ ] Edge cases covered (empty arrays, max limits, etc.)

**Estimated Effort:** 3-4 hours

---

#### Phase 7: Polish & Documentation

**Tasks:**
1. Add event emission via `emit!()` macro
2. Create client helper library (TypeScript)
   - PDA derivation helpers
   - Transaction builders
   - RPC optimization (check blacklist via RPC first)
3. Write integration documentation
   - Deployment guide
   - Usage examples
   - Configuration options
4. Create Solidity → Anchor comparison guide
   - Side-by-side function mapping
   - Pattern explanations
5. Clean up code
   - Remove unused imports
   - Add inline comments
   - Format with `anchor fmt`

**Deliverables:**
- Client helper library
- Integration docs
- Comparison guide
- Clean, commented code

**Acceptance Criteria:**
- [ ] Events emitted for all state changes
- [ ] Client helper tests pass
- [ ] Documentation is clear and accurate
- [ ] Code is formatted and linted
- [ ] No compiler warnings

**Estimated Effort:** 2-3 hours

---

## Alternative Approaches Considered

### Storage Model: HashMap vs. PDAs

**Rejected:** Using HashMap for blacklist storage

**Why:**
- `HashMap` is not serializable by Anchor's `#[account]` macro
- Would require custom serialization/deserialization
- Less rent-efficient (one large account vs. many small PDAs)
- Cannot enumerate via RPC (`get_program_accounts`)

**Chosen:** Separate PDA accounts per blacklisted address

---

### Oracle Integration: On-Chain vs. Off-Chain

**Rejected:** Off-chain oracle with periodic updates

**Why:**
- Breaks fail-closed security model
- Stale oracle data creates compliance risk
- Requires trust in off-chain data provider
- Doesn't match Solidity's on-chain call pattern

**Chosen:** On-chain CPI to oracle program (mock for testing)

---

### Read-Only Functions: RPC-Only vs. On-Chain

**Rejected:** Pure RPC-based read functions

**Why:**
- Cannot check Chainalysis Oracle without on-chain call
- Split logic creates confusion (which method to use?)
- Inconsistent with Solidity's `isApproved()` pattern

**Chosen:** On-chain instructions with documented RPC optimization

---

## Acceptance Criteria

### Functional Requirements

#### Core Functionality
- [ ] Registry can be initialized with owner, oracle, and pool factory addresses
- [ ] Owner can add addresses to blacklist
- [ ] Owner can remove addresses from blacklist (closes PDA, returns rent)
- [ ] Owner can batch add/remove up to 15 addresses atomically
- [ ] Non-owners cannot modify blacklist
- [ ] Registry owner and pool factory owner are always approved
- [ ] Blacklisted addresses are rejected
- [ ] Oracle-sanctioned addresses are rejected
- [ ] Oracle call failures revert transaction (fail-closed)

#### Ownership Management
- [ ] Owner can initiate two-step ownership transfer
- [ ] Pending owner can accept ownership
- [ ] Non-pending owner cannot accept ownership
- [ ] Owner can renounce ownership (optional)

#### Security
- [ ] `initialize` cannot be called twice (re-init attack protection)
- [ ] Registry owner cannot be blacklisted
- [ ] Pool factory owner cannot be blacklisted
- [ ] Only owner can call admin functions
- [ ] Batch operations revert entirely if any item fails

#### Oracle Integration
- [ ] When oracle not set, skip oracle check
- [ ] When oracle set, always call it (fail-closed)
- [ ] Mock oracle works for testing
- [ ] Oracle interface documented for production

---

### Non-Functional Requirements

#### Performance
- [ ] Single blacklist operation completes in <200k CU
- [ ] Batch operation (15 addresses) completes in <500k CU
- [ ] Approval check (with oracle) completes in <300k CU
- [ ] Account rent exemption is efficient (~50 bytes per entry)

#### Security
- [ ] All account validations pass Anchor's constraints
- [ ] No unchecked arithmetic overflow
- [ ] No privilege escalation vulnerabilities
- [ ] Re-initialization attack prevented
- [ ] Unauthorized ownership transfer prevented

#### Compatibility
- [ ] Matches Solidity function semantics exactly
- [ ] Events compatible with indexers
- [ ] Client library usable in TypeScript/JavaScript
- [ ] PDA derivation deterministic

---

### Quality Gates

#### Testing
- [ ] Unit tests for all instructions
- [ ] Integration tests for full flows
- [ ] Edge case tests (limits, empty arrays, duplicates)
- [ ] Error path tests (all error codes triggered)
- [ ] Test coverage ≥90%
- [ ] All tests pass consistently (10+ runs)

#### Code Quality
- [ ] No compiler warnings
- [ ] Code formatted with `anchor fmt`
- [ ] Inline comments for complex logic
- [ ] Error messages are descriptive
- [ ] Public functions have documentation comments

#### Documentation
- [ ] README with usage examples
- [ ] API documentation (comments exported)
- [ ] Deployment guide
- [ ] Solidity → Anchor comparison guide
- [ ] Client helper library documentation

---

## Success Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| **Build Success** | 100% | `anchor build` completes without errors |
| **Test Pass Rate** | 100% | All tests pass: `anchor test` |
| **Test Coverage** | ≥90% | Lines covered by tests |
| **Feature Parity** | 100% | All Solidity functions implemented |
| **Documentation** | Complete | All sections filled in README |
| **Compute Efficiency** | <500k CU | Batch operations under limit |
| **Rent Efficiency** | ~50 bytes | Per blacklist entry |

---

## Dependencies & Prerequisites

### External Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| Anchor CLI | 0.32.1 | Framework |
| Solana CLI | 3.1.8 | Tooling |
| Rust | 1.93.0 | Language |
| Node.js | v24.10.0 | Tests |
| TypeScript | 5.x | Test language |

### Internal Dependencies

| Item | Status | Notes |
|------|--------|-------|
| Mock Oracle Program | Required | Create in Phase 4 |
| GYLD Solidity Tests | Reference | Use for comparison |
| Learning Path Docs | Complete | Reference for patterns |

### Prerequisite Work

1. ✅ Brainstorm document created (`2026-02-03-gyld-solidity-to-anchor-conversion.md`)
2. ✅ Local patterns researched (account validation, PDAs, CPI)
3. ✅ SpecFlow analysis complete (15 critical questions identified)
4. ⏭️ This plan approved and ready for implementation

---

## Risk Analysis & Mitigation

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| **Chainalysis has no Solana oracle** | High | 100% | Use mock oracle for testing; document production gap |
| **Re-initialization vulnerability** | Critical | Medium | Add guard: check `owner == default()` before setting |
| **Batch size too restrictive** | Medium | Low | Document compute budget increase option for larger batches |
| **Rent costs too high** | Medium | Low | Close PDAs on unblacklist; use efficient struct layout |
| **Oracle CPI fails frequently** | High | Medium | Document fail-closed behavior; consider retry logic |
| **Transaction size limits** | Medium | Low | Conservative batch size (15); test for breaking point |
| **PDA derivation collision** | Low | Very Low | Use unique seeds; document derivation pattern |
| **Ownership transfer confusion** | Medium | Low | Clear documentation; two-step transfer prevents accidents |

---

## Open Questions (To Resolve During Implementation)

### Critical (Blocks Implementation)

1. **Q: Exact oracle CPI interface?**
   - **Decision:** Design mock oracle with `is_sanctioned(account: Pubkey) -> Result<()>`
   - Returns `Err(AddressSanctioned)` if sanctioned, `Ok(())` if not
   - Document that production requires Chainalysis to build Solana program

2. **Q: Re-initialization protection mechanism?**
   - **Decision:** Use `#[account(init)]` constraint which creates account once
   - Additional check: `require!(registry.owner == Pubkey::default())`

3. **Q: Exact batch size limit?**
   - **Decision:** Start with `MAX_BATCH_SIZE = 15`
   - Document that clients can use compute-budget program for larger batches

4. **Q: Read-only function implementation?**
   - **Decision:** Provide on-chain instructions
   - Document RPC optimization: check `BlacklistEntry` PDAs via RPC first

### Important (Affects UX)

5. **Q: Batch operation atomicity?**
   - **Decision:** Atomic (all-or-nothing) to match Solidity
   - Entire transaction reverts if any item fails

6. **Q: PDA closing behavior on unblacklist?**
   - **Decision:** Close PDA and return lamports to `registry.owner`

7. **Q: Special address handling?**
   - **Decision:** Explicitly reject with error
   - Batch fails if contains special address

8. **Q: Ownership transfer mechanism?**
   - **Decision:** Two-step transfer (`transfer_ownership` → `accept_ownership`)

9. **Q: Error codes to define?**
   - **Decision:** 6 core errors (Unauthorized, AlreadyInitialized, InvalidBatchSize, OracleFailure, CannotBlacklistSpecialAddress, AlreadyBlacklisted)

---

## File Structure

```
access-registry/
├── programs/
│   ├── access-registry/
│   │   ├── src/
│   │   │   ├── lib.rs                 # Main program
│   │   │   ├── state.rs               # Account structs
│   │   │   ├── instructions/          # Instruction handlers
│   │   │   │   ├── mod.rs
│   │   │   │   ├── initialize.rs
│   │   │   │   ├── blacklist.rs
│   │   │   │   ├── approval.rs
│   │   │   │   └── ownership.rs
│   │   │   ├── error.rs               # Error codes
│   │   │   └── constants.rs           # Constants
│   │   └── Cargo.toml
│   └── mock-chainalysis-oracle/
│       ├── src/
│       │   ├── lib.rs
│       │   ├── state.rs
│       │   └── instructions/
│       └── Cargo.toml
├── tests/
│   ├── access-registry.test.ts        # Main test suite
│   └── helpers.ts                     # Test utilities
├── client/
│   ├── src/
│   │   ├── pdas.ts                    # PDA derivation
│   │   ├── transactions.ts            # Transaction builders
│   │   └── rpc.ts                     # RPC helpers
│   └── package.json
├── Anchor.toml
├── Cargo.toml
├── README.md                          # Integration docs
└── SOLIDITY_COMPARISON.md             # Comparison guide
```

---

## Implementation Checklist

### Phase 1: Setup
- [x] Create Anchor workspace (added to existing workspace)
- [x] Generate program ID
- [x] Define account structs
- [x] Define error codes
- [x] Verify `anchor build` works

### Phase 2: Initialize & Ownership
- [x] Implement `initialize` instruction
- [x] Add re-init protection
- [x] Implement `transfer_ownership`
- [x] Implement `accept_ownership`
- [x] Write tests

### Phase 3: Blacklist Management
- [x] Implement `set_blacklisted`
- [x] Implement `set_blacklisted_batch`
- [x] Add special address checks
- [ ] Add event emission
- [x] Write tests

### Phase 4: Mock Oracle
- [ ] Create oracle workspace
- [ ] Implement oracle state
- [ ] Implement `is_sanctioned`
- [ ] Implement `set_sanctioned` (test config)
- [ ] Write tests

### Phase 5: Approval Logic
- [x] Implement `is_approved`
- [x] Implement `is_sanctioned_by_chainalysis`
- [x] Implement `get_approved_batch`
- [ ] Add CPI to oracle
- [ ] Add fail-closed error handling
- [x] Write tests

### Phase 6: Testing
- [x] Write initialization tests
- [x] Write blacklist tests
- [ ] Write oracle tests
- [x] Write approval tests
- [ ] Write edge case tests
- [ ] Verify ≥90% coverage

### Phase 7: Polish
- [ ] Add event emission
- [ ] Create client helper library
- [ ] Write README
- [ ] Write comparison guide
- [ ] Format and lint
- [ ] Final test pass

---

## References & Research

### Internal References

- **Solidity Source:** `/mnt/d/GYLD/Foundry/gyld-contracts/src/contracts/AccessRegistry.sol`
- **Foundry Tests:** `/mnt/d/GYLD/Foundry/gyld-contracts/test/VaultUpgradeableTest.t.sol`
- **Brainstorm:** `docs/brainstorms/2026-02-03-gyld-solidity-to-anchor-conversion.md`
- **Learning Path:** `docs/plans/2026-02-02-feat-solidity-dev-learning-path-cookbook-plan.md`
- **Project Patterns:** `programs/anchortokenstarter/src/lib.rs:78-183` (account validation, PDAs, CPI)
- **Test Patterns:** `tests/anchortokenstarter.ts` (Mocha, PDA derivation)

### External References

- **Anchor Documentation:** https://www.anchor-lang.com/docs
- **Solana Cookbook:** https://solanacookbook.com/
- **CPI Guide:** https://www.anchor-lang.com/docs/cross-program-invocations
- **Account Validation:** https://www.anchor-lang.com/docs/account-constraints
- **SPL Token:** https://spl.solana.com/token
- **Chainalysis Oracle (Ethereum):** 0x40C57923924B5c5c5455c48D93317139ADDaC8fb

### Related Work

- **GYLD Protocol:** https://github.com/gyld-finance (Foundry version)
- **UUPS Pattern:** OpenZeppelin upgradeable contracts
- **Fail-Closed Design:** Security best practices for compliance

---

## Appendix A: Solidity → Anchor Function Mapping

| Solidity Function | Anchor Function | Notes |
|-------------------|-----------------|-------|
| `constructor(chainalysisOracle, initialBlacklist)` | `initialize(chainalysis_oracle, pool_factory, initial_blacklist)` | Also accepts pool_factory |
| `setBlacklisted(account, blacklisted)` | `set_blacklisted(account, blacklisted)` | Same semantics |
| `setBlacklistedBatch(accounts[], blacklisted)` | `set_blacklisted_batch(accounts[], blacklisted)` | Max 15 (vs 100) |
| `isApproved(account) → bool` | `is_approved(account)` | Returns via account data, not bool |
| `isBlacklisted(account) → bool` | Read from RPC + `is_sanctioned_by_chainalysis()` | Split across RPC + CPI |
| `isSanctionedByChainalysis(account) → bool` | `is_sanctioned_by_chainalysis(account)` | CPI to oracle |
| `getApprovedBatch(accounts[]) → bool[]` | `get_approved_batch(accounts[])` | Returns via account data |
| `setChainalysisOracle(oracle)` | N/A (immutable after init) | Use redeploy to change |
| `setPoolFactory(factory)` | N/A (immutable after init) | Use redeploy to change |
| `transferOwnership(newOwner)` | `transfer_ownership(newOwner)` + `accept_ownership()` | Two-step process |

---

## Appendix B: PDA Derivation Reference

```typescript
// Registry PDA
const [registryPda] = PublicKey.findProgramAddressSync(
  [Buffer.from("access_registry")],
  program.programId
);

// Blacklist Entry PDA
const [blacklistEntryPda] = PublicKey.findProgramAddressSync(
  [Buffer.from("blacklist"), userPublicKey.toBuffer()],
  program.programId
);

// Mock Oracle State PDA
const [oracleStatePda] = PublicKey.findProgramAddressSync(
  [Buffer.from("oracle_state")],
  oracleProgramId
);
```

---

## Appendix C: Error Codes

```rust
#[error_code]
pub enum AccessRegistryError {
    #[msg("Registry already initialized")]
    AlreadyInitialized,

    #[msg("Unauthorized: caller is not the owner")]
    Unauthorized,

    #[msg("Batch size must be between 1 and 15")]
    InvalidBatchSize,

    #[msg("Chainalysis oracle call failed")]
    OracleFailure,

    #[msg("Cannot blacklist registry owner or pool factory owner")]
    CannotBlacklistSpecialAddress,

    #[msg("Address is already blacklisted")]
    AlreadyBlacklisted,

    #[msg("Address is not blacklisted")]
    NotBlacklisted,

    #[msg("Invalid pending owner")]
    InvalidPendingOwner,
}
```

---

**Document Status:** ✅ Ready for Implementation
**Next Action:** Run `/workflows:work` to begin Phase 1
