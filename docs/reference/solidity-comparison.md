# Solidity to Solana Cheat Sheet

**Quick reference for Solidity developers transitioning to Solana**

---

## Syntax Mapping

| Concept | Solidity | Anchor/Rust |
|---------|---------|-------------|
| **Contract** | `contract MyToken` | `#[program] pub mod my_token` |
| **Function** | `function mint()` | `pub fn mint()` |
| **Public** | `public` / `external` | `pub fn` |
| **State Variable** | `uint256 count` | Separate `Counter` account |
| **Constructor** | `constructor()` | `initialize()` or `new()` |
| **Event** | `emit Event()` | `msg!()` macro |
| **Require** | `require()` | `require!()` macro |
| **Error** | `revert()` | `return Err(Error::...)` |
| **Mapping** | `mapping(address => uint)` | `#[account]` struct |
| **Address** | `address` | `Pubkey` |
| **Msg.sender** | `msg.sender` | `ctx.accounts.signer` |
| **This** | `this` | `self` (Rust) |
| **Uint256** | `uint256` | `u64` (Solana common) |

---

## Account vs Contract Model

### Solidity: Code + Data Together

```solidity
contract MyToken {
    string public name = "My Token";
    mapping(address => uint256) public balances;

    function mint(address to, uint256 amount) external {
        balances[to] += amount;
    }
}
```

### Solana: Code + Data Separate

```rust
#[program]
pub mod my_token {
    pub fn mint(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        // Call Token Program via CPI
        token_interface::mint_to(cpi_context, amount)?;
        Ok(())
    }
}

#[account]
pub struct TokenConfig {
    pub name: String,
}

// Balances stored in separate Token Accounts (SPL Token program)
```

---

## Account Validation

### Solidity: Implicit

```solidity
function transfer(address to, uint256 amount) external {
    require(balances[msg.sender] >= amount, "Insufficient");
    balances[msg.sender] -= amount;
    balances[to] += amount;
}
```

### Solana: Explicit (in struct)

```rust
#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = signer,  // ← Validates ownership
    )]
    pub from: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = to,
    )]
    pub to: InterfaceAccount<'info, TokenAccount>,

    pub mint: InterfaceAccount<'info, Mint>,
}
```

---

## Function Parameters

### Solidity

```solidity
function mint(address to, uint256 amount) external {
    // to and amount are parameters
}
```

### Solana

```rust
// Parameters in function
pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
    // amount is a parameter
}

// Accounts in struct
#[derive(Accounts)]
pub struct MintTokens<'info> {
    pub signer: Signer<'info>,           // ← "to" is an account
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_account: InterfaceAccount<'info, TokenAccount>,
}
```

---

## State Storage

### Solidity: Contract Storage

```solidity
contract Counter {
    uint256 public count = 0;

    function increment() external {
        count++;
    }
}
```

### Solana: Separate Account

```rust
#[account]
pub struct Counter {
    pub count: u64,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + Counter::INIT_SPACE,
        seeds = [b"counter"],
        bump
    )]
    pub counter: Account<'info, Counter>,  // ← Separate account
    pub payer: Signer<'info>,
}
```

---

## Deploying a Token

### Solidity: Deploy Contract = Deploy Token

```solidity
// MyToken.sol
contract MyToken is ERC20 {
    constructor() ERC20("My Token", "MTK") {
        _mint(msg.sender, 1000000 * 10**18);
    }
}

// Deploy
MyToken token = new MyToken();
// token.address IS the token
```

### Solana: Create Mint Account

```rust
#[program]
pub mod my_token {
    pub fn create_mint(ctx: Context<CreateMint>, decimals: u8) -> Result<()> {
        // Mint account is created via constraint
        msg!("Created mint");
        Ok(())
    }
}

#[account(
    init,
    payer = payer,
    mint::decimals = 9,
    mint::authority = mint,
    seeds = [b"mint"],
    bump
)]
pub mint: InterfaceAccount<'info, Mint>,

// Your program is separate from the mint
```

---

## Token Operations

### ERC20 (Solidity)

```solidity
contract MyToken is ERC20 {
    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        _transfer(msg.sender, to, amount);
        return true;
    }
}
```

### SPL Token (Solana)

```rust
// Your program calls Token Program via CPI
pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
    let cpi = CpiContext::new(token_program, accounts).with_signer(seeds);
    token_interface::mint_to(cpi, amount)?;  // ← CPI
    Ok(())
}

pub fn transfer_tokens(ctx: Context<Transfer>, amount: u64) -> Result<()> {
    let cpi = CpiContext::new(token_program, accounts);
    token_interface::transfer_checked(cpi, amount, decimals)?;  // ← CPI
    Ok(())
}
```

---

## Authority Checks

### Solidity: `msg.sender`

```solidity
function onlyAdmin() external {
    require(msg.sender == admin, "Not admin");
    // ...
}
```

### Solana: Account Constraints

```rust
#[derive(Accounts)]
pub struct AdminOnly<'info> {
    #[account(
        constraint = authority.key() == config.admin @ ErrorCode::Unauthorized
    )]
    pub authority: Signer<'info>,

    pub config: Account<'info, Config>,
}
```

---

## Error Handling

### Solidity

```solidity
require(amount > 0, "Amount must be positive");
revert("Insufficient balance");
```

### Solana

```rust
require!(amount > 0, ErrorCode::InvalidAmount);
return Err(Error::InsufficientFunds.into());

#[error_code]
pub enum ErrorCode {
    #[msg("Amount must be positive")]
    InvalidAmount,

    #[msg("Insufficient funds")]
    InsufficientFunds,
}
```

---

## Events vs Logging

### Solidity Events

```solidity
event Transfer(address indexed from, address indexed to, uint256 amount);

function transfer(address to, uint256 amount) external {
    _transfer(from, to, amount);
    emit Transfer(from, to, amount);
}
```

### Solana Logging

```rust
pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
    token_interface::transfer_checked(cpi, amount, decimals)?;
    msg!("Transferred {} tokens", amount);  // ← Logging
    Ok(())
}
```

**Note:** Solana logs are not indexed like Ethereum events.

---

## Common Gotchas

| Solidity | Solana Gotcha |
|----------|---------------|
| `msg.sender` is available | All accounts must be explicitly passed |
| State lives in contract | State lives in separate accounts |
| `mapping` is built-in | Use `#[account]` structs |
| Deploy once = done | Programs can be upgraded |
| Gas fees | Rent + transaction fees |
| One contract per token | One program can manage infinite tokens |
| `address` type | `Pubkey` type (32 bytes) |
| `tx.origin` | No equivalent (security feature) |

---

## Mental Model Shift

### Think: Accounts as Files

**Solidity/Ethereum:**
- Contract = File (code + data mixed)

**Solana:**
- Program = Executable (code only)
- Accounts = Data files (read/written by programs)

### Think: PDAs as Program-Owned Files

**Solidity:**
- Contract owns its storage

**Solana:**
- Program can own files (PDAs)
- Program can sign for its own files (using bump seed)

---

## Quick Translation Guide

### Creating State

```solidity
// Solidity
contract Counter {
    uint256 public count = 0;
}
```

```rust
// Solana
#[account]
pub struct Counter {
    pub count: u64,
}

// Created in accounts struct with #[account(init, ...)]
```

### Reading State

```solidity
// Solidity
uint256 currentCount = count;
```

```rust
// Solana
let current_count = ctx.accounts.counter.count;
```

### Modifying State

```solidity
// Solidity
function increment() external {
    count++;
}
```

```rust
// Solana
pub fn increment(ctx: Context<Increment>) -> Result<()> {
    ctx.accounts.counter.count += 1;
    Ok(())
}

// Accounts struct needs #[account(mut)]
```

### Calling Other Contracts

```solidity
// Solidity
IERC20(token).transfer(recipient, amount);
```

```rust
// Solana - CPI
token_interface::transfer_checked(cpi_context, amount, decimals)?;
```

---

## Further Reading

**Patterns:**
- [Simple Counter](../patterns/initialization/simple-counter.md) - State management
- [Create Mint](../patterns/token-operations/create-mint.md) - Token deployment
- [Transfer Tokens](../patterns/token-operations/transfer-tokens.md) - Token operations

**Concepts:**
- [Account Model](../concepts/account-model.md) - Data vs programs
- [PDAs Explained](../concepts/pdas-explained.md) - Program-controlled addresses
