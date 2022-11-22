// We can be called from PRE_GIT or post-receive
//
// Gitofish always works with one preconfigured branch (default=deploy). It ignores all other branches.
//
// In pre git we need to make sure that gitolite get's synced with the file system.
// In post receive we try to update the file system from gitolite, without merging.
//
// It all needs to work with submodules.
//
// On the server, in the ideal scenario, the working dir is always clean. As data only
// comes in through git, a non clean working dir means files have changed on the server,
// that is not by the dev pushing a deploy.
//
// exit codes. On PRE_GIT, we can stop the operation with a non-zero which will be propagated
// to the user.
// On post receive however, the data is already received and the remote client will always exit
// 0. We can print error messages however.
//
// Scenario:
// - User does fetch or push to gitolite. In PRE_GIT, using GL_OPTION_GTF_USER, GL_OPTION_GTF_GITDIR
//   and GL_OPTION_GTF_TREE, we verify the repository exists on disk and is initialized. If not it is
//   created in an empty directory with no commits.
//
// - If it does exist, require clean working dir or:
//   - we add all/commit-all
//   - run GL_OPTION_GTF_POST_CHECKOUT scripts (usually permission scripts)
//   - verify clean or commit
//   - and push-all --force to gitolite.
//
// - if this was a fetch operation, user will now receive the exact state of the files on disk.
//
// - if it is a push operation. gitolite will now proceed to receive the update. If pre git commited/pushed,
//   it will reject the push from the dev because remotes don't do merges. The developer however has merge
//   rights, so they can push merge commits as well as force push. When the dev force pushes without first
//   merging in the work from the server, gitolite will accept, but the checkout on disk will not. There is
//   never a rewrite of history on the checked out files. This is to prevent any tampering.
//
// - In post-receive. We first verify working dir is clean, otherwise commit-all again and push-force.
//   If files have changed since PRE_GIT, the pull --fast-forward will fail.
//
#![ forbid ( unsafe_code                        ) ]
#![ allow  ( clippy::suspicious_else_formatting ) ]

#![ warn
(
	anonymous_parameters          ,
	missing_copy_implementations  ,
	missing_debug_implementations ,
	missing_docs                  ,
	nonstandard_style             ,
	rust_2018_idioms              ,
	single_use_lifetimes          ,
	trivial_casts                 ,
	trivial_numeric_casts         ,
	unreachable_pub               ,
	unused_extern_crates          ,
	unused_qualifications         ,
	variant_size_differences      ,
)]

#![ allow( unused_imports, dead_code, missing_docs ) ]



use
{
	libgitofish::*,
	tracing :: { info } ,
	clap::Parser as _,
};




fn main()
{
	tracing_subscriber::fmt::Subscriber::builder()

		.with_timer( tracing_subscriber::fmt::time::LocalTime::rfc_3339() )
		// .json()
	   // .with_max_level(tracing::Level::TRACE)
	   // .with_env_filter( "trace,polling=warn,async_io=warn,async_std::warn" )
	   //.with_writer( writer )
	   .init()
	;

	let args = CliArgs::parse();

	println!( "args: {:#?}\n", args );


	// if let Some( "PRE_GIT" ) = info.arg().positional.first().map( |s| s.as_str() )
	// {
	// 	task::pre_git( &info );
	// }

	// else
	// {
	// 	task::post_receive();
	// }
}
