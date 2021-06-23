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

		settings.merge( config_crate::File::with_name("/etc/gitofish.yml") )

			.expect( "/etc/gitofish.yml must exist and must be a valid configuration file" )
		;

		settings
	});


	&INSTANCE
}
