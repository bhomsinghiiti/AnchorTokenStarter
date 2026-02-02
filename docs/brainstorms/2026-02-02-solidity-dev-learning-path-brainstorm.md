# Solidity Developer Learning Path: Solana/Rust Documentation Strategy

**Date:** 2026-02-02
**Status:** Accepted
**Author:** Claude Code

---

## What We're Building

A pattern-based cookbook documentation system to help Solidity developers learn Rust and Solana development using the existing AnchorTokenStarter project as the foundation.

**User Profile:**
- Solidity developer wanting to learn Rust/Solana
- Has Rust basics (syntax, ownership, basic types)
- Prefers parallel learning (concepts + building together)
- Wants minimal Solidity comparisons - focus on Solana patterns

**Goals:**
- Learn Rust for Solana smart contracts from near-zero
- Understand SPL tokens compared to ERC20
- Learn while modifying this project incrementally
- Get productive quickly (parallel learning)

---

## Why This Approach

### Research Findings

**What Solidity Devs Struggle With Most:**
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

### Pattern-Based Cookbook Chosen

**Why this fits:**
- **Parallel learning:** Each pattern teaches concepts while building
- **Quick win:** Get counter running in 10 minutes
- **Reference-friendly:** Come back to specific patterns later
- **SPL focus:** Token patterns map well to ERC20 knowledge
- **Your project fits:** Existing instructions follow good progression

---

## Key Decisions

### Decision 1: Documentation Structure

```
docs/
├── 00-quick-start.md          # Get counter running in 10 minutes
├── patterns/
│   ├── initialization/
│   │   ├── hello-world.md         # No state, basic structure
│   │   ├── simple-counter.md      # PDA state account
│   │   └── config-account.md      # Multi-field accounts
│   ├── token-operations/
│   │   ├── create-mint.md         # SPL Token vs ERC20
│   │   ├── mint-tokens.md         # Mint to ATA with CPI
│   │   ├── transfer-tokens.md     # Transfer between ATAs
│   │   └── burn-tokens.md         # Future: Burn operation
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

### Decision 2: Content Pattern Template

Each pattern page follows this structure:
```markdown
## Pattern: [Name]

**Concept:** [What this pattern teaches]
**Related Code:** `programs/anchortokenstarter/src/lib.rs:[line-range]`
**Test:** `tests/anchortokenstarter.ts:[line-range]`

### The Concept
[Explanation of the Solana/Rust concept]

### The Code
[Code snippet with annotations]

### Running It
```bash
anchor test --skip-local
```

### Solidity Sidebar (minimal)
[Quick comparison only where essential]
```

### Decision 3: Progressive Complexity

Learning progression follows existing project:
1. **Quick Start** - Run existing counter test (success!)
2. **Pattern: Simple Counter** - PDAs, accounts, state
3. **Pattern: Create Mint** - SPL Token vs ERC20
4. **Pattern: Mint Tokens** - CPI, ATA, mint authority
5. **Pattern: Transfer Tokens** - Multi-account validation

### Decision 4: Documentation Updates Needed

**README.md Updates:**
- Add "Learning Path for Solidity Developers" section
- Link to pattern documentation
- Add SPL Token vs ERC20 comparison table

**CLAUDE.md Updates:**
- Add "Rust for Solidity Developers" quick reference
- Add account model explanation
- Add PDA derivation patterns

**New Documentation:**
- Pattern library (docs/patterns/*.md)
- Concept deep-dives (docs/concepts/*.md)
- Solidity cheat sheet (docs/reference/solidity-comparison.md)

### Decision 5: Complete Stub Implementation

**`create_mint` instruction is incomplete** - needs to actually initialize the mint using `token_interface::initialize_mint`. This should be fixed as part of the learning path.

---

## Open Questions

1. **Completion criteria:** How many patterns before the user is "productive"? (Suggested: complete first 4 patterns)

2. **Future patterns:** Should we add burn, freeze, or DeFi patterns later? (Deferred - focus on core patterns first)

3. **Testing depth:** Should we write tests for each pattern or use existing tests? (Use existing tests, add new tests for new patterns)

4. **Deployment guide:** Should we include devnet/mainnet deployment instructions? (Yes - after local testing works)

---

## Alternatives Considered

### Alternative 1: Progressive Projects
Tutorials for each project (Counter → Token Mint → Transfer).

**Rejected:** Less reference-friendly, more redundancy. User wants parallel learning, not linear tutorials.

### Alternative 2: Mental Model First
Deep dive into SVM, Rust, then build.

**Rejected:** Slower to working code. User wants to learn while building, not abstract study first.

---

## Next Steps

1. ✅ Run `/workflows:plan` to create implementation plan
2. Write pattern documentation following template
3. Update README.md with learning path section
4. Update CLAUDE.md with Rust/Solidity reference
5. Fix `create_mint` implementation (it's a stub!)
6. Add Solidity cheat sheet to reference/
7. Test all patterns with `anchor test`

---

## Sources

- [Solana EVM to SVM Complete Guide](https://solana.com/developers/evm-to-svm/complete-guide)
- [Solidity vs Rust - RareSkills](https://rareskills.io/post/solidity-vs-rust)
- [Learning Rust for Solana - Medium](https://medium.com/@temiw3/learning-rust-for-solana-what-two-bootcamps-actually-taught-me-73fc832499fc)
- [60 Days of Solana Tutorial](https://rareskills.io/solana-tutorial)
- [Project-Based Learning vs Tutorials](https://www.frontendmentor.io/articles/project-based-learning-vs-tutorials)
