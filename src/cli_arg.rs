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

