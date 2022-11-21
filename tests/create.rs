//! Test creating the checked out repository on the server.
//!
//! Tested:
//!
//!   - simple use case of creating unexisting repository.
//!   - gitolite repo has no commits in deploy branch
//!   - gitolite repo has commits in deploy branch
//!   - no permissions to parent directory
//!   - dst exists and is file
//!   - dst exists and is file we have no read permissions to
//!   - dst exists and is file we have no write permissions to
//!   - dst exists and is empty dir
//!   - dst exists and is empty dir we have no write permissions to
//!   - dst exists and is empty dir we have no read permissions to
//!   - dst exists and is non-empty directory where our repo already exists
//!   - dst exists and is non-empty directory with another repository
//!   - dst exists and is non-empty directory with random files
//!
//!   - all of the above with a separate git_dir.
//!   - do we run refresh at creation?
//!
//!
//
mod common;

use
{
	libgitofish :: * ,
	common      :: * ,
};
