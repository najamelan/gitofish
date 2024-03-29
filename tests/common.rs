#![ allow( dead_code, unused_imports ) ]


use
{
	std :: { process::Command, error::Error, path::PathBuf, fs, io::Write } ,
	tempdir :: { TempDir } ,
	libgitofish :: *,
};



pub type DynResult<T> = Result<T, Box<dyn Error + Send + Sync> >;

pub static COMMIT_MSG: &str = "SECURITY: New/Modified files appeared on server";


/// Creates 2 repositories, a remote and a local in a temp directory.
//
#[ derive( Debug ) ]
//
pub struct TempRepo
{
	pub local : PathBuf ,
	pub remote: PathBuf ,
	pub tmpdir: TempDir ,
	pub sub   : Option<Box< TempRepo >>,
}



impl TempRepo
{
	/// Create a new temporary directory and clone tests/data/simple in it.
	//
	pub fn new() -> DynResult<TempRepo>
	{
		let tmpdir = TempDir::new( "test_gitofish" )?;
		let source = PathBuf::from( "tests/data/simple/simple.git" );

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


		Ok( Self{ local, remote, tmpdir, sub: None } )
	}



	/// Get standard cli arguments corresponding to this repository.
	//
	pub fn args( &self ) -> CliArgs
	{
		CliArgs
		{
			branch: CliArgs::parse_ref( "deploy" )                           ,
			remote: self.remote.to_str().expect( "path.to_str" ).to_string() ,
			tree  : self.local.clone()                                       ,

			..CliArgs::default()
		}
	}



	/// Get the `git2::Repository` for this repo.
	//
	pub fn repo( &self ) -> DynResult< git2::Repository >
	{
		Ok( git2::Repository::open( &self.local )? )
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
	pub fn rename_file( self ) -> DynResult<Self>
	{
		let old = self.local.join( "file"         );
		let new = self.local.join( "file_renamed" );

		fs::rename( old, new )?;

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



	/// like base but modifies a file in the working directory.
	//
	pub fn add_sub( self ) -> DynResult<Self>
	{
		let source = PathBuf::from( "tests/data/sub/sub.git" );

		let remote = self.tmpdir.path().join( "sub_remote" );
		let local  = self.tmpdir.path().join( "simple/sub" );


		Command::new( "git" )

			.arg( "clone"           )
			.arg( "--bare"          )
			.arg( "--branch=deploy" )
			.arg( source            )
			.arg( &remote           )
			.status()?
		;


		Command::new( "git" )

			.arg( "submodule"  )
			.arg( "add"        )
			.arg( &remote      )
			.arg( &local       )
			.status()?
		;


		Command::new( "git" )

			.arg( "submodule"  )
			.arg( "set-branch" )
			.arg( "deploy"     )
			.arg( &local       )
			.status()?
		;


		Command::new( "git" )

			.arg( "submodule" )
			.arg( "update"    )
			.arg( "--init"    )
			.arg( &local      )
			.status()?
		;


		Command::new( "git" )

			.arg( "commit"        )
			.arg( "--message"     )
			.arg( "Add submodule" )
			.status()?
		;


		Ok( self )
	}
}
