process.env.ANCHOR_LOG = "debug";

import { SystemProgram, PublicKey } from "@solana/web3.js";

import * as anchor from "@coral-xyz/anchor";
import { Entry } from "../target/types/entry";
import self from "./preset";

const program = anchor.workspace.Entry as anchor.Program<Entry>;
const provider = anchor.AnchorProvider.env();

anchor.setProvider(provider);
self.setENV(provider,program.programId);

const reqs={
    cpi:async()=>{
        const users=await self.init({balance:true});
        self.output.start(`System initialization`);

        const kingDataPda=self.getPDA([Buffer.from("king_data")],program.programId);

        const sign_init= await program.methods
        .init()
        .accounts({
            payer:users.root.pair.publicKey,
            //kingData: kingDataPda.pu,
            //systemProgram: SystemProgram.programId, // ✅ 必须传入
        })
        .signers([users.root.pair])
        .rpc()
        .catch((err)=>{
            self.output.hr("Got Error");
            console.log(err);
        });
        //await self.info.treasurystate();
        self.output.end(`Signature of "init": ${sign_init}`);

    },  
    direct:async()=>{

    },
}

describe("CPI call demo.",() => {
  it("Call KING method `start` via ENTRY `router`", async () => {
    await reqs.cpi();
    //await reqs.create(index+1,json);
  });
});
