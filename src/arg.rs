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


/// Are we running from PRE_GIT?
//
pub fn is_pre_git() -> bool
{
	let args = &arg().positional;

	if args.is_empty()
	{
		false
	}

	else
	{
		args[0] == "PRE_GIT"
	}
}
