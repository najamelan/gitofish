use crate::{ import::*, CliArgs, task };


// TODO: actually implement functionality.
//
pub fn verify_dir( dir: &Path, git_dir: Option< &Path >, args: &CliArgs ) -> Result<Repository, Box<dyn std::error::Error> >
{
	// We will clone again, so clear git_dir as well.
	//
	if dir.exists() && !dir.is_dir()
	{
		remove_path( dir )?;

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
				task::create( &dir, args )?
			}

			// it's not a repo, and it's not empty.
			//
			else
			{
				todo!()
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
