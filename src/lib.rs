#![ cfg_attr( nightly, feature( doc_cfg ) ) ]
#![ cfg_attr(nightly, cfg_attr( nightly, doc = include!("../README.md") )) ]
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

mod tasks;
// mod config;
mod env;
mod git;


pub use
{
	tasks  :: * ,
	// config :: * ,
	env    :: * ,
	git    :: * ,
};



// External dependencies
//
mod import
{
	pub(crate) use
	{
		std :: {  } ,
		serde :: { Deserialize } ,
		tracing :: { trace, debug, info, warn, error, span } ,
	};


	#[ cfg( test ) ]
	//
	pub(crate) use
	{
		pretty_assertions :: { assert_eq } ,
	};
}


