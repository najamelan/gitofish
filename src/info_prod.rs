use crate::{ import::*, CliArgs, Env, Info };


/// The version of Info to run in production. (A different one is used for testing).
//
#[ derive(Debug) ]
//
pub struct InfoProd
{
	arg: CliArgs,
	cfg: ConfigCrate,
	env: Env,
}


impl InfoProd
{
	/// Create a new info from cli, env and /etc/gitofish.yml.
	//
	pub fn new() -> Self
	{
		let arg = CliArgs::from_args();

		// Load our configuration file:
		//
		let mut cfg = ConfigCrate::default();

		// TODO: fix this.
		// cfg.merge( config_crate::File::with_name("/etc/gitofish.yml") )

		// 	.expect( "/etc/gitofish.yml must exist and must be a valid configuration file" )
		// ;

		cfg.merge( config_crate::File::with_name("/vagrant/conf/gitofish.yml") )

			.expect( "/vagrant/conf/gitofish.yml must exist and must be a valid configuration file" )
		;


		let env = envy::from_env::<Env>()

			.expect( "parsing environment should never fail." );


		Self{ arg, cfg, env }
	}
}



impl Info for InfoProd
{
	fn arg( &self ) -> &CliArgs
	{
		&self.arg
	}



	fn cfg( &self ) -> &ConfigCrate
	{
		&self.cfg
	}



	fn env( &self ) -> &Env
	{
		&self.env
	}
}


impl Default for InfoProd
{
	fn default() -> Self
	{
		Self::new()
	}
}
