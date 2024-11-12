pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("Cvsj4CpzRrf5Zb9BJ4CTELtWo2Watf3rabYUFGtoBNF6");

#[program]
pub mod swap {
    use super::*;

    pub fn make_offer(
        ctx: Context<MakeOffer>,
        id: u64,
        token_a_offered_amount: u64,
        token_b_wanted_amount: u64,
    ) -> Result<()> {
        instructions::send_offered_tokens_to_vault(&ctx, token_a_offered_amount)?;
        instructions::save_offer(ctx, id, token_b_wanted_amount)?;

        Ok(())
    }
}
