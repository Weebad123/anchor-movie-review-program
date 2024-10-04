use anchor_lang::prelude::*;

declare_id!("BcaZJEi6XqEJsjfAgnJr9rSqimCwde2H2kbaa2KLyrY4");

const MIN_RATING: u8 = 1;

const MAX_RATING: u8 = 5;

const MAX_TITLE_LENGTH: usize = 20;

const MAX_DESCRIPTION_LENGTH: usize = 50;

#[program]
pub mod anchor_movie_review_program {
    use super::*;
    // we're gonna implement the add_movie_review instruction, which will require a Context of type AddMovieReview.:
    // The instruction will require 3 additional args as instruction data provided by a reviewer: title, description and rating.
    // within the instruction logic, we'll populate the data of the new movie_review account with the instruction data.. also set
    // the reviewer field as the initializer account from the instruction context
    pub fn add_movie_review(
        ctx: Context<AddMovieReview>,
        title: String,
        description: String,
        rating: u8,
    ) -> Result<()> {
        // We require that the rating is between 1 and 5
        require!(rating >= MIN_RATING && rating <= MAX_RATING, MovieReviewError::InvalidRating);
        // We require that the title is not longer than 20 characters
        require!(title.len() <= MAX_TITLE_LENGTH, MovieReviewError::TitleTooLong);
        // We require that the description is not longer than 50 characters
        require!(description.len() <= MAX_DESCRIPTION_LENGTH, MovieReviewError::DescriptionTooLong);
        // We provide some logging messages of the movie created
        msg!("Movie Review Account Created");
        msg!("Title: {}", title);
        msg!("Description: {}", description);
        msg!("Rating: {}", rating);

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.reviewer = ctx.accounts.initializer.key();
        movie_review.title = title;
        movie_review.rating = rating;
        movie_review.description = description;
        Ok(())
    }


    // we're gonna implement the update_movie_review instruction with a context whose generic type is UpdateMovieReview
    // the instruction will require 3 additional args as instruction data provided by a reviewer: title, description and rating
    // within the instruction logic, we'll update the rating and description stored on the movie_review account.
    // while the title doesn't get used in the instruction function itself, we'll need it for account validation of movie_review in the next step
    pub fn update_movie_review(
        ctx: Context<UpdateMovieReview>,
        title: String,
        description: String,
        rating: u8,
    ) -> Result<()> {
        // let's log some messages of the update
        msg!("Movie review account space reallocated");
        msg!("Title: {}", title);
        msg!("Description: {}", description);
        msg!("Rating: {}", rating);

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.rating = rating;
        movie_review.description = description;

        Ok(())
    }


    // We're gonna implement the delete_movie_review instruction to close an existing movie_review account with a context whose generic type is DeleteMovieReview
    // and won't include any additional instruction data. Since we're only closing an account, we actually don't need any instruction logic inside the function's body
    // as the closing itself is handled by the Anchor close constraint in the generic type
    pub fn delete_movie_review(_ctx: Context<DeleteMovieReview>, title: String) -> Result<()> {
        // log the instruction purpose
        msg!("Movie review for {} deleted", title);
        Ok(())
    }

}


#[account]
#[derive(InitSpace)]// using this macro on the AccountStruct automatically calculates the INIT_SPACE constant which represents the space required for the account fields
pub struct MovieAccountState {
    pub reviewer: Pubkey,
    pub rating: u8,
    #[max_len(20)]
    pub title: String,
    #[max_len(50)]
    pub description: String,
}

const DISCRIMINATOR: usize = 8;

// We'll be doing some checks and throwing some custom errors in case those checks are not successful.. so let's implement an enum to handle the different errors types
// The #[error_code] macro will generate error types to be used as return types from our instruction handlers
#[error_code]
enum MovieReviewError {
    #[msg("Rating must be between 1 and 5")]
    InvalidRating,
    #[msg("Movie Title too long")]
    TitleTooLong,
    #[msg("Movie Description too long")]
    DescriptionTooLong,
}


//Now, we create the AddMovieReview struct that we used as the generic in the instruction's context. This struct will list the accounts
// the add_movie_review instruction requires . The ff macros is needed: "#[derive(Accounts)]", "#[instruction(...)]" macro to access instruction dta
// passed into the instruction and "#[account(...)]" macro to specify additional constraints
// The movie_review account is a PDA that needs to be initialized so we'll add the seeds(movie title and reviewer's public key) and bump constraints 
//as well as the init constraint with its required payer(reviewer) and space (account discriminator, reviewer's public key, rating, title, description) constraints.
#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct AddMovieReview<'info> {
    #[account(
        init,
        seeds = [title.as_bytes(), initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = DISCRIMINATOR + MovieAccountState::INIT_SPACE
    )]
    pub movie_review: Account<'info, MovieAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


// Now, we create the UpdateMovieReview struct to define the accounts that the update_movie_review instruction needs
// Since the movie_review account will have already been initialized by this point, we no longer need the init constraint.
// However, since the value of description may now be different, we need to use the realloc constraint to reallocate the space on the account.
// To use the realloc, we need mut, realloc::payer, and realloc::zero constraints. we still need seeds and bump constraints
// note that the realloc::zero is true cox the movie_review account may be updated multiple times either by shrinking or expanding the allocated space to the account.
#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct UpdateMovieReview<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), initializer.key().as_ref()],
        bump,
        realloc = DISCRIMINATOR + MovieAccountState::INIT_SPACE,
        realloc::payer = initializer,
        realloc::zero = true,
    )]
    pub movie_review: Account<'info, MovieAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}



// Now, let's implement the DeleteMovieReview struct
// we use the close constraint to specify we're closing the movie_review account and that the rent should be refunded to the initializer account.
// we also include the seeds and bump constraints for the movie_review account for validation.. the closing logic is then handled by Anchor
#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteMovieReview<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), initializer.key.as_ref()],
        bump,
        close = initializer
    )]
    pub movie_review: Account<'info, MovieAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}