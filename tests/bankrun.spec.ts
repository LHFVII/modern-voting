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
        console.log("Deploying the program");
        const client = context.banksClient;
        const payer = context.payer;
        const blockhash = context.lastBlockhash;

        const signature = await puppetProgram.methods.initialize(
            new anchor.BN(1),
            new anchor.BN(0),
            new anchor.BN(0),
            "test-poll",
            "description",
        ).rpc()

        console.log(JSON.stringify(await puppetProgram.account.pollAccount.fetch(pollAddress)));

        expect(1).equal(1);
    });
    
});