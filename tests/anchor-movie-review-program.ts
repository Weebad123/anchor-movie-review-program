import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import { AnchorMovieReviewProgram } from "../target/types/anchor_movie_review_program";

describe("anchor-movie-review-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorMovieReviewProgram as Program<AnchorMovieReviewProgram>;

  // Create default values for the movie review instruction data
  const movie = {
    title: "Just a test movie",
    description: "Wow what a good movie it was real great",
    rating: 5,
  };

// Derive the movie review account PDA
  const [moviePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(movie.title), provider.wallet.publicKey.toBuffer()],
    program.programId,
  );

  // Creating placeholders for tests
  it("Movie review is added", async() => {
    // Add the test here
    const tx = await program.methods
      .addMovieReview(movie.title, movie.description, movie.rating)
      .rpc();

    const account = await program.account.movieAccountState.fetch(moviePda);
    expect(movie.title == account.title);
    expect(movie.description == account.description);
    expect(movie.rating == account.rating);
    expect(account.reviewer == provider.wallet.publicKey);
  });

  it("Movie review is updated", async() => {
    // Add the updateMovie review instruction here
    const newDescription = "Wow this is new";
    const newRating = 4;

    const tx = await program.methods
      .updateMovieReview(movie.title, newDescription, newRating)
      .rpc();

    const account = await program.account.movieAccountState.fetch(moviePda);
    expect(movie.title == account.title);
    expect(newRating == account.rating);
    expect(newDescription == account.description);
    expect(account.reviewer == provider.wallet.publicKey);
  });

  it("Deletes a movie review", async() => {
    // Add the delete review test here
    const tx = await program.methods
      .deleteMovieReview(movie.title)
      .rpc();
  });


});
