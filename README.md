Issue 3709

The `#[account(...)]` macro with associated_token:: doesn't work on mainnet/devnet if the ATA doesn't already exist because init_if_needed only work if the CPI associated_token::create
``
#[derive(Account)]
``