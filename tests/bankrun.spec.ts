import { startAnchor} from 'solana-bankrun';
import { BankrunProvider } from 'anchor-bankrun';
import { Program } from "@coral-xyz/anchor";
import { ModernVoting } from "../target/types/modern_voting";
import { expect } from 'chai';

const IDL = require("../target/idl/modern_voting.json");
const Voting = require("../target/types/modern_voting");

describe("Create a system account", async() => {
    test("Bankrun should be able to deploy the program", async () => {
        const context = await startAnchor("", [], []);
        const provider = new BankrunProvider(context);
        console.log("Deploying the program");
        expect(1).equal(1);
    });
    
});