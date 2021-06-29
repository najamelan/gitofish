#![ allow( dead_code, unused_imports ) ]


use
{
	std :: { process::Command, error::Error, path::PathBuf } ,
	tempdir :: { TempDir } ,
};



pub type DynResult<T> = Result<T, Box<dyn Error + Send + Sync> >;



// returns 2 paths, remote and checkout. In a temp directory.
// The temp dir will be deleted when dropped.
//
pub fn base() -> DynResult< (TempDir, PathBuf, PathBuf) >
{
	let tmpdir   = TempDir::new( "test_gitofish" )?;
	let source   = PathBuf::from( "tests/data/simple" );
	let remote   = tmpdir.path().join( "simple_remote" );
	let checkout = tmpdir.path().join( "simple" );

	Command::new( "git" )

		.arg( "clone"  )
		.arg( "--bare" )
		.arg( "--branch=deploy" )
		.arg( source   )
		.arg( &remote  )
		.status()?
	;

	Command::new( "git" )

		.arg( "clone"   )
		.arg( "--branch=deploy" )
		.arg( &remote   )
		.arg( &checkout )
		.status()?
	;

	Ok( (tmpdir, remote, checkout) )
}
