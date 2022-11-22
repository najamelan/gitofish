use crate::import::*;


static VERSION: &str = const_format::formatcp!( "version: {}, commit: {}", clap::crate_version!(), git_version!() );

// The next line contains an invisible Unicode character (U+2800) to allow a line of white-space after
// the program name + version in the about text. It also won't be removed by editors that trim trailing
// white-space.
// Since we use #[ clap( verbatim_doc_comment ) ] there is no post processing on the message, so we
// don't have line breaks in paragraphs so they work on any width terminal.
// TODO: should we add a --user option to manually set the git config user value? With fallback to the USER env var?
/// ⠀
/// Keep a directory in sync with a (remote) repository. The remote repository is the connection between the developer and the files checked out on the server. The remote can be installed on the same machine (eg. gitolite, a path uri) or actually be remote.
///
/// Gitofish is supposed to be called from the remote (eg. through ssh, or maybe `sudo -u file-owner gitofish...` if it is local) before a git operation (PRE_GIT trigger in gitolite). This way when the dev does a fetch and files have (maliciously) changed on the server, they will get the new commits.
///
/// Next it should be called again as a post-receive hook so when the dev has pushed new commits gitofish will pull them in (fast-forward only) and check out the new files.
///
/// Gitofish will do the following steps:
///
///   • commit any uncommitted changes in the working directory.
///   • if there are any, push force them to remote.
///   • try to pull --ff from remote.
///
/// All steps are recursive in the case of sub-modules.
///
/// This means that we never do merges on the checked out repository and the remote is always (forcefully) kept in sync with the working directory. The developer has to deal with any files that have changed in the deployed code by pulling them first, merging manually and pushing again.
///
/// Remote can be anything git understands and the current user has push and rewrite permissions to.
///
//
#[ derive( ClapParser, Debug, Default ) ]
#[ clap( verbatim_doc_comment, version = VERSION ) ]
//
pub struct CliArgs
{
	/// Which branch to use for the checkout. If not present, defaults to `deploy`. Gitofish will ignore and not touch any other branches.
	//
	#[ clap( short, long, default_value = "deploy", value_parser, verbatim_doc_comment ) ]
	//
	pub branch: String,


	/// Only used on initial repository clone. The path to a detached git dir. Will default to the working directory (as specified by --tree).
	//
	#[ clap( short, long, value_parser, verbatim_doc_comment ) ]
	//
	pub git_dir: Option<PathBuf>,


	/// Where to log debugging information. Will default to `/var/log/gitofish.log`. You can pass `stdout`.
	//
	#[ clap( short, long, value_parser, verbatim_doc_comment ) ]
	//
	pub log: Option<PathBuf>,


	/// Log level. Must be one of `trace`, `debug`, `info`, `warning`, `error`. Defaults to `info`.
	//
	#[ clap( short = 'e', long, verbatim_doc_comment ) ]
	//
	pub log_level: Option<LogLvl>,


	/// Only used on initial repository clone. The remote from which to clone.
	//
	#[ clap( short, long, verbatim_doc_comment ) ]
	//
	pub remote: String,


	/// The path to the checked out files.
	//
	#[ clap( short, long, value_parser, verbatim_doc_comment ) ]
	//
	pub tree: PathBuf,


	// /// Print version and exit.
	// //
	// #[ clap( short, long ) ]
	// //
	// pub version: bool,


	// The path to a script that will be run post checkout. This allows correcting permissions
	// on the checked out files. The script will run with pwd of --tree.
	//
	// #[ clap( short, long, parse(from_os_str) ) ]
	//
	// pub post_checkout: Option<PathBuf>,
}


impl CliArgs
{
	/// git2 will use `/refs/heads/branch` notation, but we don't burden the user with this.
	/// This will be used by clap to transform the branchname.
	//
	pub fn parse_ref( src: &str ) -> String
	{
		format!( "refs/heads/{}", src )
	}
}




/// The loglevel as passed in through the cli.
//
#[ derive(Debug, Copy, Clone) ]
//
pub enum LogLvl
{
	Trace,
	Debug,
	Info,
	Warning,
	Error,
}


impl std::str::FromStr for LogLvl
{
	type Err = String;

	fn from_str( input: &str ) -> Result<Self, <Self as std::str::FromStr>::Err>
	{
		match input
		{
			"Trace"   => Ok(Self::Trace  ) ,
			"Debug"   => Ok(Self::Debug  ) ,
			"Info"    => Ok(Self::Info   ) ,
			"Warning" => Ok(Self::Warning) ,
			"Error"   => Ok(Self::Error  ) ,
			_         => Err( "Invalid argument for log-level, must be one of Trace, Debug, Info, Warning, Error.".to_string() ) ,
		}
	}
}
