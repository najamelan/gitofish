//! Test libgitofish::task::commit.
//!
//! commits all new changes.
//!
//! It returns `Result<RefreshStatus, git2::Error>` where RefreshStatus indicates whether there was new content.
//!
//! Scenarios:
//!
//!   - verify RefreshStatus when no new changes.
//!   - verify RefreshStatus when there are new changes.
//!   - verify the new changes are committed.
//!   - verify commit message and author.
//!
//! Tested:
//!
//! - clean working dir
//! - changed file
//! - new file
//! - deleted file
//! - renamed file
//! - sub modules...
//!
mod common;

use
{
	common      :: * ,
	libgitofish :: { *, task::RefreshStatus } ,
	git2        :: { Repository } ,
};


#[ test ]
//
fn no_changes() -> DynResult<()>
{
	let tmp = TempRepo::new()?;


	let args = CliArgs
	{
		branch: CliArgs::parse_ref( "deploy" ),
		remote: tmp.remote.to_str().expect( "path.to_str" ).to_string(),
		tree  : tmp.local.clone(),

		..CliArgs::default()
	};


	let mut repo = Repository::open( &tmp.local )?;

	// std::thread::park();

	assert_eq!( Ok(RefreshStatus::Clean), task::commit( &mut repo, &args ) );


	Ok(())
}


#[ test ]
//
fn changed_file() -> DynResult<()>
{
	let tmp = TempRepo::new()?

		.change_file()?
	;


	let args = CliArgs
	{
		branch: CliArgs::parse_ref( "deploy" ) ,
		remote: tmp.remote.to_str().expect( "path.to_str" ).to_string(),
		tree  : tmp.local.clone(),

		..CliArgs::default()
	};


	let mut repo = Repository::open( &tmp.local )?;

	// std::thread::park();

	assert_eq!( Ok(RefreshStatus::NewContent), task::commit( &mut repo, &args ) );
	assert_eq!( Ok(RefreshStatus::Clean     ), task::commit( &mut repo, &args ) );

	// TODO: verfiy the commit and commit message... calling commit again should be clean now.


	Ok(())
}



#[ test ]
//
fn new_file() -> DynResult<()>
{
	let tmp = TempRepo::new()?

		.new_file()?
	;


	let args = CliArgs
	{
		branch: CliArgs::parse_ref( "deploy" ) ,
		remote: tmp.remote.to_str().expect( "path.to_str" ).to_string(),
		tree: tmp.local.clone(),
		..CliArgs::default()
	};

	let mut repo = Repository::open( &tmp.local )?;

	// std::thread::park();

	assert_eq!( Ok(RefreshStatus::NewContent), task::commit( &mut repo, &args ) );


	Ok(())
}
