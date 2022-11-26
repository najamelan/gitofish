use crate::import::*;


static VERSION: &str = const_format::formatcp!( "version: {}, commit: {}", clap::crate_version!(), git_version!() );

/// Keep a directory in sync with a (remote) repository. The remote repository is the connection
/// between the developer and the files checked out on the server. The remote can be installed
/// on the same machine (eg. gitolite, a path uri) or actually be remote.
///
/// Gitofish is supposed to be called from the remote (eg. through ssh, or maybe
/// `sudo -u file-owner gitofish...` if it is local) before a git operation (PRE_GIT trigger in
/// gitolite). This way when the dev does a fetch and files have (maliciously) changed on the
/// server, they will get the new commits.
///
/// Next it should be called again as a post-receive hook so when the dev has pushed new commits
/// gitofish will pull them in (fast-forward only) and check out the new files.
///
/// Gitofish will do the following steps:
///
///   • commit any uncommitted changes in the working directory.
///   • if there are any, push force them to remote.
///   • try to pull --ff from remote.
///
/// All steps are recursive in the case of sub-modules.
///
/// This means that we never do merges on the checked out repository and the remote is always
/// (forcefully) kept in sync with the working directory. The developer has to deal with any
/// files that have changed in the deployed code by pulling them first, merging manually and
/// pushing again.
///
/// Remote can be anything git understands and the current user has push and rewrite permissions
/// to.
///
//
#[ derive( ClapParser, Debug ) ]
#[ command( author, name="gitofish", verbatim_doc_comment, version = VERSION ) ]
//
pub struct CliArgs
{
	/// Which branch to use for the checkout. If not present, defaults to `deploy`.
	/// Gitofish will ignore and not touch any other branches.
	//
	#[ arg( short, long, default_value = "deploy", value_parser, verbatim_doc_comment ) ]
	//
	pub branch: String,


	/// Only used on initial repository clone. The path to a detached git dir.
	/// Will default to the working directory (as specified by --tree).
	//
	#[ arg( short, long, value_parser, verbatim_doc_comment ) ]
	//
	pub gitdir: Option<PathBuf>,


	/// Where to log debugging information. Will default to `/var/log/gitofish.log`.
	/// You can pass `stdout`.
	//
	#[ arg( short, long, value_parser, verbatim_doc_comment ) ]
	//
	pub log: Option<PathBuf>,


	/// Log level. Must be one of `trace`, `debug`, `info`, `warning`, `error`.
	//
	#[ arg( short = 'e', long, default_value = "info", verbatim_doc_comment ) ]
	//
	pub log_level: LogLvl,


	/// The path to the checked out files.
	//
	#[ arg( short, long, value_parser, verbatim_doc_comment ) ]
	//
	pub tree: PathBuf,


	/// The the user that owns the checked out files.
	//
	#[ arg( short, long, value_parser, verbatim_doc_comment ) ]
	//
	pub owner: String,


	/// The gitolite user triggering the operation.
	//
	#[ arg( short, long, value_parser, verbatim_doc_comment ) ]
	//
	pub user: String,


	#[command(subcommand)]
	pub command: Commands,
}


#[allow(variant_size_differences)]
#[derive(Subcommand, Debug)]
//
pub enum Commands
{
	/// The pre git trigger of Gitolite
	//
	PreGit
	{
		#[ arg( short, long, value_parser ) ]
		//
		mode: Mode,
	},

	/// The post-receive hook in all Gitolite repositories
	//
	PostReceive
	{
		// The path to a script that will be run post checkout. This allows correcting permissions
		// on the checked out files. The script will run with pwd of --tree.
		//
		#[ arg( short, long, value_parser ) ]
		//
		post_checkout: Option<PathBuf>,

		/// Only used on initial repository clone. The remote from which to clone.
		//
		#[ arg( short, long, verbatim_doc_comment ) ]
		//
		remote: Option<String>,
	}
}


#[derive(Copy, Clone, Debug, strum::EnumString)]
//
pub enum Mode
{
	#[strum(serialize = "R")]
	Read,

	#[strum(serialize = "W")]
	Write,
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


	pub fn validate( self ) -> Result<Self, &'static str>
	{
		Ok(self)
	}
}




/// The loglevel as passed in through the cli.
//
#[ derive(Debug, Copy, Clone, strum::EnumString) ]
//
pub enum LogLvl
{
	#[strum(ascii_case_insensitive)] Trace,
	#[strum(ascii_case_insensitive)] Debug,
	#[strum(ascii_case_insensitive)] Info,
	#[strum(ascii_case_insensitive)] Warning,
	#[strum(ascii_case_insensitive)] Error,
}
