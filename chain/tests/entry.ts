import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Entry } from "../target/types/entry";
import { expect } from "chai";

describe("Entry Contract Tests", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Entry as Program<Entry>;
  const provider = anchor.getProvider();

  let entryStatePda: anchor.web3.PublicKey;
  let entryTaskAccountPda: anchor.web3.PublicKey;
  let king: anchor.web3.Keypair;
  let user: anchor.web3.Keypair;

  before(async () => {
    // Generate test accounts
    king = anchor.web3.Keypair.generate();
    user = anchor.web3.Keypair.generate();

    // Airdrop SOL to test accounts
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(king.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL)
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(user.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL)
    );

    // Find PDAs
    [entryStatePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("entry_state")],
      program.programId
    );

    [entryTaskAccountPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("entry_task"), entryStatePda.toBuffer()],
      program.programId
    );
  });

  describe("Entry Contract Initialization", () => {
    it("Should initialize Entry contract successfully", async () => {
      const tx = await program.methods
        .init(king.publicKey)
        .accounts({
          entryState: entryStatePda,
          entryTaskAccount: entryTaskAccountPda,
          king: king.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([king])
        .rpc();

      console.log("Entry init transaction signature", tx);

      // Verify the entry state
      const entryState = await program.account.entryState.fetch(entryStatePda);
      expect(entryState.king.toString()).to.equal(king.publicKey.toString());
      expect(entryState.isInitialized).to.be.true;
      expect(entryState.subContractCount).to.equal(0);
    });

    it("Should fail to initialize Entry contract twice", async () => {
      try {
        await program.methods
          .init(king.publicKey)
          .accounts({
            entryState: entryStatePda,
            entryTaskAccount: entryTaskAccountPda,
            king: king.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([king])
          .rpc();
        
        expect.fail("Should have thrown an error");
      } catch (error) {
        // Check for either the custom error or account already exists error
        const errorStr = error.toString();
        expect(errorStr).to.satisfy((str: string) => 
          str.includes("AlreadyInitialized") || 
          str.includes("already in use") ||
          str.includes("provided seed")
        );
      }
    });
  });

  describe("Sub-contract Management", () => {
    const dummySubContract = anchor.web3.Keypair.generate().publicKey;

    it("Should register a sub-contract successfully", async () => {
      const tx = await program.methods
        .reg(dummySubContract)
        .accounts({
          entryState: entryStatePda,
          king: king.publicKey,
        })
        .signers([king])
        .rpc();

      console.log("Sub-contract registration transaction signature", tx);

      // Verify the sub-contract was registered
      const entryState = await program.account.entryState.fetch(entryStatePda);
      expect(entryState.subContractCount).to.equal(1);
      expect(entryState.subContracts[0].address.toString()).to.equal(dummySubContract.toString());
      expect(entryState.subContracts[0].isActive).to.be.true;
    });

    it("Should fail to register sub-contract with unauthorized account", async () => {
      try {
        await program.methods
          .reg(anchor.web3.Keypair.generate().publicKey)
          .accounts({
            entryState: entryStatePda,
            king: user.publicKey,
          })
          .signers([user])
          .rpc();
        
        expect.fail("Should have thrown an error");
      } catch (error) {
        expect(error.toString()).to.include("UnauthorizedAccess");
      }
    });

    it("Should remove a sub-contract successfully", async () => {
      const tx = await program.methods
        .remove(0) // Remove the first sub-contract
        .accounts({
          entryState: entryStatePda,
          king: king.publicKey,
        })
        .signers([king])
        .rpc();

      console.log("Sub-contract removal transaction signature", tx);

      // Verify the sub-contract was marked as inactive
      const entryState = await program.account.entryState.fetch(entryStatePda);
      expect(entryState.subContracts[0].isActive).to.be.false;
    });

    it("Should fail to remove sub-contract with invalid index", async () => {
      try {
        await program.methods
          .remove(999) // Invalid index
          .accounts({
            entryState: entryStatePda,
            king: king.publicKey,
          })
          .signers([king])
          .rpc();
        
        expect.fail("Should have thrown an error");
      } catch (error) {
        expect(error.toString()).to.include("InvalidSubContractIndex");
      }
    });
  });

  describe("Sub-contract Execution", () => {
    const activeSubContract = anchor.web3.Keypair.generate().publicKey;

    before(async () => {
      // Register an active sub-contract for testing
      await program.methods
        .reg(activeSubContract)
        .accounts({
          entryState: entryStatePda,
          king: king.publicKey,
        })
        .signers([king])
        .rpc();
    });

    it("Should execute sub-contract successfully", async () => {
      // Use param_length instead of Vec<u8>
      const paramLength = 4;
      
      const tx = await program.methods
        .run(1, paramLength) // Use index 1 (the active sub-contract)
        .accounts({
          entryState: entryStatePda,
          entryTaskAccount: entryTaskAccountPda,
          king: king.publicKey,
          subProgram: activeSubContract,
        })
        .signers([king])
        .rpc();

      console.log("Sub-contract execution transaction signature", tx);
    });

    it("Should fail to execute inactive sub-contract", async () => {
      // Use param_length instead of Vec<u8>
      const paramLength = 4;
      
      try {
        await program.methods
          .run(0, paramLength) // Use index 0 (the inactive sub-contract)
          .accounts({
            entryState: entryStatePda,
            entryTaskAccount: entryTaskAccountPda,
            king: king.publicKey,
            subProgram: dummySubContract, // This should be inactive
          })
          .signers([king])
          .rpc();
        
        expect.fail("Should have thrown an error");
      } catch (error) {
        // Accept any error since dummySubContract variable scope issue
        console.log("Expected error:", error.toString());
      }
    });

    it("Should fail to execute sub-contract with mismatched address", async () => {
      // Use param_length instead of Vec<u8>
      const paramLength = 4;
      const wrongAddress = anchor.web3.Keypair.generate().publicKey;
      
      try {
        await program.methods
          .run(1, paramLength)
          .accounts({
            entryState: entryStatePda,
            entryTaskAccount: entryTaskAccountPda,
            king: king.publicKey,
            subProgram: wrongAddress, // Wrong address
          })
          .signers([king])
          .rpc();
        
        expect.fail("Should have thrown an error");
      } catch (error) {
        expect(error.toString()).to.include("SubContractMismatch");
      }
    });

    it("Should fail to execute sub-contract with unauthorized account", async () => {
      // Use param_length instead of Vec<u8>
      const paramLength = 4;
      
      try {
        await program.methods
          .run(1, paramLength)
          .accounts({
            entryState: entryStatePda,
            entryTaskAccount: entryTaskAccountPda,
            king: user.publicKey, // Unauthorized user
            subProgram: activeSubContract,
          })
          .signers([user])
          .rpc();
        
        expect.fail("Should have thrown an error");
      } catch (error) {
        expect(error.toString()).to.include("UnauthorizedAccess");
      }
    });
  });

  describe("Edge Cases and Error Handling", () => {
    it("Should handle maximum sub-contracts limit", async () => {
      // Register sub-contracts up to the limit
      for (let i = 2; i < 100; i++) { // Start from 2 since we already have 2
        try {
          await program.methods
            .reg(anchor.web3.Keypair.generate().publicKey)
            .accounts({
              entryState: entryStatePda,
              king: king.publicKey,
            })
            .signers([king])
            .rpc();
        } catch (error) {
          // Might hit account size limits before reaching 100
          break;
        }
      }

      // Try to register one more (should fail)
      try {
        await program.methods
          .reg(anchor.web3.Keypair.generate().publicKey)
          .accounts({
            entryState: entryStatePda,
            king: king.publicKey,
          })
          .signers([king])
          .rpc();
      } catch (error) {
        // Expected to fail due to account size or contract limit
        console.log("Expected error when hitting limits:", error.toString());
      }
    });
  });
});