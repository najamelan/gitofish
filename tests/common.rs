#![ allow( dead_code, unused_imports ) ]


use
{
	std :: { process::Command, error::Error, path::PathBuf, fs, io::Write } ,
	tempdir :: { TempDir } ,
};



pub type DynResult<T> = Result<T, Box<dyn Error + Send + Sync> >;



/// Creates 2 repositories, a remote and a local in a temp directory.
//
#[ derive( Debug ) ]
//
pub struct TempRepo
{
	pub local : PathBuf ,
	pub remote: PathBuf ,
	pub tmpdir: TempDir ,
}



impl TempRepo
{
	/// Create a new temporary directory and clone tests/data/simple in it.
	//
	pub fn new() -> DynResult<TempRepo>
	{
		let tmpdir = TempDir::new( "test_gitofish" )?;
		let source = PathBuf::from( "tests/data/simple" );

		let remote = tmpdir.path().join( "simple_remote" );
		let local  = tmpdir.path().join( "simple"        );


		Command::new( "git" )

			.arg( "clone"           )
			.arg( "--bare"          )
			.arg( "--branch=deploy" )
			.arg( source            )
			.arg( &remote           )
			.status()?
		;


		Command::new( "git" )

			.arg( "clone"           )
			.arg( "--branch=deploy" )
			.arg( &remote           )
			.arg( &local            )
			.status()?
		;


		Ok( Self{ local, remote, tmpdir } )
	}



	/// like base but modifies a file in the working directory.
	//
	pub fn change_file( self ) -> DynResult<Self>
	{
		let file = self.local.join( "file" );


		let mut file = fs::OpenOptions::new()

			.append(true)
			.open( file )?
		;


		writeln!( file, "A new line!" )?;

		Ok( self )
	}



	/// like base but modifies a file in the working directory.
	//
	pub fn new_file( self ) -> DynResult<Self>
	{
		let     name = self.local.join( "file2" );
		let mut file = fs::File::create( name )?;

		writeln!( file, "A new file!" )?;

		Ok( self )
	}
}
