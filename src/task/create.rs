use crate::import::*;


// TODO: actually implement functionality.
//
pub fn create( dir: &Path ) -> Result<Repository, git2::Error>
{
	Repository::init( dir )
}
