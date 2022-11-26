#![ cfg_attr(nightly, cfg_attr( nightly, doc = include_str!("../README.md") )) ]
#![ doc = "" ] // empty doc line to handle missing doc warning when the feature is missing.

#![ doc    ( html_root_url = "https://docs.rs/gitofish" ) ]
#![ forbid ( unsafe_code                                ) ]
#![ allow  ( clippy::suspicious_else_formatting         ) ]

#![ warn
(
	anonymous_parameters          ,
	missing_copy_implementations  ,
	missing_debug_implementations ,
	missing_docs                  ,
	nonstandard_style             ,
	rust_2018_idioms              ,
	single_use_lifetimes          ,
	trivial_casts                 ,
	trivial_numeric_casts         ,
	unreachable_pub               ,
	unused_extern_crates          ,
	unused_qualifications         ,
	variant_size_differences      ,
)]

#![ allow( unused_imports, dead_code, missing_docs ) ]

pub mod cli_arg;
pub mod git;
pub mod task;


pub use
{
	cli_arg :: * ,
};


// External dependencies
//
mod import
{
	pub(crate) use
	{
		config_crate :: { Config as ConfigCrate, ConfigError                             } ,
		git_version  :: { git_version                                                    } ,
		git2         :: { Repository                                                     } ,
		once_cell    :: { sync::Lazy                                                     } ,
		serde        :: { Deserialize                                                    } ,
		std          :: { path::{ PathBuf, Path }, sync::mpsc::channel, process::Command } ,
		clap         :: { Parser as ClapParser, crate_version, Subcommand                } ,
		tracing      :: { trace, debug, info, warn, error, span, error_span              } ,
	};


	#[ cfg( test ) ]
	//
	pub(crate) use
	{
		pretty_assertions :: { assert_eq } ,
	};
}


