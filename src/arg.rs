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


pub fn arg() -> &'static CliArgs
{
	static INSTANCE: Lazy< CliArgs > = Lazy::new( ||
	{
		CliArgs::from_args()
	});


	&INSTANCE
}
