use anchor_lang::prelude::*;

declare_id!("DYDBwsWtfvSHLVNcHPkMjCNUrEZwEw2aYHh21U1e74BZ");

#[program]
pub mod solred {
    use super::*;

    pub fn create_user(ctx: Context<CreateUser>, username: String) -> Result<()> 
    {
        let user_account = &mut ctx.accounts.user_account;

        user_account.creator = ctx.accounts.user.key();
        user_account.username = username;
        user_account.posts = 0;

        Ok(())
    }

    pub fn create_ssr(ctx: Context<CreateSSR>, ssr_name: String, ssr_type: String, private: bool) -> Result<()> 
    {
        let ssr_account = &mut ctx.accounts.ssr_account;

        ssr_account.creator = ctx.accounts.creator.key();
        ssr_account.ssr_name = ssr_name;
        ssr_account.ssr_type = ssr_type;
        ssr_account.post_counts = 0;
        ssr_account.private = private;

        Ok(())
    }

    pub fn create_thread(ctx: Context<CreateThread>, _ssr_name: String, thread_name: String, thread_content: String) -> Result<()> 
    {
        let thread_account = &mut ctx.accounts.thread_account;
        let ssr_account = &mut ctx.accounts.ssr_account;

        ssr_account.post_counts = ssr_account.post_counts.checked_add(1).unwrap();

        thread_account.creator = ctx.accounts.creator.key();
        thread_account.thread_name = thread_name;
        thread_account.thread_content = thread_content;
        thread_account.upvotes = 0;
        thread_account.downvotes = 0;
        thread_account.replies_count = 0;

        Ok(())
    }

    pub fn upvote_thread(ctx: Context<UpvoteDownvoteThread>, _thread_name: String, remove_upvote: bool) -> Result<()> 
    {
        let thread_account = &mut ctx.accounts.thread_account;

        if remove_upvote
        {
            thread_account.upvotes = thread_account.upvotes.checked_sub(1).unwrap();
        }
        else
        {
            thread_account.upvotes = thread_account.upvotes.checked_add(1).unwrap();
        }

        Ok(())
    }

    pub fn downvote_thread(ctx: Context<UpvoteDownvoteThread>, _thread_name: String, remove_downvote: bool) -> Result<()> 
    {
        let thread_account = &mut ctx.accounts.thread_account;

        if remove_downvote
        {
            thread_account.downvotes = thread_account.downvotes.checked_sub(1).unwrap();
        }
        else
        {
            thread_account.downvotes = thread_account.downvotes.checked_add(1).unwrap();
        }

        Ok(())
    }

    pub fn reply(ctx: Context<AddReply>, _thread_name: String, reply: String) -> Result<()>
    {
        let thread_account = &mut ctx.accounts.thread_account;
        let reply_account = &mut ctx.accounts.reply_account;

        thread_account.replies_count = thread_account.replies_count.checked_add(1).unwrap();

        reply_account.creator = ctx.accounts.creator.key();
        reply_account.reply = reply;
        reply_account.upvotes = 0;
        reply_account.downvotes = 0;
        
        Ok(())
    }

    pub fn upvote_reply(ctx: Context<UpvoteDownvoteReply>, _reply: String, remove_upvote: bool) -> Result<()> 
    {
        let reply_account = &mut ctx.accounts.reply_account;

        if remove_upvote
        {
            reply_account.upvotes = reply_account.upvotes.checked_sub(1).unwrap();
        }
        else
        {
            reply_account.upvotes = reply_account.upvotes.checked_add(1).unwrap();
        }

        Ok(())
    }

    pub fn downvote_reply(ctx: Context<UpvoteDownvoteReply>, _reply: String, remove_downvote: bool) -> Result<()> 
    {
        let reply_account = &mut ctx.accounts.reply_account;

        if remove_downvote
        {
            reply_account.downvotes = reply_account.downvotes.checked_sub(1).unwrap();
        }
        else
        {
            reply_account.downvotes = reply_account.downvotes.checked_add(1).unwrap();
        }

        Ok(())
    }

    pub fn remove_reply(ctx: Context<RemoveReply>, _thread_name: String, _reply: String) -> Result<()>
    {
        let thread_account = &mut ctx.accounts.thread_account;


        thread_account.replies_count = thread_account.replies_count.checked_sub(1).unwrap();
        
        Ok(())
    }

    pub fn remove_thread(ctx: Context<RemoveThread>, _ssr_name: String, _thread_name: String) -> Result<()>
    {
        let ssr_account = &mut ctx.accounts.ssr_account;

        ssr_account.post_counts = ssr_account.post_counts.checked_sub(1).unwrap();
        
        Ok(())
    }

    pub fn remove_ssr(_ctx: Context<RemoveSSR>, _ssr_name: String) -> Result<()>
    {
        Ok(())
    }

    pub fn remove_user(_ctx: Context<RemoveUser>) -> Result<()>
    {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(username: String)]
pub struct CreateUser<'info> 
{
    #[account(init, payer = user, seeds = [b"user_account", user.key().as_ref()], bump, space = 8 + 32 + 8 + 4 + username.len())]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(ssr_name: String, ssr_type: String)]
pub struct CreateSSR<'info> 
{
    #[account(init, payer = creator, seeds = ["ssr_account".as_ref(), ssr_name.as_ref()], bump, 
    space = 8 + 32 + 16 + 4 + ssr_name.len() + 4 + ssr_type.len() + 1)]
    pub ssr_account: Account<'info, SSRAccount>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(ssr_name: String, thread_name: String, thread_content: String)]
pub struct CreateThread<'info> 
{
    #[account(mut, seeds = ["ssr_account".as_ref(), ssr_name.as_ref()], bump)]
    pub ssr_account: Account<'info, SSRAccount>,

    #[account(init, payer = creator, seeds = ["thread_account".as_ref(), thread_name.as_ref()], bump, 
    space = 8 + 32 + 4 + thread_name.len() + 4 + thread_content.len() + 8 + 8 + 8)]
    pub thread_account: Account<'info, ThreadAccount>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(thread_name: String)]
pub struct UpvoteDownvoteThread<'info> 
{
    #[account(mut, seeds = ["thread_account".as_ref(), thread_name.as_ref()], bump)]
    pub thread_account: Account<'info, ThreadAccount>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(thread_name: String, reply: String)]
pub struct AddReply<'info> 
{
    #[account(mut, seeds = ["thread_account".as_ref(), thread_name.as_ref()], bump)]
    pub thread_account: Account<'info, ThreadAccount>,

    #[account(init, payer = creator, seeds = ["reply_account".as_ref(), reply.as_ref()], bump, space = 8 + 32 + 4 + reply.len() + 8 + 8)]
    pub reply_account: Account<'info, ReplyAccount>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(reply: String)]
pub struct UpvoteDownvoteReply<'info> 
{
    #[account(mut, seeds = ["reply_account".as_ref(), reply.as_ref()], bump)]
    pub reply_account: Account<'info, ReplyAccount>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(thread_name: String, reply: String)]
pub struct RemoveReply<'info> 
{
    #[account(mut, seeds = ["thread_account".as_ref(), thread_name.as_ref()], bump)]
    pub thread_account: Account<'info, ThreadAccount>,

    #[account(mut, close = creator, constraint = reply_account.creator == creator.key(), seeds = ["reply_account".as_ref(), reply.as_ref()], bump)]
    pub reply_account: Account<'info, ReplyAccount>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(ssr_name: String, thread_name: String)]
pub struct RemoveThread<'info> 
{
    #[account(mut, seeds = ["ssr_account".as_ref(), ssr_name.as_ref()], bump)]
    pub ssr_account: Account<'info, SSRAccount>,

    #[account(mut, close = creator, constraint = thread_account.creator == creator.key(), seeds = ["thread_account".as_ref(), thread_name.as_ref()], bump)]
    pub thread_account: Account<'info, ThreadAccount>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(ssr_name: String)]
pub struct RemoveSSR<'info> 
{
    #[account(mut, close = creator, constraint = ssr_account.creator == creator.key(), seeds = ["ssr_account".as_ref(), ssr_name.as_ref()], bump)]
    pub ssr_account: Account<'info, SSRAccount>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct RemoveUser<'info> 
{
    #[account(mut, close = user, constraint = user_account.creator == user.key(), seeds = [b"user_account", user.key().as_ref()], bump)]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[account]
pub struct UserAccount
{
    pub creator: Pubkey,
    pub username: String,
    pub posts: u64
}

#[account]
pub struct SSRAccount
{
    pub creator: Pubkey,
    pub post_counts: u128,
    pub ssr_name: String,
    pub ssr_type: String,
    pub private: bool
}

#[account]
pub struct ThreadAccount
{
    pub creator: Pubkey,
    pub thread_name: String,
    pub thread_content: String,
    pub upvotes: u64,
    pub downvotes: u64,
    pub replies_count: u64
}

#[account]
pub struct ReplyAccount
{
    pub creator: Pubkey,
    pub reply: String,
    pub upvotes: u64,
    pub downvotes: u64
}