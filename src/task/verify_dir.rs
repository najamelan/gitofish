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
	// if git_dir is set
	//   if git_dir exists
	//     - does it have config core.worktree set to tree?
	//   else
	//     if tree exists, remove it
	//     call git clone
	//
	// if git_dir is not set
	//   if tree exists
	//     - do useful things
	//   else create
	//
	if let Some(g) = git_dir
	{
		if g.exists()
		{
			// Try to open the repository
			//
			let repo = Repository::open( g ).context( "Opening git dir as repo at: {g:?}" )?;

			// Assert config core.workdir points to tree.
			//
			let config   = repo.config().context( "Opening git config file for {g:?}" )?;
			let worktree = config.get_path( "core.worktree" ).context( "Read core.worktree from git config file for {g:?}" )?;

			if worktree != tree
			{
				return Err( anyhow!( "Git dir {g:?} has a core.worktree ({worktree:?}) set to a different path than gitolite configuration: ({tree:?})." ) );
			}

			// We assume that tree is in a decent state, and if it is not, well when we operate on it it will throw errors then.
		}

		// Clone
		// git clone --separate-git-dir git_dir tree
		// set core.worktree
		// if configured, remove .git in worktree for security reasons.
		// like everyone else, use git cli for ssh because libgit2 fails to use .ssh/config.
		// We actually control the ssh keys for gitofish, so it wouldn't be to hard in this
		// case, but also libgit2 rust port does not support --separate-git-dir.
		// TODO: test thoroughly and verify error messages are reasonable. eg. what if that branch does not exist?
		//
		let     remote   = format!( "{}@{}:{}", args.remote_user, args.gitolite_domain, args.repo );
		let mut separate = OsString::from( "--separate-git-dir=" );
		separate.push(g);

		Command::new( "git" )

			.arg( "clone"                               )
			.arg( "--recursive"                         )
			.arg( "--single-branch"                     )
			.arg( format!( "--branch={}", args.branch ) )
			.arg( separate                              )
			.arg( "--"                                  )
			.arg( &remote                               )
			.arg( &args.tree                            )
			.status()
			.context( format!( "cloning repo {}", args.repo ) )?
		;

		// Git clone does not set this automatically.
		//
		Command::new( "git" )
			.current_dir( g )
			.arg( "config"        )
			.arg( "core.worktree" )
			.arg( tree            )
			.status()
			.context( format!( "setting core.workree in newly cloned repo {:?}", g ) )?
		;

		// If asked, remove the .git entry in tree.
		//
		if args.remove_dot_git
		{
			std::fs::remove_file( tree.join(".git") )

				.context( format!( "Removing .git in tree: {tree:?}" ) )?
		}
	}


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
