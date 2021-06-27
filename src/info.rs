use crate::{ import::*, CliArgs, Env };


/// Trait representing an object that has all the information from startup of the program.
///
/// That is:
/// - environment variables
/// - configuration files
/// - cli arguments
//
pub trait Info
{
	fn arg( &self ) -> &CliArgs;
	fn cfg( &self ) -> &ConfigCrate;
	fn env( &self ) -> &Env;



	/// Are we running from PRE_GIT?
	//
	fn is_pre_git( &self ) -> bool
	{
		let args = &self.arg().positional;

		if args.is_empty()
		{
			false
		}

		else
		{
			args[0] == "PRE_GIT"
		}
	}


	/// Are we running from post-receive?
	//
	fn is_post_receive( &self ) -> bool
	{
		! self.is_pre_git()
	}



	/// The branch on which we operate. Defaults to `deploy`.
	//
	fn branch( &self ) -> String
	{
		match self.cfg().get_str( "branch" )
		{
			Ok (b) => b,

			Err(e) =>
			{
				cfg_handle_error(e);

	 			// if handle_error didn't panic it was a NotFound.
	 			// We will set a default here.
	 			//
	 			"deploy".to_string()
			}
		}
	}



	/// Post checkout script.
	//
	fn post_checkout( &self ) -> Option< String >
	{
		self.env().gl_option_gtf_post_checkout.clone()
	}
}



// Handle errors from the config crate.
// Basically panics except for NotFound.
//
fn cfg_handle_error( error: ConfigError )
{
	match error
	{
		ConfigError::Frozen => unreachable!( "gitofish does not modify the config file." ),

		ConfigError::NotFound(_) => {},

		ConfigError::PathParse(ek) => panic!( "Configuration file could not be parsed: {:?}", ek ),
		ConfigError::FileParse{ uri, cause } => panic!( "Configuration file could not be parsed: file: {:?}, error: {}", uri, cause ),

		ConfigError::Type{ origin, key, expected, unexpected } =>

			panic!( "Configuration file could not be parsed, Value could not be converted in the requested type: file: {:?}, key: {:?}, expected: {:?}, got: {:?}", origin, key, expected, unexpected ),

		ConfigError::Message(msg) => panic!( "Configuration error: {}", msg ),

		ConfigError::Foreign(err) => panic!( "Configuration error: {}", err ),


	}
}
