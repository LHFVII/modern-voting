import { startAnchor} from 'solana-bankrun';
import { BankrunProvider } from 'anchor-bankrun';
import { PublicKey } from "@solana/web3.js";
import { Program } from "@coral-xyz/anchor";
import { ModernVoting } from "../target/types/modern_voting";
import { expect } from 'chai';
import * as anchor from "@coral-xyz/anchor";
 
const IDL = require("../target/idl/modern_voting.json");
const Voting = require("../target/types/modern_voting");

describe("Create a system account", async () => {
    it("Bankrun should be able to deploy the program", async () => {
        const programId = PublicKey.unique()
        const context = await startAnchor("",[{name:"modern_voting", programId: programId}],[])
        const provider = new BankrunProvider(context);
        const puppetProgram = new Program<ModernVoting>(IDL, provider);
        
        const [pollAddress] = PublicKey.findProgramAddressSync([Buffer.from("poll"), new anchor.BN(1).toArrayLike(Buffer, "le", 8)], puppetProgram.programId);

        await puppetProgram.methods.initialize(
            new anchor.BN(1),
            new anchor.BN(0),
            new anchor.BN(1821707382),
            "test-poll",
            "description",
        ).rpc()

        await puppetProgram.methods.proposeCandidate(new anchor.BN(1), "Pizza")
            .accounts({
            pollAccount: pollAddress
          })
            .rpc()

        await puppetProgram.methods.proposeCandidate(new anchor.BN(1), "Sushi")
            .accounts({
            pollAccount: pollAddress
          })
            .rpc()

        await puppetProgram.methods.vote(new anchor.BN(1),"Pizza").rpc()
        const [candidateOneAddress] = PublicKey.findProgramAddressSync([new anchor.BN(1).toArrayLike(Buffer, "le", 8),Buffer.from("Pizza") ], puppetProgram.programId);
        const [candidateTwoAddress] = PublicKey.findProgramAddressSync([new anchor.BN(1).toArrayLike(Buffer, "le", 8),Buffer.from("Sushi") ], puppetProgram.programId);
        const candidateOne = await puppetProgram.account.candidateAccount.fetch(candidateOneAddress);
        const candidateTwo = await puppetProgram.account.candidateAccount.fetch(candidateTwoAddress);
        expect(candidateOne.candidateVotes.toNumber()).equal(1);
        expect(candidateTwo.candidateVotes.toNumber()).equal(0);
    });
    
});