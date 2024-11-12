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

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        
    }
}
