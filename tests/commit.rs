//! Test libgitofish::task::commit.
//!
//! commits all new changes.
//!
//! It returns `Result<RepoStatus, git2::Error>` where RepoStatus indicates whether there was new content.
//!
//! Scenarios:
//!
//!   - verify RepoStatus when no new changes.
//!   - verify RepoStatus when there are new changes.
//!   - verify the new changes are committed.
//!   - verify commit message and author.
//!
//! Tested:
//!
//! ✓ clean working dir
//! ✓ changed file
//! ✓ new file
//! ✓ deleted file
//! ✓ renamed file
//! - sub modules...
//!
mod common;

use
{
	common      :: * ,
	libgitofish :: { *, task::RepoStatus } ,
};



#[ test ]
//
fn no_changes() -> DynResult<()>
{
	let      tmp = TempRepo::new()?;
	let mut repo = tmp.repo()?;

	assert_eq!( Ok(RepoStatus::Clean), task::commit( &mut repo, &tmp.args() ) );

	Ok(())
}



#[ test ]
//
fn changed_file() -> DynResult<()>
{
	let tmp = TempRepo::new()?

		.change_file()?
	;

	let mut repo = tmp.repo()?;

	assert_eq!( Ok(RepoStatus::NewContent), task::commit( &mut repo, &tmp.args() ) );
	assert_eq!( Ok(RepoStatus::Clean     ), task::commit( &mut repo, &tmp.args() ) );

	let commit = repo.head()?.peel_to_commit()?;
	let author = commit.author();

	assert_eq!( Some( COMMIT_MSG ), commit.message() );
	assert_eq!( Some( "gitofish" ), author.name()    );
	// TODO: email

	Ok(())
}



#[ test ]
//
fn new_file() -> DynResult<()>
{
	let tmp = TempRepo::new()?

		.new_file()?
	;

	let mut repo = tmp.repo()?;

	assert_eq!( Ok(RepoStatus::NewContent), task::commit( &mut repo, &tmp.args() ) );
	assert_eq!( Ok(RepoStatus::Clean     ), task::commit( &mut repo, &tmp.args() ) );

	let commit = repo.head()?.peel_to_commit()?;
	let author = commit.author();

	assert_eq!( Some( COMMIT_MSG ), commit.message() );
	assert_eq!( Some( "gitofish" ), author.name()    );

	Ok(())
}



#[ test ]
//
fn rename_file() -> DynResult<()>
{
	let tmp = TempRepo::new()?

		.rename_file()?
	;

	let mut repo = tmp.repo()?;

	assert_eq!( Ok(RepoStatus::NewContent), task::commit( &mut repo, &tmp.args() ) );
	assert_eq!( Ok(RepoStatus::Clean     ), task::commit( &mut repo, &tmp.args() ) );

	let commit = repo.head()?.peel_to_commit()?;
	let author = commit.author();

	assert_eq!( Some( COMMIT_MSG ), commit.message() );
	assert_eq!( Some( "gitofish" ), author.name()    );

	Ok(())
}
