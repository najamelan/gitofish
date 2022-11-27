use crate::{ import::*, CliArgs, task };


// - Test if repository is managed by gitofish
// - Test if repository exists.
//   - if it doesn't, run create.
// - run refresh.
//
// At a high level:
// - from the tree, commit all and push to gitolite.
//   as no merges are allowed, this will stop any push from the dev if diverged.
// - We can exit non-zero as well to stop gitolite from continuing. We can send error
//   messages to the user.
//
// Steps:
// - verify directories:
//   valid:
//   - they don't exist, create,
//   - existing:
//     - content owner needs permissions to the tree and gitdir
//     - must be directories
//     - must either contain the repo or be empty
// - in tree git add all
// - in tree git push all --force
//
pub fn pre_git( args: &CliArgs ) -> anyhow::Result<()>
{
	let gitdir = args.gitdir.as_ref().map( AsRef::as_ref );

	let _span = error_span!
	(
		"In PRE_GIT",
		gitdir = format!( "{:?}", gitdir     ).as_str() ,
		tree   = format!( "{:?}", &args.tree ).as_str() ,
	)
	.entered();


	let mut repo = task::verify_dir( &args.tree, gitdir, args ).context( "verify directories in pre_git for: {args:?}" )?;

	task::refresh( &mut repo, args ).context( "Refresh repo failed" )?;

	Ok(())
}



