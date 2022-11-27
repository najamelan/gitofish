use crate::{ import::*, CliArgs };


// TODO: actually implement functionality.
//
pub fn create( tree: &Path, _git_dir: Option<&Path>, _args: &CliArgs ) -> anyhow::Result<Repository>
{
	Ok(Repository::init( tree )?)
}
