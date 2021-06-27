use crate::{ import::*, Info };


// TODO: actually implement functionality.
//
pub fn create( dir: &Path, _info: &impl Info ) -> Result<Repository, git2::Error>
{
	Repository::init( dir )
}
