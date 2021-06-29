use crate::{ import::*, CliArgs };


// TODO: actually implement functionality.
//
pub fn create( dir: &Path, _args: &CliArgs ) -> Result<Repository, git2::Error>
{
	Repository::init( dir )
}
