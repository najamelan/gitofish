// Wed Jul 26 01:33:47 GMT 2017 pre-git arguments: PRE_GIT etc gitEtcAdmin R any git-upload-pack
// uid=107(gitetc) gid=113(gitetc) groups=113(gitetc),111(gitolite)
// GL_ADMIN_BASE=/home/gitetc/.gitolite
// GL_BINDIR=/usr/local/lib/gitolite/src
// GL_LIBDIR=/usr/local/lib/gitolite/src/lib
// GL_LOGFILE=/home/gitetc/.gitolite/logs/gitolite-2017-07.log
// GL_OPTION_GTF_GITDIR=/etc
// GL_OPTION_GTF_TREE=/etc
// GL_OPTION_GTF_USER=root
// GL_REPO_BASE=/home/gitetc/repositories
// GL_REPO=etc
// GL_TID=3948
// GL_USER=gitEtcAdmin
// HOME=/home/gitetc
// LANG=en_US.UTF-8
// LOGNAME=gitetc
// MAIL=/var/mail/gitetc
// PATH=/usr/local/lib/gitolite/src:/usr/local/bin:/usr/bin:/bin:/usr/games
// PWD=/home/gitetc
// SHELL=/bin/sh
// SHLVL=1
// SSH_CLIENT=192.168.56.1 46708 22
// SSH_CONNECTION=192.168.56.1 46708 192.168.56.10 22
// SSH_ORIGINAL_COMMAND=git-upload-pack 'etc'
// USER=gitetc
// XDG_RUNTIME_DIR=/run/user/107
// XDG_SESSION_ID=6
//
//
//
// Wed Jul 26 01:41:44 GMT 2017 post-receive arguments:
// uid=107(gitetc) gid=113(gitetc) groups=113(gitetc),111(gitolite)

use crate::import::*;

#[derive(Deserialize, Debug)]
//
pub struct Env
{
	git_dir               : Option<String>,
	git_push_option_count : Option<String>,
	gl_admin_base         : Option<String>,
	gl_bindir             : Option<String>,
	gl_libdir             : Option<String>,
	gl_logfile            : Option<String>,
	gl_option_gtf_gitdir  : Option<String>,
	gl_option_gtf_tree    : Option<String>,
	gl_option_gtf_user    : Option<String>,
	gl_repo               : Option<String>,
	gl_repo_base          : Option<String>,
	gl_tid                : Option<String>,
	gl_user               : Option<String>,
	home                  : Option<String>,
	lang                  : Option<String>,
	logname               : Option<String>,
	mail                  : Option<String>,
	path                  : Option<String>,
	pwd                   : Option<String>,
	shell                 : Option<String>,
	shlvl                 : Option<String>,
	ssh_client            : Option<String>,
	ssh_connection        : Option<String>,
	ssh_original_command  : Option<String>,
	user                  : Option<String>,
	xdg_runtime_dir       : Option<String>,
	xdg_session_id        : Option<String>,
}
