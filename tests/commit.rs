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
	let (_tmp, remote, tree) = base()?;

	let args = CliArgs
	{
		branch: CliArgs::parse_ref( "deploy" ) ,
		remote: remote.to_str().expect( "path.to_str" ).to_string(),
		tree: tree.clone(),
		..CliArgs::default()
	};

	let mut repo = Repository::open( &tree )?;

	// std::thread::park();

	assert_eq!( Ok(RefreshStatus::Clean), task::commit( &mut repo, &args ) );


	Ok(())

}
