---
title: Solidity Developer Learning Path - Pattern-Based Cookbook
type: feat
date: 2026-02-02
status: Draft
---

# Solidity Developer Learning Path - Pattern-Based Cookbook

## Overview

Create a pattern-based cookbook documentation system to help Solidity developers learn Rust and Solana development using the existing AnchorTokenStarter project as the foundation.

**Target Audience:** Solidity developers with Rust basics (syntax, ownership, basic types) who want parallel learning - building features while understanding concepts simultaneously.

**Goal:** Enable developers to become productive with Solana/Anchor development through a quick start (10 minutes) followed by referenceable pattern documentation, concept deep-dives, and reference cheat sheets.

---

## Problem Statement / Motivation

**The Challenge:** Solidity developers face significant mental model shifts when moving to Solana:
- Account-based model vs EVM's storage-based contracts
- Program Derived Addresses (PDAs) - completely new concept
- Explicit account validation vs `msg.sender` pattern
- Cross-Program Invocations (CPIs) for common operations
- Different economic model (rent/lamports vs gas)

**The Gap:** Existing Solana documentation assumes familiarity with these concepts or presents them abstractly before showing working code. Developers learn best with quick wins followed by referenceable patterns.

**The Solution:** A pattern-based cookbook that gets users running in 10 minutes, then provides building-block patterns they can reference when building features.

---

## Proposed Solution

### Documentation Structure

```
docs/
├── 00-quick-start.md              # 10-minute success path
├── patterns/
│   ├── README.md                  # Pattern overview & navigation
│   ├── initialization/
│   │   ├── hello-world.md         # No state, basic structure
│   │   ├── simple-counter.md      # PDA state account
│   │   └── config-account.md      # Multi-field accounts (Exercise)
│   ├── token-operations/
│   │   ├── create-mint.md         # SPL Token vs ERC20
│   │   ├── mint-tokens.md         # Mint to ATA with CPI
│   │   ├── transfer-tokens.md     # Transfer between ATAs
│   │   └── burn-tokens.md         # Advanced Exercise
│   ├── cross-program-calls/
│   │   ├── cpi-to-token.md        # Calling SPL Token program
│   │   └── cpi-to-system.md       # System program interactions
│   └── security/
│       ├── signer-validation.md   # Authority checks
│       └── account-constraints.md # Validation patterns
├── concepts/
│   ├── account-model.md           # Data vs programs
│   ├── pdas-explained.md          # Program Derived Addresses
│   ├── anchor-macros.md           # #[account] attributes
│   └── rust-survival.md           # Ownership, Result, macros
└── reference/
    ├── solidity-comparison.md     # Quick cheat sheet
    └── common-errors.md           # Troubleshooting
```

### Pattern Template

Each pattern page follows this structure:

```markdown
## Pattern: [Name]

**Concept:** [What this pattern teaches]
**Related Code:** `programs/anchortokenstarter/src/lib.rs: <function_name>`
**Test:** `tests/anchortokenstarter.ts: <test_name>`
**Prerequisites:** [List any patterns or concepts to complete first]

### What You're Building
[Brief description of what this pattern accomplishes]

### The Concept
[Explanation of the Solana/Rust concept, 2-3 paragraphs]

### The Code
```rust
[Code snippet with inline comments]
```

### Running It
```bash
[Command to run]
```

**Expected output:** [What success looks like]

**If it fails:** [Common errors and solutions]

### Solidity Sidebar
[Quick comparison only where essential - 2-3 sentences]

### You've Learned
- [Skill 1]
- [Skill 2]

### Next Steps
- [Related pattern to try next]
```

### Progressive Complexity

Learning progression follows the existing project structure:

1. **Quick Start** → Run counter test, see it work (10 min)
2. **Hello World** → Basic program structure, logging
3. **Simple Counter** → PDAs, account state, initialization
4. **Create Mint** → SPL Token structure, mint accounts
5. **Mint Tokens** → CPIs, ATAs, mint authority
6. **Transfer Tokens** → Multi-account validation, authority checks

---

## Technical Considerations

### Cross-Reference Strategy

**Pattern → Concept:** Inline concept summaries (2-3 sentences) with "Deep dive: `concepts/<topic>.md`" links

**Concept → Pattern:** Each concept page includes "See this in action" section linking to relevant patterns

**Example:**
```markdown
### The Concept
A **PDA (Program Derived Address)** is a deterministic address derived from seeds and a program ID.
[2-3 sentence explanation]

**Deep dive:** See `concepts/pdas-explained.md` for full details on derivation, bump seeds, and signing.
```

### Code Reference Strategy

Use **function/struct names** instead of line ranges (line numbers break when code changes):

**Before:** `Related Code: lib.rs:21-27`
**After:** `Related Code: lib.rs: increment instruction, Increment accounts struct, Counter account`

### Environment Support

**Supported:** WSL2/Ubuntu only (documented in Quick Start)
**Planned:** macOS support in future
**Not supported:** Windows native (users must use WSL)

Quick Start includes prerequisite validation:
```bash
# Check versions
rustc --version    # Should be 1.93.0
solana --version   # Should be 3.1.8
anchor --version   # Should be 0.32.1
```

### Error Recovery Paths

Each pattern includes "If it fails" section with:
1. Common error messages
2. What they mean
3. How to fix them
4. When to check `reference/common-errors.md`

**Example:**
```markdown
**If it fails:**
- `Error: Account already exists` → The mint was created in a previous run. Run `solana-test-validator --reset --ledger /tmp/test-ledger` to clear.
- `Error: Invalid account data` → The account exists but isn't a valid mint. Delete it and re-run.
- See `reference/common-errors.md` for more troubleshooting.
```

### Solidity Comparison Balance

**"Minimal" means:** One sidebar per pattern, only when:
- The Solidity concept is directly analogous
- The comparison provides an "aha!" moment
- It explains a fundamental mental model shift

**Examples:**
- ✅ `create-mint.md`: Compare ERC20 deployment vs SPL mint creation (directly analogous)
- ✅ `simple-counter.md`: Compare contract storage vs PDA account (fundamental shift)
- ❌ `hello-world.md`: No comparison needed (basic syntax is assumed knowledge)

---

## Acceptance Criteria

### Phase 1: Foundation (MVP)

- [ ] Fix `create_mint` stub implementation to actually initialize mint using `token_interface::initialize_mint`
- [ ] Create `00-quick-start.md` with:
  - Prerequisite version checks
  - "Supported environments" section (WSL/Linux)
  - Run counter test instructions
  - Expected output and error recovery
  - "You are here" progress diagram
- [ ] Create `patterns/README.md` with navigation overview
- [ ] Update `README.md` with "Learning Path for Solidity Developers" section linking to docs
- [ ] Add "Rust for Solidity Developers" quick reference to `CLAUDE.md` (already partially done)

### Phase 2: Core Patterns

- [ ] Create `patterns/initialization/hello-world.md`
- [ ] Create `patterns/initialization/simple-counter.md` (explains PDAs, accounts)
- [ ] Create `patterns/token-operations/create-mint.md` (SPL Token vs ERC20)
- [ ] Create `patterns/token-operations/mint-tokens.md` (CPI, ATA, authority)
- [ ] Create `patterns/token-operations/transfer-tokens.md` (multi-account validation)

### Phase 3: Concepts Deep-Dives

- [ ] Create `concepts/account-model.md` (data vs programs, rent exemption)
- [ ] Create `concepts/pdas-explained.md` (derivation, bump seeds, signing)
- [ ] Create `concepts/anchor-macros.md` (#[account] attributes, constraints)
- [ ] Create `concepts/rust-survival.md` (ownership in Solana context, Result types)

### Phase 4: Reference Materials

- [ ] Create `reference/solidity-comparison.md` cheat sheet
- [ ] Create `reference/common-errors.md` with:
  - Build errors and solutions
  - Runtime errors and solutions
  - Account management errors
  - Test failure troubleshooting

### Phase 5: Advanced Content

- [ ] Mark `config-account.md` and `burn-tokens.md` as "Advanced Exercises"
- [ ] Create `patterns/cross-program-calls/cpi-to-token.md` (if not covered in mint-tokens)
- [ ] Create `patterns/security/signer-validation.md`
- [ ] Add "Deploying to Devnet" section (optional, based on user feedback)

### Quality Gates

- [ ] All code examples use function/struct names (not line ranges)
- [ ] All patterns include "If it fails" section
- [ ] All concepts link back to relevant patterns
- [ ] Quick Start test passes on fresh WSL2 environment
- [ ] Cross-links work between all pages

---

## Success Metrics

**Quantitative:**
- Quick Start can be completed in <10 minutes on fresh environment
- All patterns include runnable code snippets
- Zero broken links in cross-references

**Qualitative:**
- Users can modify existing patterns after completing first 4 patterns
- Users understand account model vs EVM storage model
- Users can debug common test failures using documentation

---

## Dependencies & Risks

### Dependencies

- **Rust 1.93.0**, **Anchor 0.32.1**, **Solana CLI 3.1.8** - versions pinned in CLAUDE.md
- Existing project structure (programs/anchortokenstarter, tests/)
- `create_mint` stub must be fixed before documenting that pattern

### Risks & Mitigation

| Risk | Impact | Mitigation |
|------|--------|------------|
| Code changes invalidate examples | High - docs become outdated | Use function names instead of line ranges; update docs when code changes |
| macOS users attempt and fail | Medium - poor experience | Explicitly state WSL/Linux only in Quick Start |
| Users skip to complex patterns | Low - confusion | Mark prerequisites on each pattern; suggest progressive order |
| Over-explaining Solidity | Low - violates user preference | Limit to one sidebar per pattern; focus on Solana patterns |
| `create_mint` fix is complex | Medium - delays documentation | Phase 1 includes the fix; document other patterns in parallel |

---

## Implementation Details

### Fixing `create_mint` Stub

**Current state** (lib.rs:30-33):
```rust
pub fn create_mint(ctx: Context<CreateMint>, decimals: u8) -> Result<()> {
    msg!("Creating token mint with {} decimals", decimals);
    Ok(())
}
```

**Required implementation:**
```rust
pub fn create_mint(ctx: Context<CreateMint>, decimals: u8) -> Result<()> {
    let cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        InitializeMint {
            mint: ctx.accounts.mint.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        },
    );
    token_interface::initialize_mint(cpi_context, decimals, &ctx.accounts.mint.key(), None)?;
    Ok(())
}
```

Also add `rent: Sysvar<'info, Rent>` to `CreateMint` accounts struct.

### File Creation Checklist

```
docs/
├── 00-quick-start.md              # NEW
├── patterns/
│   ├── README.md                  # NEW
│   ├── initialization/
│   │   ├── hello-world.md         # NEW
│   │   ├── simple-counter.md      # NEW
│   │   └── config-account.md      # NEW (marked as Exercise)
│   ├── token-operations/
│   │   ├── create-mint.md         # NEW
│   │   ├── mint-tokens.md         # NEW
│   │   ├── transfer-tokens.md     # NEW
│   │   └── burn-tokens.md         # NEW (marked as Exercise)
│   ├── cross-program-calls/
│   │   ├── cpi-to-token.md        # NEW (or merge into mint-tokens)
│   │   └── cpi-to-system.md       # NEW
│   └── security/
│       ├── signer-validation.md   # NEW
│       └── account-constraints.md # NEW
├── concepts/
│   ├── account-model.md           # NEW
│   ├── pdas-explained.md          # NEW
│   ├── anchor-macros.md           # NEW
│   └── rust-survival.md           # NEW
└── reference/
    ├── solidity-comparison.md     # NEW
    └── common-errors.md           # NEW
```

### Files to Modify

- `README.md` - Add "Learning Path for Solidity Developers" section
- `CLAUDE.md` - Enhance Solidity reference section (partially done)
- `programs/anchortokenstarter/src/lib.rs` - Fix `create_mint` implementation
- `programs/anchortokenstarter/src/lib.rs` - Add `rent` account to `CreateMint` struct

---

## Open Questions from SpecFlow Analysis

**Resolved during planning:**

1. ✅ **Fix `create_mint` first?** Yes - Phase 1 includes the fix before documenting that pattern
2. ✅ **Version pinning strategy?** Reference CLAUDE.md in Quick Start prerequisite checks
3. ✅ **Environment support?** Explicitly WSL/Linux only in Quick Start
4. ✅ **Account exists errors?** Include "Re-running Tests" section in Quick Start
5. ✅ **Inline vs separate concepts?** Hybrid: inline summaries + deep dive links
6. ✅ **"Minimal" Solidity comparisons?** One sidebar per pattern, for direct analogies or mental model shifts
7. ✅ **Line ranges vs anchors?** Use function/struct names instead of line numbers
8. ✅ **"You've learned" summaries?** Include in pattern template
9. ✅ **Terminal output?** Abbreviated with note about transaction signatures
10. ✅ **Burn/config patterns?** Mark as "Advanced Exercises" with implementation instructions

**Deferred to user feedback:**
- Should we add a progress tracking mechanism? (Simple "You are here" diagram in MVP)
- Devnet deployment documentation scope? (Phase 5, optional)

---

## References & Research

### Internal References

- **Existing program code:** `programs/anchortokenstarter/src/lib.rs`
- **Test suite:** `tests/anchortokenstarter.ts`
- **Project configuration:** `Anchor.toml`, `rust-toolchain.toml`
- **Development guidelines:** `CLAUDE.md` (lines 137-223 contain Solidity/Rust reference)
- **Current README:** `README.md` (lines 1-101)

### Brainstorm Context

- **Brainstorm document:** `docs/brainstorms/2026-02-02-solidity-dev-learning-path-brainstorm.md`
- **Key decisions:** Pattern-Based Cookbook approach, progressive complexity, minimal Solidity comparisons
- **Chosen alternatives:** Rejected Progressive Projects and Mental Model First approaches

### External References

- [Solana EVM to SVM Complete Guide](https://solana.com/developers/evm-to-svm/complete-guide)
- [Solidity vs Rust - RareSkills](https://rareskills.io/post/solidity-vs-rust)
- [Learning Rust for Solana - Medium](https://medium.com/@temiw3/learning-rust-for-solana-what-two-bootcamps-actually-taught-me-73fc832499fc)
- [60 Days of Solana Tutorial](https://rareskills.io/solana-tutorial)
- [Solana Account Model - Alchemy](https://www.alchemy.com/overviews/solana-data-vs-program-accounts)
- [Project-Based Learning vs Tutorials](https://www.frontendmentor.io/articles/project-based-learning-vs-tutorials)

### Research Insights

**What Solidity Devs Struggle With:**
1. Account-based vs storage-based model (biggest hurdle)
2. PDAs (Program Derived Addresses) - completely new concept
3. Explicit account validation vs `msg.sender` pattern
4. Rust ownership/borrowing in Solana context
5. CPIs (Cross-Program Invocations)
6. Rent/lamports economic model

**Effective Learning Elements:**
- Quick wins first (hello world in 10 minutes)
- Pattern-based learning (recurring solutions)
- Concepts explained in context
- Test-driven exploration
- Incremental complexity progression

---

## Documentation Plan

### Templates to Create

1. **Pattern Template** (for authors creating new patterns)
2. **Concept Template** (for authors creating concept deep-dives)
3. **Quick Start Template** (for other projects to adapt)

### Style Guide

- **Bold** for emphasis on important terms
- Backticks for code paths: `programs/anchortokenstarter/src/lib.rs`
- Code blocks with language identifiers: `rust`, `typescript`, `bash`
- Tables for comparisons and structured data
- Section headers with `##` for major sections
- Sources at bottom with markdown links

### Naming Conventions

- Pattern files: `kebab-case.md` (e.g., `simple-counter.md`)
- Concept files: `kebab-case.md` (e.g., `account-model.md`)
- Reference files: `kebab-case.md` (e.g., `solidity-comparison.md`)
- All lowercase, hyphen-separated

---

## Future Considerations

### Phase 6+ (Post-MVP)

- macOS support and testing
- Video walkthroughs for visual learners
- Interactive code examples (playground)
- Community-contributed patterns
- Translations for international developers
- Advanced DeFi patterns (AMM, staking, lending)

### Extensibility

Pattern library structure allows easy addition of new patterns:
1. Create new file in appropriate category directory
2. Follow pattern template
3. Link from existing patterns and concepts
4. Update `patterns/README.md` navigation

---

## Appendix: SpecFlow Analysis Summary

The SpecFlow analyzer identified **21 gaps** across categories:
- **Onboarding & Setup:** Prerequisite validation, environment-specific instructions, clean state handling
- **Error Handling:** Test failure escalation, build error troubleshooting, rent exemption
- **Navigation:** Pattern-to-concept linking, bidirectional navigation, progressive dependencies
- **Code Implementation:** Stub implementation, test coverage, code line ranges
- **Content Strategy:** Solidity comparison balance, missing implementations
- **Deployment:** Devnet path, program ID handling
- **Measurement:** Productivity definition, progress tracking
- **Accessibility:** Code annotation style, terminal output samples

All critical gaps (Priority 1 and 2) have been addressed in this plan. Priority 3 gaps deferred to post-MVP.

**Critical Questions Resolved:**
- Q1: Fix `create_mint` before documenting ✅
- Q2: Version pinning via CLAUDE.md reference ✅
- Q3: Environment = WSL/Linux only ✅
- Q4: "Re-running Tests" section in Quick Start ✅
- Q5: Hybrid concept linking ✅
- Q6: "Minimal" = one sidebar for direct analogies ✅
- Q7: Function names instead of line ranges ✅
