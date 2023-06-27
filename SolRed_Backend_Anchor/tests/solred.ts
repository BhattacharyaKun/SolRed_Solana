import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solred } from "../target/types/solred";
import { expect } from "chai";

describe("solred", () => 
{
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Solred as Program<Solred>;

  it("Create User!", async () => 
  {
    const [userAccountPDA] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("user_account"), provider.publicKey.toBuffer()], program.programId); 

    await program.methods.createUser("Memxor").rpc();

    const userAccount = await program.account.userAccount.fetch(userAccountPDA);

    expect(userAccount.username).eq("Memxor");
    expect(userAccount.posts.toNumber()).eq(0);

  });

  it("Create SSR!", async () => 
  {
    const SSRName = "Solana";
    const SSRType = "Tech";
    const [SSRAccountPDA] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("ssr_account"), Buffer.from(SSRName)], program.programId); 

    await program.methods.createSsr(SSRName, SSRType, false).rpc();

    const ssrAccount = await program.account.ssrAccount.fetch(SSRAccountPDA);

    expect(ssrAccount.ssrName).eq(SSRName);
    expect(ssrAccount.ssrType).eq(SSRType);
    expect(ssrAccount.postCounts.toNumber()).eq(0);
    expect(ssrAccount.private).eq(false);
  });

  it("Create Thread!", async () => 
  {
    const SSRName = "Solana";
    const ThreadName = "Test Solana";
    const ThreadContent = "My Content";
    const [SSRAccountPDA] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("ssr_account"), Buffer.from(SSRName)], program.programId); 
    const [ThreadAccountPDA] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("thread_account"), Buffer.from(ThreadName)], program.programId); 
    
    const ssrAccountBefore = await program.account.ssrAccount.fetch(SSRAccountPDA);

    await program.methods.createThread(SSRName, ThreadName, ThreadContent).rpc();

    const ssrAccount = await program.account.ssrAccount.fetch(SSRAccountPDA);
    const threadAccount = await program.account.threadAccount.fetch(ThreadAccountPDA);

    expect(ssrAccount.postCounts.toNumber()).eq(ssrAccountBefore.postCounts.toNumber() + 1);
    expect(threadAccount.threadName).eq(ThreadName);
    expect(threadAccount.threadContent).eq(ThreadContent);
    expect(threadAccount.upvotes.toNumber()).eq(0);
    expect(threadAccount.downvotes.toNumber()).eq(0);
    expect(threadAccount.repliesCount.toNumber()).eq(0);
  });

  it("Thread Upvote!", async () => 
  {
    const ThreadName = "Test Solana";
    const [ThreadAccountPDA] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("thread_account"), Buffer.from(ThreadName)], program.programId); 

    const threadAccountInitial = await program.account.threadAccount.fetch(ThreadAccountPDA);

    await program.methods.upvoteThread(ThreadName, false).rpc();

    const threadAccountAddingUV = await program.account.threadAccount.fetch(ThreadAccountPDA);

    expect(threadAccountAddingUV.upvotes.toNumber()).eq(threadAccountInitial.upvotes.toNumber() + 1);

    await program.methods.upvoteThread(ThreadName, true).rpc();

    const threadAccountRemovingUV = await program.account.threadAccount.fetch(ThreadAccountPDA);

    expect(threadAccountRemovingUV.upvotes.toNumber()).eq(threadAccountAddingUV.upvotes.toNumber() - 1);
  });

  it("Thread Downvote!", async () => 
  {
    const ThreadName = "Test Solana";
    const [ThreadAccountPDA] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("thread_account"), Buffer.from(ThreadName)], program.programId); 

    const threadAccountInitial = await program.account.threadAccount.fetch(ThreadAccountPDA);

    await program.methods.downvoteThread(ThreadName, false).rpc();

    const threadAccountAddingUV = await program.account.threadAccount.fetch(ThreadAccountPDA);

    expect(threadAccountAddingUV.downvotes.toNumber()).eq(threadAccountInitial.downvotes.toNumber() + 1);

    await program.methods.downvoteThread(ThreadName, true).rpc();

    const threadAccountRemovingDV = await program.account.threadAccount.fetch(ThreadAccountPDA);

    expect(threadAccountRemovingDV.downvotes.toNumber()).eq(threadAccountAddingUV.downvotes.toNumber() - 1);
  });

  it("Add Reply!", async () => 
  {
    const Reply = "Solana!!!";
    const ThreadName = "Test Solana";
    const [ThreadAccountPDA] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("thread_account"), Buffer.from(ThreadName)], program.programId); 
    const [ReplyAccountPDA] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("reply_account"), Buffer.from(Reply)], program.programId); 
    
    const threadAccountBefore = await program.account.threadAccount.fetch(ThreadAccountPDA);

    await program.methods.reply(ThreadName, Reply).rpc();

    const threadAccount = await program.account.threadAccount.fetch(ThreadAccountPDA);
    const replyAccount = await program.account.replyAccount.fetch(ReplyAccountPDA);

    expect(threadAccount.repliesCount.toNumber()).eq(threadAccountBefore.repliesCount.toNumber() + 1);
    expect(replyAccount.reply).eq(Reply);
    expect(replyAccount.upvotes.toNumber()).eq(0);
    expect(replyAccount.downvotes.toNumber()).eq(0);
  });

  it("Reply Upvote!", async () => 
  {
    const Reply = "Solana!!!";
    const [ReplyAccountPDA] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("reply_account"), Buffer.from(Reply)], program.programId);  

    const replyAccountInitial = await program.account.replyAccount.fetch(ReplyAccountPDA);

    await program.methods.upvoteReply(Reply, false).rpc();

    const replyAccount = await program.account.replyAccount.fetch(ReplyAccountPDA);

    expect(replyAccount.upvotes.toNumber()).eq(replyAccountInitial.upvotes.toNumber() + 1);

    await program.methods.upvoteReply(Reply, true).rpc();

    const replyAccountRemovingUV = await program.account.replyAccount.fetch(ReplyAccountPDA);

    expect(replyAccountRemovingUV.upvotes.toNumber()).eq(replyAccount.upvotes.toNumber() - 1);
  });

  it("Reply Downvote!", async () => 
  {
    const Reply = "Solana!!!";
    const [ReplyAccountPDA] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("reply_account"), Buffer.from(Reply)], program.programId);  

    const replyAccountInitial = await program.account.replyAccount.fetch(ReplyAccountPDA);

    await program.methods.downvoteReply(Reply, false).rpc();

    const replyAccount = await program.account.replyAccount.fetch(ReplyAccountPDA);

    expect(replyAccount.downvotes.toNumber()).eq(replyAccountInitial.downvotes.toNumber() + 1);

    await program.methods.downvoteReply(Reply, true).rpc();

    const replyAccountRemovingDV = await program.account.replyAccount.fetch(ReplyAccountPDA);

    expect(replyAccountRemovingDV.downvotes.toNumber()).eq(replyAccount.downvotes.toNumber() - 1);
  });

  it("Remove Reply!", async () => 
  {
    const Reply = "Solana!!!";
    const ThreadName = "Test Solana";
    const [ThreadAccountPDA] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("thread_account"), Buffer.from(ThreadName)], program.programId); 
    
    const threadAccountInitial = await program.account.threadAccount.fetch(ThreadAccountPDA);

    await program.methods.removeReply(ThreadName, Reply).rpc();

    const threadAccount = await program.account.threadAccount.fetch(ThreadAccountPDA);

    expect(threadAccount.repliesCount.toNumber()).eq(threadAccountInitial.repliesCount.toNumber() - 1);
  });

  it("Remove Thread!", async () => 
  {
    const SSRName = "Solana";
    const ThreadName = "Test Solana";
    const [SSRAccountPDA] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("ssr_account"), Buffer.from(SSRName)], program.programId); 
    
    const ssrAccountInitial = await program.account.ssrAccount.fetch(SSRAccountPDA);

    await program.methods.removeThread(SSRName, ThreadName).rpc();

    const ssrAccount = await program.account.ssrAccount.fetch(SSRAccountPDA);

    expect(ssrAccount.postCounts.toNumber()).eq(ssrAccountInitial.postCounts.toNumber() - 1);
  });

  it("Remove SSR!", async () => 
  {
    const SSRName = "Solana";

    await program.methods.removeSsr(SSRName).rpc();
  });

  it("Remove User!", async () => 
  {
    await program.methods.removeUser().rpc();
  });
});
