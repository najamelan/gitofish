use crate::import::*;


#[ derive( StructOpt, Debug ) ]
//
pub struct CliArgs
{
	#[ structopt( short, long ) ]
	//
	pub version   : bool        ,
	pub positional: Vec<String> ,
}


pub fn arg() -> RwLockReadGuard< 'static, CliArgs >
{
	static INSTANCE: Lazy<RwLock< CliArgs >> = Lazy::new( ||
	{
		RwLock::new( CliArgs::from_args() )
	});


	INSTANCE.read().expect( "CliArgs lock poisoned" )
}
