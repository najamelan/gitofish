use crate::{ import::*, CliArgs, task };


// Verify directory conditions for the dir (worktree) and git_dir (normally .git inside dir, but can be detached).
// Scenarios for both dir and git_dir:
// - doesn't exist
// - we don't have permissions
// - exists but is not a directory
// - exists but is empty
// - exists but is not empty and not a repository
// - exists and is repository but does not have the remote.
//
pub fn verify_dir( dir: &Path, git_dir: Option< &Path >, args: &CliArgs ) -> Result<Repository, Box<dyn std::error::Error> >
{
	// If it exists but it's not a directory, remove it.
	//
	if dir.exists() && !dir.is_dir()
	{
		remove_path( dir )?;

		// We will clone again, so clear git_dir as well.
		//
		if let Some(d) = git_dir
		{
			remove_path( d )?;
		}
	}


	if !dir.exists()
	{
		// return
		//
		Ok( task::create( dir, args )? )
	}


	// It exists.
	//
	else
	{
		let uri = format!( "{}@localhost", std::env::var( "SUDO_USER" ).as_ref().expect( "sudo_user" ) );

		let repo =

			// if it's a repository
			//
			if let Ok(repo) = Repository::open( dir )
			{
				repo
			}

			// it's an empty directory.
			//
			else if dir.read_dir()?.next().is_none()
			{
				task::create( dir, args )?
			}

			// it's not a repo, and it's not empty.
			//
			else
			{
				Err("Cannot use {dir} for ")?
			}
		;


		// verify it has gitolite remote
		//
		if !repo.remotes().expect( "list remotes" ).iter().any( |n| n == Some("gitolite") )
		{
			repo.remote( "gitolite", &uri )?;
		}

		Ok( repo )
	}
}



pub fn remove_path( path: &Path ) -> std::io::Result<()>
{
	if path.exists()
	{
		if path.is_dir()
		{
			std::fs::remove_dir_all( path )?;
		}

		else
		{
			std::fs::remove_file( path )?;
		}
	}

	Ok(())
}
