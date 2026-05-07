pub fn log_message(ctx: Context<LogMessage>) -> ProgramResult {
    msg!("GM {}", ctx.accounts.authority.key().to_string());
    Ok(())
}

#[derive(Accounts)]
pub struct LogMessage<'info> {
    authority: AccountInfo<'info>,
}