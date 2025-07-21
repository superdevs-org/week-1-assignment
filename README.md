Issue 3709

The `#[account(...)]` macro with associated_token:: doesn't work on mainnet/devnet if the ATA doesn't already exist because init_if_needed only work if the CPI associated_token::create


`
#[derive(Accounts)]
pub struct BuyNft<'info> {
    #[account(mut)]
    pub buyer_authority: Signer<'info>,

    #[account(mut)]
    pub nft_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = buyer_authority,
        associated_token::mint = nft_mint,
        associated_token::authority = buyer_authority,
    )]
    pub buyer_nft_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
`

`
let expected_ata = get_associated_token_address(&ctx.accounts.buyer_authority.key(), &ctx.accounts.nft_mint.key());
require!(expected_ata == ctx.accounts.buyer_nft_account.key(), CryptoVerseAIError::InvalidATA);

// Only create if it doesn’t exist
if ctx.accounts.buyer_nft_account.owner != &spl_token::ID {
    associated_token::create(
        CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            associated_token::Create {
                payer: ctx.accounts.buyer_authority.to_account_info(),
                associated_token: ctx.accounts.buyer_nft_account.to_account_info(),
                authority: ctx.accounts.buyer_authority.to_account_info(),
                mint: ctx.accounts.nft_mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            }
        )
    )?;
}
`