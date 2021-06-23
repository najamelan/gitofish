//! This task makes sure a checked out repository is ready for interaction from gitolite.
//! This means that all modified files are commited and pushed to gitolite. Before doing
//! this repository, we will recurse into all submodules.
//!
//! Steps:
//!
//! - verify GL_OPTION_GTF_TREE, we should not get called without it set.
//! - if GL_OPTION_GTF_TREE is not an existing directory with git repo, we shouldn't do anything. Ideally, don't get called here either.
//! - For PRE_GIT:
//!   - commit-all
//!   - push-all
//!
//! - For post-receive:
//!   - vc pull --ff-only gitolite deployed:deployed
//!   - vc submodule update
//!
//!     # We shouldn't overwrite changes on disk... although that has maybe been checked on pre_git.
//!   - vc submodule foreach "git checkout deployed; git reset --hard HEAD@{1}"
//!
//!   - after checkouts, need etckeeper init.
//!   - if there are GL_OPTION_GTF_POST_CHECKOUT scripts, run them. In fish we always ran as sudo, but shoulb be specified probably.
//!   - commit-all
//!   - push-all
//!
//!
//! Where:
//!
//! commit-all =
//!   - git submodule foreach --recursive '/usr/local/lib/gitofish/commit-all'
//!   - vc checkout deployed
//!   - vc add --all
//!   - vc commit --message="SECURITY: New/Modified files appeared on server"
//!
//! push-all:
//!   - git submodule foreach '/usr/local/lib/gitofish/push-all'
//!   - git checkout deployed
//!   - vc push --force gitolite deployed:deployed
//!
//!
