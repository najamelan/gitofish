use crate::{ import::*, CliArgs, Env, Info };


/// The version of Info to run during tests. This starts of with empty values and
/// allows mut access to set them to the exact input desired for tests.
//
#[ derive(Debug) ]
//
pub struct InfoTest
{
	arg: CliArgs,
	cfg: ConfigCrate,
	env: Env,
}


impl InfoTest
{
	/// Create a new info with empty inputs.
	//
	pub fn new() -> Self
	{
		let arg = CliArgs     :: default();
		let cfg = ConfigCrate :: default();
		let env = Env         :: default();

		Self{ arg, cfg, env }
	}


	/// Get mut access to the cli argument list.
	//
	pub fn arg_mut( &mut self ) -> &mut CliArgs
	{
		&mut self.arg
	}


	/// Get mut access to the configuration.
	//
	pub fn cfg_mut( &mut self ) -> &mut ConfigCrate
	{
		&mut self.cfg
	}


	/// Get mut access to the environment variables.
	//
	pub fn env_mut( &mut self ) -> &mut Env
	{
		&mut self.env
	}
}



impl Info for InfoTest
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


impl Default for InfoTest
{
	fn default() -> Self
	{
		Self::new()
	}
}
