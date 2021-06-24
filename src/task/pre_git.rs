use crate::{ import::*, env, arg, cfg };

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
}

