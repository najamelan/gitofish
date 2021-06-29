//! Test libgitofish::task::refresh.
//!
//! refresh needs to make sure that the repository is in a state ready to pull new commits. It does:
//! - commit all
//! - push --force to remote
//! - work recursively on submodules.
//!
//! It returns `Result<RefreshStatus, git2::Error>` where RefreshStatus indicates whether there was new content.
//!
//! Scenarios:
//!
//!   -
