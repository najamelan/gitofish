//! This task makes sure a checked out repository is ready for interaction from gitolite.
//! This means that all modified files are commited and pushed to gitolite. Before doing
//! this repository, we will recurse into all submodules.
//!
//! Steps:
//!
//! - verify GL_OPTION_GTF_TREE, we should not get called without it set.
//! - if GL_OPTION_GTF_TREE is not an existing directory with git repo, we shouldn't do anything. Ideally, don't get called here either.
//!
//! - be recursive, that is first run refresh on submodules.
//!
//! - For PRE_GIT:
//!   - commit-all
//!   - push-all
//!
//! - For post-receive:
//!   - commit-all
//!   - push-all
//!
//!    - vc pull --ff-only gitolite deployed:deployed
//!   - vc submodule update
//!
//!     # We shouldn't overwrite changes on disk... although that has maybe been checked on pre_git.
//!   - vc submodule foreach "git checkout deployed; git reset --hard HEAD@{1}"
//!
//!   - after checkouts, need etckeeper init.
//!   - if there are GL_OPTION_GTF_POST_CHECKOUT scripts, run them. In fish we always ran as sudo, but should be configurable probably.
//!
//!   - commit-all
//!   - push-all
//!
//!
//! Where:
//!
//! commit-all =
//!   - vc checkout deploy
//!   - vc add --all
//!   - vc commit --message="SECURITY: New/Modified files appeared on server"
//!
//! push-all:
//!   - vc checkout deploy
//!   - vc push --force gitolite deployed:deployed
//!
use crate::{ import::*, CliArgs };


#[ derive( Copy, Clone, Debug, PartialEq, Eq, Hash ) ]
//
pub enum RefreshStatus
{
	Clean,
	NewContent,
}

impl RefreshStatus
{
	/// This allows to update the status. NewContent always has bigger weight.
	/// That is for status to be clean, both self and new_status must be clean.
	//
	fn merge( self, new: Self ) -> Self
	{
		if    self == RefreshStatus::NewContent
		   || new  == RefreshStatus::NewContent
		{
			RefreshStatus::NewContent
		}

		else
		{
			RefreshStatus::Clean
		}
	}
}

/// See module docs.
///
//  TODO: evaluate all the error handling. What should be handled here, what should go up the stack...
//
pub fn refresh( repo: &mut Repository, args: &CliArgs ) -> Result<RefreshStatus, git2::Error>
{
	let status = RefreshStatus::Clean;


	// Be recursive.
	//
	for sub in repo.submodules()?
	{
		let mut repo = sub.open()?;

		status.merge( refresh( &mut repo, args )? );
	}


	status.merge( commit( repo, args )? );

	// We push inconditionally, just in case. There might be commits from a prior run
	// that have not been pushed. So this just makes sure the remote is up to date.
	//
	push( repo, args )?;


	// In pre-git, when we committed new stuff we probably want to fail the operation here if it is a push
	// from dev. As they will have to fetch/merge first anyway.


	Ok( status )
}


pub fn commit( repo: &mut Repository, args: &CliArgs ) -> Result<RefreshStatus, git2::Error>
{
	// From git2 docs: "If the provided reference points to a branch, the HEAD will point to that branch, staying attached,
	// or become attached if it isn’t yet. If the branch doesn’t exist yet, no error will be returned.
	// The HEAD will then be attached to an unborn branch." <- TODO: whatever this means.
	//
	repo.set_head( &args.branch )?;

	// if the repository is clean, we don't need to do anything.
	//
	if repo.state() == git2::RepositoryState::Clean
	{
		return Ok( RefreshStatus::Clean );
	}


	// git add --all
	//
	let mut idx = repo.index()?;

	idx.add_all( ["."].iter(), git2::IndexAddOption::DEFAULT, None )?;


	let entry = idx.iter().next().unwrap();
	let path = std::ffi::CString::new(&entry.path[..]).unwrap();
	println!( "-- index not empty: len = {:?}, path: {:?}", &idx.len(), path );


	// TODO: more robust way of getting user name.
	//
	let usr = std::env::var( "USER" ).expect( "A user name to be in environment vars." );

	// TODO: audit hostname crate.
	// TODO: hostname returns os-str, check utf safety.
	//
	let sig = git2::Signature::now( "gitofish", &format!( "{}@{:?}", usr, hostname::get() ) )?;

	let msg = "SECURITY: New/Modified files appeared on server";

	let oid    = idx.write_tree()?;
	let tree   = repo.find_tree( oid )?;
	let head   = repo.head()?;
	let parent = head.peel_to_commit()?;


	// git commit --message="SECURITY: New/Modified files appeared on server"
	//
	repo.commit( Some( &args.branch ), &sig, &sig, &msg, &tree, &[&parent] )?;


	Ok( RefreshStatus::NewContent )
}



/// This pushes the branch configured for use by gitofish to the gitolite
/// remote.
///
/// TODO: probably want to panic a bit less.
//
pub fn push( repo: &mut Repository, args: &CliArgs ) -> Result<(), git2::Error>
{
	let mut remote = repo.find_remote( "gitolite" )?;

	// This is required to know whether the push succeeded.
	//
	let (tx, rx) = channel();
	let cb = move | _: &str, status: Option<&str> |
	{
		let resp = status.map( |s| s.to_string() );

		if tx.send( resp ).is_err()
		{
			panic!( "Failed to send on channel" );
		}

		Ok(())
	};

	let mut r_cbs  = git2::RemoteCallbacks::new();
	r_cbs.push_update_reference( cb );

	let mut p_opts = git2::PushOptions::new();
	p_opts.remote_callbacks( r_cbs );

	// NOTE: the '+' here means force push...
	//
	let refspec = format!( "+{}:{}", &args.branch, &args.branch );

	remote.push( &[ refspec ], Some(&mut p_opts) )?;

	if let Some(msg) = rx.recv().expect( "Receive on channel" )
	{
		panic!( "Push to gitolite failed with error: {}", msg );
	}

	Ok(())
}
