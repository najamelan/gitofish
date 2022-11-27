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
// - verify directories:
//   valid:
//   - they don't exist, create,
//   - existing:
//     - content owner needs permissions to the tree and gitdir
//     - must be directories
//     - must either contain the repo or be empty
//     - if directory and not empty but doesn't have gitolite remote, add.
//     - gitdir must correspond to the setting inside of tree
//       eg in tree, .git file with `gitdir: /path/to/repo.git`
//       in repo, set config `core.worktree` to know where worktree is.
//
//  With the knowledge above, you can even set up git version control for an working directory without having write permissions. If you either use --git-dir on every git command or execute every command from within the repository (instead of the working directory), you can leave out the .git file and therefore do not need to create any files within the working directory. See also Leos answer
//
// We will do this, run every command in gitdir, that way we don't need to put a .git file in the tree.
// Unfortunately .gitignore is still necessary.
//
#[allow(clippy::needless_return)]
//
pub fn verify_dir( tree: &Path, git_dir: Option< &Path >, args: &CliArgs ) -> anyhow::Result<Repository>
{
	// If it exists but it's not a directory, for now error.
	//
	if tree.exists() && !tree.is_dir()
	{
		return Err( anyhow!("tree: {tree:?} exists but is not a directory. Please inspect the file and remove manually or choose a different path." ) );
	}

	// If it exists but it's not a directory, for now error.
	//
	if let Some(g) = git_dir {
	if g.exists() && !g.is_dir()
	{
		return Err( anyhow!( "git_dir: {g:?} exists but is not a directory. Please inspect the file and remove manually or choose a different path." ) );
	}}


	// tree doew not exist
	//
	if !tree.exists()
	{
		return task::create( tree, git_dir, args );
	}


	// tree exists and is a directory.
	//
	let uri = format!( "{}@localhost", std::env::var( "SUDO_USER" ).as_ref().expect( "sudo_user" ) );

	let repo =

		// if it's a repository
		//
		if let Ok(repo) = Repository::open( tree )
		{
			repo
		}

		// it's an empty directory.
		//
		else if tree.read_dir()?.next().is_none()
		{
			task::create( tree, git_dir, args )?
		}

		// it's not a repo, and it's not empty.
		//
		else
		{
			return Err( anyhow!( "Cannot use {tree:?} for ") );
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
