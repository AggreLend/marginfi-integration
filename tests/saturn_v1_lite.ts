import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SaturnV1Lite } from "../target/types/saturn_v1_lite";
import {
  PublicKey,
  Transaction,
  SystemProgram,
  LAMPORTS_PER_SOL,
  SYSVAR_RENT_PUBKEY,
  ComputeBudgetProgram,
  sendAndConfirmTransaction,
  Keypair,
} from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import {MINT_SEED, SATURN_GROUP_SEED} from './constants/constants'
import {
  createAssociatedTokenAccountInstruction,
  createMint,
  mintTo
} from "@solana/spl-token";
import { getConfig, MarginfiClient } from "@mrgnlabs/marginfi-client-v2";


describe("saturn_v1_lite", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const METADATA_PROGRAM_ID = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
  const program = anchor.workspace.SaturnV1Lite as Program<SaturnV1Lite>;


  const apiBackendApiKey = Keypair.fromSecretKey(new Uint8Array([
    50, 219,  30, 153,  64, 101, 249, 167, 159, 252, 131,
     8, 114, 104, 101, 233, 118,  58,  18, 149, 149, 199,
    73, 197, 106, 253, 168, 218,  87, 143,  77,  25, 219,
    89,  73, 180,  31, 249,  42,  15,  35,   6,  68, 136,
   247,  65, 240,  38, 122, 193, 170, 116, 205, 100, 252,
    96, 175, 192,  45,  52, 233, 148, 234, 182
 ]))

  const signer = [71,146,124,110,52,27,200,71,176,62,239,184,147,191,204,60,101,38,183,182,10,200,203,110,29,18,188,71,241,51,254,151,174,252,231,101,18,242,132,191,130,116,218,107,54,224,207,37,230,193,51,44,83,123,130,217,7,162,5,114,168,71,169,134]
  let mint_key = new PublicKey("11111111111111111111111111111111");
  let user_mint_usdc_key = new PublicKey("11111111111111111111111111111111");
  



 const bondId = "7"


  it("Create USDC Mint", async () => {
  await program.provider.connection.requestAirdrop(apiBackendApiKey.publicKey, 20_000_000_000);
  await program.provider.connection.requestAirdrop(program.provider.publicKey, 20_000_000_000);
  const mint = await createMint(
    program.provider.connection,
    apiBackendApiKey,
    apiBackendApiKey.publicKey,
    null, // freeze authority (you can use null to disable)
    9, // 9 decimals (standard for most tokens)
  );
  mint_key = mint

  const ataUser = await anchor.utils.token.associatedAddress({
    mint: mint,
    owner: program.provider.publicKey,
  });
  user_mint_usdc_key = ataUser

  let transactionBefore = new Transaction();
  transactionBefore.add(
    createAssociatedTokenAccountInstruction(
      apiBackendApiKey.publicKey,
      ataUser, // ata
      program.provider.publicKey, // owner
      mint // mint
    )
  );


  let signature = await sendAndConfirmTransaction(program.provider.connection, transactionBefore, [apiBackendApiKey], {
    skipPreflight: true
  })

  let mintTx = await mintTo(
    program.provider.connection,
    apiBackendApiKey,
    mint,
    ataUser,
    apiBackendApiKey,
    1_000_000_000_000
  )

  console.log(mintTx)


  console.log(">>> Create Mint:", signature)
});


  it("Init Token", async () => {
    const [mintPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from(MINT_SEED)],
      program.programId
    );

    const [metadataPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        METADATA_PROGRAM_ID.toBuffer(),
        mintPDA.toBuffer(),
      ],
      METADATA_PROGRAM_ID
    );

    const tokenAccount = await anchor.utils.token.associatedAddress({
      mint: mintPDA,
      owner: program.provider.publicKey,
    });

    const ix = await program.methods
      .mintToken(
        new anchor.BN(LAMPORTS_PER_SOL * 1_000_000), // amount (e.g., 1 token with 9 decimals)
        "Saturn", // name
        "STF", // symbol
        "https://token-uri-s-example.vercel.app/uri.json" // uri
      )
      .accounts({
        mint: mintPDA,
        tokenAccount: tokenAccount,
        metadata: metadataPDA,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        tokenMetadataProgram: METADATA_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      }).instruction();

    const modifyComputeUnits = ComputeBudgetProgram.setComputeUnitLimit({ 
      units: 1000000 
    });

    
    
    const transaction = new Transaction()
    .add(modifyComputeUnits)
    .add(
        ix
      );
    
    try {
      let signature = await sendAndConfirmTransaction(program.provider.connection, transaction, [Keypair.fromSecretKey(Uint8Array.from(signer))], {
        skipPreflight: true
      } )

      console.log(">>> Init Mint", signature);
    } catch(e) {
      console.log(e)
    }

    // Add assertions here to verify the state after minting
  });

  it("Innit Treasury", async () => {
    // Add your test here.
    const tx = await program.methods.initTreasury().accounts({
      saturnApiKey: apiBackendApiKey.publicKey
    }).rpc({skipPreflight: true});
    console.log(">>> Init Treasurty: ", tx);
  });



  it("Stake STF", async () => {
    const [mintPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from(MINT_SEED)],
      program.programId
    );

    const [treasuryAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from(SATURN_GROUP_SEED)],
      program.programId
    );

    const userTokenAccount = await anchor.utils.token.associatedAddress({
      mint: mintPDA,
      owner: program.provider.publicKey,
    });

    const saturngroupTreasuryTokenAccount = await anchor.utils.token.associatedAddress({
      mint: mintPDA,
      owner: treasuryAccount,
    });

    // When a new instance is made there are errors if the saturn token account is not initialized but the following code is the fix.
    //  (Saves compute instead of having the init)

    let transactionBefore = new Transaction();
    transactionBefore.add(
      createAssociatedTokenAccountInstruction(
        program.provider.publicKey,
        saturngroupTreasuryTokenAccount, // ata
        treasuryAccount, // owner
        mintPDA // mint
      )
    );

    let signature = await sendAndConfirmTransaction(program.provider.connection, transactionBefore, [Keypair.fromSecretKey(Uint8Array.from(signer))], {
      skipPreflight: true
    } )

    console.log(">>> Token Account Init: ", signature);

    // Add your test here.
    const tx = await program.methods.stakeStf(new anchor.BN(1_000_000_000)).accounts(
      {
        userTokenAccount: userTokenAccount,
        treasuryTokenAccount: saturngroupTreasuryTokenAccount,
        stfTokenMint: mintPDA,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      }
    ).rpc({
      skipPreflight: true
    });
    console.log(">>> Stake STF: ", tx);
  });



  it("Unstake STF", async () => {

    const [mintPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from(MINT_SEED)],
      program.programId
    );

    const [treasuryAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from(SATURN_GROUP_SEED)],
      program.programId
    );

    const userTokenAccount = await anchor.utils.token.associatedAddress({
      mint: mintPDA,
      owner: program.provider.publicKey,
    });

    const saturngroupTreasuryTokenAccount = await anchor.utils.token.associatedAddress({
      mint: mintPDA,
      owner: treasuryAccount,
    });

    // Add your test here.
    const tx = await program.methods.unstakeStf(new anchor.BN(1_000_000_000)).accounts(
      {
        userTokenAccount: userTokenAccount,
        treasuryTokenAccount: saturngroupTreasuryTokenAccount,
        stfTokenMint: mintPDA,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      }
    ).rpc({
      skipPreflight: true
    });
    console.log(">>> Unstake STF: ", tx);
  });

  it("Update Bond Quote", async () => {
   

    await program.provider.connection.requestAirdrop(apiBackendApiKey.publicKey, 20_000_000_000);

    let transaction = new Transaction();
    // Add your test here.
    const ix = await program.methods.updateQuote(new anchor.BN(1_100_000_000), new anchor.BN(1_000_000_000)).accounts(
      {
        saturnApiKey: apiBackendApiKey.publicKey
      }
    ).instruction()
    transaction.add(ix)

    let signature = await sendAndConfirmTransaction(program.provider.connection, transaction, [apiBackendApiKey], {
      skipPreflight: true
    } )

    console.log(">>> Bond Quote: ", signature);
  });


  it("Create Bond ", async () => {


    const [treasuryAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from(SATURN_GROUP_SEED)],
      program.programId
    );



    const saturngroupTreasuryTokenAccount = await anchor.utils.token.associatedAddress({
      mint: mint_key,
      owner: treasuryAccount,
    });
    console.log(mint_key)
    // When a new instance is made there are errors if the saturn token account is not initialized but the following code is the fix.
    //  (Saves compute instead of having the init)
 
    let transactionBefore = new Transaction();
    transactionBefore.add(
      createAssociatedTokenAccountInstruction(
        program.provider.publicKey,
        saturngroupTreasuryTokenAccount, // ata
        treasuryAccount, // owner
        mint_key // mint
      )
    );
  
    let signature = await sendAndConfirmTransaction(program.provider.connection, transactionBefore, [Keypair.fromSecretKey(Uint8Array.from(signer))], {
      skipPreflight: true
    } )

    console.log(">>> Token Account Init: ", signature);
 

    // try {
    // Add your test here.
    const transaction = new Transaction()
    const tx = await program.methods.createBond(bondId, new anchor.BN(1_000_000_000)).accounts(
      {
        user: program.provider.publicKey,
        userTokenAccountUsdc: user_mint_usdc_key,
        treasuryTokenAccountUsdc: saturngroupTreasuryTokenAccount,
        usdcTokenMint: mint_key,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      }
    ).instruction()
      transaction.add(tx)
    let signature2 = await sendAndConfirmTransaction(program.provider.connection, transaction, [Keypair.fromSecretKey(Uint8Array.from(signer))], {
      skipPreflight: true
    } )
     console.log(">>> Create Bond: ", signature2);
    // } catch(e) {
    //   console.log(e)
    // }
  });


  it("Reedeem Bond", async () => {

    const [mintPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from(MINT_SEED)],
      program.programId
    );

    const [treasuryAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from(SATURN_GROUP_SEED)],
      program.programId
    );

    const userTokenAccount = await anchor.utils.token.associatedAddress({
      mint: mintPDA,
      owner: program.provider.publicKey,
    });

    const saturngroupTreasuryTokenAccount = await anchor.utils.token.associatedAddress({
      mint: mintPDA,
      owner: treasuryAccount,
    });

    // Add your test here.
    const tx = await program.methods.redeemBond(bondId).accounts(
      {
        userTokenAccount: userTokenAccount,
        treasuryTokenAccount: saturngroupTreasuryTokenAccount,
        stfTokenMint: mintPDA,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId
      }
    ).rpc({
      skipPreflight: true
    });
    console.log(">>> Unstake STF: ", tx);
  });


  it("Reedem Token!", async () => {


    const [treasuryAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from(SATURN_GROUP_SEED)],
      program.programId
    );



    const saturngroupTreasuryTokenAccount = await anchor.utils.token.associatedAddress({
      mint: mint_key,
      owner: treasuryAccount,
    });

    const userTokenAccountUSDC = await anchor.utils.token.associatedAddress({
      mint: mint_key,
      owner: program.provider.publicKey,
    });
    console.log(mint_key)

    const [mintPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from(MINT_SEED)],
      program.programId
    );


    const userTokenAccount = await anchor.utils.token.associatedAddress({
      mint: mintPDA,
      owner: program.provider.publicKey,
    });

    const saturngroupTreasuryTokenAccountToken = await anchor.utils.token.associatedAddress({
      mint: mintPDA,
      owner: treasuryAccount,
    });

    // try {
    // Add your test here.
    const transaction = new Transaction()
    const tx = await program.methods.redeemStf( new anchor.BN(1_000_000)).accounts(
      {
        userTokenAccount: userTokenAccount,
        treasuryTokenAccount: saturngroupTreasuryTokenAccountToken,
        user: program.provider.publicKey,
        userTokenAccountUsdc: userTokenAccountUSDC,
        treasuryTokenAccountUsdc: saturngroupTreasuryTokenAccount,
        usdcTokenMint: mint_key,
        tokenProgram: TOKEN_PROGRAM_ID,
        stfTokenMint: mintPDA,
        systemProgram: SystemProgram.programId,
      }
    ).instruction()
      transaction.add(tx)
    let signature2 = await sendAndConfirmTransaction(program.provider.connection, transaction, [Keypair.fromSecretKey(Uint8Array.from(signer))], {
      skipPreflight: true
    } )
     console.log(">>> Redeem Token:", signature2);
    // } catch(e) {
    //   console.log(e)
    // }
  });



  // it("Margin Fi Initiliaze", async () => {

  //   const [mintPDA] = PublicKey.findProgramAddressSync(
  //     [Buffer.from(MINT_SEED)],
  //     program.programId
  //   );

  //   const [treasuryAccount] = PublicKey.findProgramAddressSync(
  //     [Buffer.from(SATURN_GROUP_SEED)],
  //     program.programId
  //   );

  //   const userTokenAccount = await anchor.utils.token.associatedAddress({
  //     mint: mintPDA,
  //     owner: program.provider.publicKey,
  //   });

  //   const saturngroupTreasuryTokenAccount = await anchor.utils.token.associatedAddress({
  //     mint: mintPDA,
  //     owner: treasuryAccount,
  //   });

  //   // Add your test here.
  //   const tx = await program.methods.marginfiLendingInitialize().accounts(
  //     {
  //       marginfiAccount: "BagdC73FqyxfDBvN7FGWQuQtPsJ34DvVEgSsbH9syvGa",
  //       marginfiGroup: "BagdC73FqyxfDBvN7FGWQuQtPsJ34DvVEgSsbH9syvGa",
  //       marginfiProgram: "MFv2hWf31Z9kbCa1snEPYctwafyhdvnV7FZnsebVacA"
  //     }
  //   ).rpc({
  //     skipPreflight: true
  //   });
  //   console.log(">>> Unstake STF: ", tx);
  // });



});
