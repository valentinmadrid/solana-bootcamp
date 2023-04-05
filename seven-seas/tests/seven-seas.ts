import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SevenSeas } from "../target/types/seven_seas";
import { PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata";

function createKeypairFromFile(path: string): anchor.web3.Keypair {
  return anchor.web3.Keypair.fromSecretKey(
    Buffer.from(JSON.parse(require("fs").readFileSync(path, "utf-8")))
  );
}

describe("high-seas", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SevenSeas as Program<SevenSeas>;

  const mint = anchor.web3.Keypair.generate();

  const payer = createKeypairFromFile(
    require("os").homedir() + "/.config/solana/id.json"
  );
  const [boat] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("boat"), payer.publicKey.toBuffer()],
    program.programId
  );

  const defending = createKeypairFromFile(
    require("os").homedir() + "/.config/solana/testkeypair1.json"
  );
  const [defendingBoat] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("boat"), defending.publicKey.toBuffer()],
    program.programId
  );

  const mintAuthority = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("gold-authority")],
    program.programId
  )[0];
  console.log("Starting Tests....");

  it("Create Gold Token", async () => {
    const tokenUri =
      "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json";
    const metadataAddress = await anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mint.publicKey.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    )[0];

    const tx = await program.methods
      .createGoldToken("Pirate Gold s", "PR", tokenUri, 9)
      .accounts({
        metadataAccount: metadataAddress,
        mintAccount: mint.publicKey,
        mintAuthority: mintAuthority,
        signer: payer.publicKey,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      })
      .instruction();

    const tx2 = new anchor.web3.Transaction().add(tx);
    const sig = await anchor.web3.sendAndConfirmTransaction(
      anchor.getProvider().connection,
      tx2,
      [payer, mint]
    );
    console.log("Your transaction signature", sig);

    const acc = await program.account.mintAuthority.fetch(mintAuthority);
    console.log(acc.bump);
  });

  it("Spawn Boat", async () => {
    const acc = await program.account.mintAuthority.fetch(mintAuthority);
    console.log("mint auth bump", acc.bump);
    const associatedTokenAccountAddress =
      await anchor.utils.token.associatedAddress({
        mint: mint.publicKey,
        owner: payer.publicKey,
      });
    console.log(associatedTokenAccountAddress);

    const tx = await program.methods
      .spawnBoat(24, 24)
      .accounts({
        signer: payer.publicKey,
        boat: boat,
        associatedTokenAccount: associatedTokenAccountAddress,
        mintAccount: mint.publicKey,
        mintAuthority: mintAuthority,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .rpc();

    console.log("Your transaction signature", tx);
    const coordinates = await program.account.boat.fetch(boat);
    console.log("Your boat coordinates", coordinates.x, coordinates.y);
  });
  it("Spawn Defending Boat", async () => {
    const coordinates = await program.account.boat.fetch(boat);
    console.log("Your boat coordinates", coordinates.x, coordinates.y);
    const ix = await program.methods
      .spawnBoat(24, 25)
      .accounts({
        boat: defendingBoat,
        signer: defending.publicKey,
        associatedTokenAccount: await anchor.utils.token.associatedAddress({
          mint: mint.publicKey,
          owner: defending.publicKey,
        }),
        mintAccount: mint.publicKey,
        mintAuthority: mintAuthority,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([defending])
      .instruction();
    const tx = new anchor.web3.Transaction().add(ix);
    const sig = await anchor.web3.sendAndConfirmTransaction(
      anchor.getProvider().connection,
      tx,
      [defending]
    );
    console.log("Your transaction signature", sig);
  });

  it("Move Boat", async () => {
    const ix = await program.methods
      .moveBoat(25, 24)
      .accounts({
        boat: boat,
        signer: payer.publicKey,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([payer])
      .instruction();
    const tx = new anchor.web3.Transaction().add(ix);
    const sig = await anchor.web3.sendAndConfirmTransaction(
      anchor.getProvider().connection,
      tx,
      [payer]
    );
    console.log("Your transaction signature", sig);
  });

  it("Attack defending boat", async () => {
    setTimeout(function () {}, 10000);
    const ix = await program.methods
      .attackBoat()
      .accounts({
        firingBoat: boat,
        defendingBoat: defendingBoat,
        signer: payer.publicKey,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([payer])
      .instruction();
    const tx = new anchor.web3.Transaction().add(ix);
    const sig = await anchor.web3.sendAndConfirmTransaction(
      anchor.getProvider().connection,
      tx,
      [payer]
    );
    console.log("Your transaction signature", sig);
  });

  /*
  it("Pillage defending boat with VRF", async () => {
    const boatcoordinates = await program.account.boat.fetch(boat);
    console.log("Your boat coordinates", boatcoordinates.x, boatcoordinates.y);
    const defendingcoordinates = await program.account.boat.fetch(
      defendingBoat
    );
    console.log(
      "Defending boat coordinates",
      defendingcoordinates.x,
      defendingcoordinates.y
    );

    const treasury = anchor.web3.Keypair.generate();
    const randomness = anchor.web3.Keypair.generate().publicKey;
    const ix = await program.methods
      .pillageBoat([...randomness.toBuffer()])
      .accounts({
        attackingBoat: boat,
        defendingBoat: defendingBoat,
        config: networkStateAccountAddress(),
        treasury: networkStateAccountAddress(),
        vrf: vrf.programId,
        random: randomnessAccountAddress(randomness.toBuffer()),
        signer: payer.publicKey,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([payer])
      .rpc();
    const tx = new anchor.web3.Transaction().add(ix);
    const sig = await anchor.web3.sendAndConfirmTransaction(
      anchor.getProvider().connection,
      tx,
      [payer]
    );
    console.log("Your transaction signature", sig);
    
  });
  */
});
