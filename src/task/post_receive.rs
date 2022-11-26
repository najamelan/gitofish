use crate::{ import::*, CliArgs };

pub fn post_receive( _args: &CliArgs )
{
	info!( "In post-receive" );



}


pub fn pull_gitolite( repo: &mut Repository, args: &CliArgs ) -> Result<(), git2::Error>
{
	// If this is post-receive, we also do:
	//
	// - vc pull --ff-only gitolite deployed:deployed
	// - vc submodule update
	//
	// # We shouldn't overwrite changes on disk... although that has maybe been checked on pre_git.
	//
	//   FIXME: this shouldn't be necessary as update checks out.
	// - vc submodule foreach "git checkout deployed; git reset --hard HEAD@{1}"
	//
	// - after checkouts, need etckeeper init.
	// - if there are GL_OPTION_GTF_POST_CHECKOUT scripts, run them. In fish we always ran as sudo, but should be configurable probably.
	//
	// - commit-all
	// - push-all
	//
	let branch = &args.branch;

	let mut gitolite = repo.find_remote( "gitolite" )?;
	gitolite.fetch(&[ &branch ], None, None )?;

	let ref_spec   = format!( "gitolite/{}", branch );
	let reference  = repo.find_reference( &ref_spec )?;
	let annotated  = repo.reference_to_annotated_commit( &reference )?;
	let (merge, _) = repo.merge_analysis( &[ &annotated ] )?;

	if merge == git2::MergeAnalysis::ANALYSIS_FASTFORWARD
	{
		let mut r      = repo.find_reference( branch )?;
		let reflog_msg = format!( "Fast forward merge of gitolite/{} into {}", branch, branch );

		r.set_target( annotated.id(), &reflog_msg )?;

		// TODO: Don't know if this is necessary.
		//
		repo.set_head( branch )?;
		repo.checkout_head( None )?;


		// update all submodules.
		//
		for mut sub in repo.submodules()?
		{
			sub.update( true, None )?;
		}

		// TODO: verify etckeeper after checkout.


		// if let Some( path ) = args.post_checkout
		// {
		// 	let mut script = Command::new( path );

		// 	let result = script.status();

		// 	// TODO: handle error. This is the only not git2 error in this entire function...
		// 	// do we use anyhow?
		// }
	}

	Ok(())
}
