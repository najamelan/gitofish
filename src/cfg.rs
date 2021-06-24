use crate::import::*;


/// Get the configuration.
///
/// This is parsed from /etc/gitofish.yml which must be present and valid.
///
pub fn cfg() -> &'static ConfigCrate
{
	static INSTANCE: Lazy< ConfigCrate > = Lazy::new( ||
	{
		// Load our configuration file:
		//
		let mut settings = ConfigCrate::default();

		// settings.merge( config_crate::File::with_name("/etc/gitofish.yml") )

		// 	.expect( "/etc/gitofish.yml must exist and must be a valid configuration file" )
		// ;

		settings.merge( config_crate::File::with_name("/vagrant/conf/gitofish.yml") )

			.expect( "/vagrant/conf/gitofish.yml must exist and must be a valid configuration file" )
		;

		settings
	});


	&INSTANCE
}



/// The branch on which we operate. Defaults to `deploy`.
//
pub fn branch() -> String
{
	match cfg().get_str( "branch" )
	{
		Ok (b) => b,

		Err(e) =>
		{
			handle_error(e);

 			// if handle_error didn't panic it was a NotFound.
 			// We will set a default here.
 			//
 			"deploy".to_string()
		}
	}
}



// Basically panics except for NotFound.
//
fn handle_error( error: ConfigError )
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
