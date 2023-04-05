use anchor_lang::prelude::*;
use anchor_lang::solana_program::account_info::AccountInfo;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
// use mpl_token_metadata::instruction as mpl_instruction;

declare_id!("7r9y16bKYXK7975aWT368s6jtnkhqiHCoa96M9URK9kC");

const GRID_SIZE: u16 = 50;

#[program]
pub mod seven_seas {
    use super::*;

    pub fn create_gold_token(
        ctx: Context<CreateGoldToken>,
        _token_title: String,
        _token_symbol: String,
        _token_uri: String,
        _token_decimals: u8,
    ) -> Result<()> {
        /*
               anchor_lang::solana_program::program::invoke(
                    &mpl_instruction::create_metadata_accounts_v3(
                        ctx.accounts.token_metadata_program.key(),      // Program ID (the Token Metadata Program)
                        ctx.accounts.metadata_account.key(),            // Metadata account
                        ctx.accounts.mint_account.key(),                // Mint account
                        ctx.accounts.mint_authority.key(),              // Mint authority
                        ctx.accounts.signer.key(),                       // Payer
                        ctx.accounts.mint_authority.key(),              // Update authority
                        token_title,                                    // Name
                        token_symbol,                                   // Symbol
                        token_uri,                                      // URI
                        None,                                           // Creators
                        0,                                              // Seller fee basis points
                        true,                                           // Update authority is signer
                        false,                                          // Is mutable
                        None,                                           // Collection
                        None,                                           // Uses
                        None,                                           // Collection Details
                    ),
                    &[
                        ctx.accounts.metadata_account.to_account_info(),
                        ctx.accounts.mint_account.to_account_info(),
                        ctx.accounts.mint_authority.to_account_info(),
                        ctx.accounts.signer.to_account_info(),
                        ctx.accounts.mint_authority.to_account_info(),
                        ctx.accounts.rent.to_account_info(),
                    ]
                )?;
        */

        ctx.accounts.mint_authority.bump = *ctx.bumps.get("mint_authority").unwrap();
        Ok(())
    }

    pub fn spawn_boat(ctx: Context<SpawnBoat>, x: u16, y: u16) -> Result<()> {
        assert!(
            x <= GRID_SIZE && y <= GRID_SIZE,
            "x and y must be under 100"
        );
        ctx.accounts.boat.owner = ctx.accounts.signer.key();
        ctx.accounts.boat.x = x;
        ctx.accounts.boat.y = y;
        ctx.accounts.boat.life = 100;
        ctx.accounts.boat.last_move = ctx.accounts.clock.unix_timestamp as f64;

        let bump = ctx.accounts.mint_authority.bump;

        let gold_amount = 1000;
        anchor_spl::token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::MintTo {
                    mint: ctx.accounts.mint_account.to_account_info(),
                    to: ctx.accounts.associated_token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
                &[&[b"gold-authority".as_ref(), &[bump]]],
            ),
            gold_amount,
        )?;

        Ok(())
    }

    pub fn move_boat(ctx: Context<MoveBoat>, x: u16, y: u16) -> Result<()> {
        // assert that x and y are under 100 and if not return an error
        assert!(x < GRID_SIZE && y < GRID_SIZE, "x and y must be under 100");
        // assert that the boat is owned by the signer
        assert!(
            ctx.accounts.boat.owner == ctx.accounts.signer.key(),
            "boat is not owned by signer"
        );
        // assert that the boat has not moved in the last 5 minutes
        assert!(
            ctx.accounts.clock.unix_timestamp as f64 - ctx.accounts.boat.last_move > /* 60.0 * */ 5.0,
            "boat has moved in the last 5 seconds"
        );
        /*
        // assert that the boat is moving in a straight line
        assert!(
            ctx.accounts.boat.x == x || ctx.accounts.boat.y == y,
            "boat must move in a straight line"
        );
        */
        // assert that the boat is moving by 1
        assert!(
            (ctx.accounts.boat.x as i16 - x as i16).abs() == 1
                || (ctx.accounts.boat.y as i16 - y as i16).abs() == 1,
            "boat must move by 1"
        );
        ctx.accounts.boat.x = x;
        ctx.accounts.boat.y = y;
        ctx.accounts.boat.last_move = ctx.accounts.clock.unix_timestamp as f64;

        Ok(())
    }

    pub fn attack_boat(ctx: Context<AttackBoat>) -> Result<()> {
        // assert that the boat is owned by the signer
        assert!(
            ctx.accounts.firing_boat.owner == ctx.accounts.signer.key(),
            "boat is not owned by signer"
        );
        // assert that the boat has not moved in the last 5 minutes
        assert!(
            ctx.accounts.clock.unix_timestamp as f64 - ctx.accounts.firing_boat.last_move
                > /* 60.0 * */ 5.0,
            "boat has moved in the last 5 minutes"
        );

        let dx = ctx.accounts.firing_boat.x as u16 - ctx.accounts.defending_boat.x as u16;
        let dy = ctx.accounts.firing_boat.y as u16 - ctx.accounts.defending_boat.y as u16;

        let distance = (dx.pow(2) + dy.pow(2)) as f32;
        let distance = distance.sqrt().round() as u16;

        assert!(distance <= 3, "boat is too far away to attack");

        let damage = 50 / distance as u16;

        ctx.accounts.defending_boat.life -= damage;
        ctx.accounts.firing_boat.last_move = ctx.accounts.clock.unix_timestamp as f64;

        Ok(())
    }

    /*
        pub fn pillage_boat(ctx: Context<PillageBoat>, randomness: [u8; 32]) -> Result<()> {

            // assert that the boat is owned by the signer
            assert!(
                ctx.accounts.attacking_boat.owner == ctx.accounts.signer.key(),
                "boat is not owned by signer"
            );
            // assert that the boat has not moved in the last 5 minutes
            assert!(
                ctx.accounts.clock.unix_timestamp as f64 - ctx.accounts.attacking_boat.last_move
                    > /* 60.0 * */ 5.0,
                "boat has moved in the last 5 seconds"
            );

            // assert that both boats are on the same coordinate and if not return an error
            assert!(
                ctx.accounts.attacking_boat.x == ctx.accounts.defending_boat.x
                    && ctx.accounts.attacking_boat.y == ctx.accounts.defending_boat.y,
                "boats are not on the same coordinate"
            );

            msg!("Running CPI");
            let cpi_program = ctx.accounts.vrf.to_account_info();
            msg!("CPI ACCOUNTS");
            let network_state_address = orao_solana_vrf::network_state_account_address();

            let cpi_accounts = orao_solana_vrf::cpi::accounts::Request {
                payer: ctx.accounts.signer.to_account_info(),
                network_state: ctx.accounts.config.to_account_info(),
                treasury: ctx.accounts.treasury.to_account_info(),
                request: ctx.accounts.random.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            };
            msg!("CPI CONTEXT");
            let cpi_ctx = anchor_lang::context::CpiContext::new(cpi_program, cpi_accounts);
            msg!("request cpi");
            orao_solana_vrf::cpi::request(cpi_ctx, randomness)?;
            msg!("request cpi done");
            let account_info = ctx.accounts.random.to_account_info();
                msg!("account info {:?}" , account_info);
            if account_info.data_is_empty() {
                // return an error if the account is not initialized
                msg!("Account not initialized");
            }
            let account = Randomness::try_deserialize(&mut &account_info.data.borrow()[..])?;
            let randomness = account.randomness;
            let value = randomness[0..size_of::<u64>()].try_into().unwrap();
            let result = u64::from_le_bytes(value) % 2 == 0;
            if result == true {
                ctx.accounts.defending_boat.life = ctx.accounts.defending_boat.life / 2;
            } else if result == false {
                ctx.accounts.attacking_boat.life = ctx.accounts.attacking_boat.life / 2;
            }

            Ok(())
        }
    */
}

#[derive(Accounts)]
pub struct CreateGoldToken<'info> {
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,
    #[account(
        init,
        payer = signer,
        mint::decimals = 6,
        mint::authority = mint_authority.key(),
    )]
    pub mint_account: Account<'info, Mint>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    /// CHECK: Metaplex will check this
    pub token_metadata_program: UncheckedAccount<'info>,
    #[account(
        init,
        seeds = [b"gold-authority".as_ref()],
        bump,
        payer = signer,
        space = 40,
    )]
    pub mint_authority: Account<'info, MintAuthority>,
}

#[derive(Accounts)]
pub struct SpawnBoat<'info> {
    #[account(
        mut,
        mint::decimals = 6,
        mint::authority = mint_authority.key(),
    )]
    pub mint_account: Account<'info, Mint>,
    #[account(mut)]
    pub mint_authority: Account<'info, MintAuthority>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint_account,
        associated_token::authority = signer,
    )]
    pub associated_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    #[account(
        init,
        seeds = [b"boat".as_ref(), signer.key().as_ref()],
        bump,
        payer = signer,
        space = 200
    )]
    pub boat: Account<'info, Boat>,
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct MoveBoat<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub boat: Account<'info, Boat>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct AttackBoat<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub firing_boat: Account<'info, Boat>,
    #[account(mut)]
    pub defending_boat: Account<'info, Boat>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

/*
#[derive(Accounts)]
#[instruction(randomness: [u8; 32])]
pub struct PillageBoat<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub attacking_boat: Account<'info, Boat>,
    #[account(mut)]
    pub defending_boat: Account<'info, Boat>,
    /// CHECK:
    #[account(
        mut,
        seeds = [RANDOMNESS_ACCOUNT_SEED.as_ref(), &randomness],
        bump,
        seeds::program = orao_solana_vrf::ID
    )]
    random: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    treasury: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [CONFIG_ACCOUNT_SEED.as_ref()],
        bump,
        seeds::program = orao_solana_vrf::ID
    )]
    config: Account<'info, NetworkState>,
    vrf: Program<'info, OraoVrf>,
    pub system_program: Program<'info, System>,
    // add accounts that are needed for orao vrf
    pub clock: Sysvar<'info, Clock>,
}
*/

#[account]
pub struct Boat {
    pub owner: Pubkey,
    pub x: u16,
    pub y: u16,
    pub life: u16,
    pub mint: Pubkey,
    pub last_move: f64,
}

#[account]
pub struct MintAuthority {
    pub bump: u8,
}
