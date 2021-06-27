use crate::{ import::*, env, arg, cfg, task };

pub fn pre_git()
{
	let gitdir = env().gl_option_gtf_gitdir.as_ref();
	let tree   = env().gl_option_gtf_tree.as_ref();
	let user   = env().gl_option_gtf_user.as_ref();

	let _span = error_span!
	(
		"In PRE_GIT",
		gitdir = format!( "{:?}", gitdir ).as_str() ,
		tree   = format!( "{:?}", tree   ).as_str() ,
		user   = format!( "{:?}", user   ).as_str() ,
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
	let dir = match env().gl_option_gtf_tree.as_ref()
	{
		// This repository is not managed by gitofish. However we get called for every repo
		// in gitolite.
		//
		None    => return       ,
		Some(d) => Path::new(d) ,
	};

	let git_dir = env().gl_option_gtf_tree.as_ref().map( |p| Path::new(p) );


	let mut repo = task::verify_dir( &dir, git_dir ).expect( "verify dir" );

	// TODO: Error handling.
	//
	task::refresh( &mut repo ).expect( "Refresh repo failed" );
}



