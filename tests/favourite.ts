import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Favourite } from "../target/types/favourite";

describe("favourite program test", () => {

  
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.Favourite as Program<Favourite>;

  it("Sets favourite values successfully", async () => {

    const user = provider.wallet.publicKey;

    
    const [favouritePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("IITROORKEE"),
        user.toBuffer(),
      ],
      program.programId
    );

    console.log("Favourite PDA:", favouritePda.toBase58());

    
    const favouriteNumber = new anchor.BN(99);
    const favouriteColour = "Blue";
    const favouriteHobbies = ["Coding", "Gym"]; 
    const favouriteAnimal = "Tiger";

    
    await program.methods
      .setFavourite(
        favouriteNumber,
        favouriteColour,
        favouriteHobbies,
        favouriteAnimal
      )
      .accounts({
        user: user,
        favouriteAccount: favouritePda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    
    const account = await program.account.favouriteData.fetch(favouritePda);

    console.log("Fetched account:", account);

    
    expect(account.number.toNumber()).to.equal(99);
    expect(account.colour).to.equal("Blue");
    expect(account.hobbies).to.deep.equal(["Coding", "Gym"]);
    expect(account.animal).to.equal("Tiger");
  });
});
