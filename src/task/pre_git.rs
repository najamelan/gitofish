use crate::{ import::*, CliArgs, task };

pub fn pre_git( args: &CliArgs )
{
	let gitdir = args.git_dir.as_ref().map( |p| p.as_ref() );

	let _span = error_span!
	(
		"In PRE_GIT",
		gitdir = format!( "{:?}", gitdir     ).as_str() ,
		tree   = format!( "{:?}", &args.tree ).as_str() ,
	)
	.entered();


	// - Test if repository is managed by gitofish
	// - Test if repository exists.
	//   - if it doesn't, run create.
	// - run refresh.
	//
	// Scenarios:
	// - path isn't a directory -> remove it, remove git_dir as well. OK
	// - path doesn't exist -> clone into it.
	// - path is a dir but not empty.
	//   - not a repository -> git init in it and set gitolite as remote.
	//   - path already exists and is a repository but it holds the wrong repo/remote.
	//   - path already exists and is a repository
	//
	//
	// let dir = match tree
	// {
	// 	// This repository is not managed by gitofish. However we get called for every repo
	// 	// in gitolite.
	// 	//
	// 	None    => return       ,
	// 	Some(d) => Path::new(d) ,
	// };


	let mut repo = task::verify_dir( &args.tree, gitdir, args ).expect( "verify dir" );

	// TODO: Error handling.
	//
	task::refresh( &mut repo, args ).expect( "Refresh repo failed" );
}



