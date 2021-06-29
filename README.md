# gitofish

[![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)
[![Build Status](https://github.com/najamelan/gitofish/workflows/ci/badge.svg?branch=release)](https://github.com/najamelan/gitofish/actions)
[![Docs](https://docs.rs/gitofish/badge.svg)](https://docs.rs/gitofish)
[![crates.io](https://img.shields.io/crates/v/gitofish.svg)](https://crates.io/crates/gitofish)


> Deploy to your server with git push

Gitofish is a system which allows deploying code to servers through git push. This tackles 2 challenges:
- you need a git server (solved by using gitolite), although it doesn't strictly need to be on the same machine as the checked out content.
- git does not save all metadata, notably permissions (solved by running a permissions script on the server after checkout).

Gitofish has two components:
- server provisioning. It allows to write a configuration file and have an installation script setup gitolite and
  user accounts, permissions etc.
- the gitofish binary which is run by gitolite on pre-git trigger and on post-receive hook in managed repositories.

The main advantages of this system are:
- most developers already use git as a version control system. This allows to just have a deploy branch in your repository
  and push to that in order to deploy your code.
- you configure/program everything conveniently on your developer machine and push rather than having to manage a server

- git has good support for multiple remotes, so you can easily test staging and production separately from the same repo.
- it also works with etckeeper so you can also configure your server locally and push rather than modify configuration files
  through ssh.
- Everything is versioned. Compared to the temptation of going into a server with ssh and modifying some configuration files
  without leaving a clear trace, since everything is in git, there will be a clear trace of all modifications to the server.
- detect (malicious) changes to server files. The server will never do merges. That is when you want to push code out,
  it will first commit all changes on the server and oblige you to fetch and merge them. It only does fast forward merges.
  This way if some hacker changed you php files, or the files in /etc. You will notice. You can verify any time with a
  fetch or you will be forced to deal with it on a push.

How does it compare to [git-deploy](https://github.com/git-deploy/git-deploy)? Well, the docs there say:

> One thing it definitely doesn't do is worry about how your code gets copied around to your production servers, that's completely up to you.

Gitofish implements that part, and can probably be used with `git-deploy` if you need to manage deployments between several users.
I usually manage my servers alone, so I haven't played around with `git-deploy`.

## Table of Contents

- [Install](#install)
   - [Upgrade](#upgrade)
   - [Dependencies](#dependencies)
   - [Security](#security)
- [Architecture](#architecture)
- [Usage](#usage)
   - [Basic Example](#basic-example)
   - [API](#api)
- [Contributing](#contributing)
   - [Code of Conduct](#code-of-conduct)
- [License](#license)


## Install
With [cargo add](https://github.com/killercup/cargo-edit):
`cargo add gitofish`

With [cargo yaml](https://gitlab.com/storedbox/cargo-yaml):
```yaml
dependencies:

   gitofish: ^0.1
```

In Cargo.toml:
```toml
[dependencies]

   gitofish = "0.1"
```

### Upgrade

Please check out the [changelog](https://github.com/najamelan/gitofish/blob/release/CHANGELOG.md) when upgrading.


### Dependencies

This crate has few dependencies. Cargo will automatically handle it's dependencies for you.

There are no optional features.


### Security

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev) to verify the trustworthiness of each of your dependencies, including this one.


## Architecture

The server setup is as follows:

- one gitolite user which allows you to push to the server. This user has bare versions of your repositories.
- one content user that owns the actual checked out files

You setup gitolite through the gitolite-admin repository with it's ssh keys and its config file. This manages
what repositories are hosted by it and who has permissions to what. In this way you also benefit from all the
power of gitolite and can have different users with different levels of access.

The `gitofish` executable runs as a pre-git trigger. When a repository is not configured to be managed by gitofish,
it does strictly nothing. You just have a standard gitolite server.

When it's configured for checkout, it tells the content user to commit all uncommitted changes in the checked out
repository. Next that user will push those changes to gitolite locally.

Next, the user command is evaluated:

1. On fetch, you will now see the most up to date version of the files on disk on the server.
2. on push, gitofish will run again on post-receive. Now it will once more verify there are no
   uncommited changes and then it will pull --ff from gitolite, checking out the files on disk.
   It will then run a permissions script if one is configured in the gitolite configuration.

In terms of permissions this allows the following setup (example a php website):

content user owns the checked out files.
gitcontent user (gitolite) can call gitofish as content so it can manipulate the repository.
php group gets read permissions on the php files in website.
www-data group gets read permissions on html, htm, css, js, images etc.

only content/root can directly modify files, although the gitcontent also has the power to change the files,
as it manages the source of truth for the repository.

Developers cannot push --force to gitolite. The server will reject rewinds from outside, however the content user
can push force in case files on the server have changed. This way we always have an exact representation of the checked out
files on gitolite.

When using gitofish for /etc, a separate gitolite user is created because of the sensitive nature of /etc. /etc itself is of
course owned by root.

Content can live anywhere else on the server as long as the permissions are setup correctly. That is, you can host your website
from `/home/content/website` or from `/var/www/website` as you see fit.

Gitofish writes a log file if you want to so you can debug things that break.

It all works with submodules as well. Since gitofish pushes local changes to files back into gitolite, it must be able to push
the submodule repositories. Also as usual, you need to make sure you push new work in submodules before pushing the super repo.

In principle gitolite does not need to be hosted on the same server. You could host it somewhere else, as long as that server
can run the hooks on the checked out server (through ssh for example), as the content user pushes and pulls from gitolite through ssh,
it doesn't access repositories through the file system directly. You can probably run those hooks with a github action for example.


## Usage



### Basic example

```rust

```

## API

API documentation can be found on [docs.rs](https://docs.rs/gitofish).


## Contributing

Please check out the [contribution guidelines](https://github.com/najamelan/gitofish/blob/release/CONTRIBUTING.md).


### Testing


### Code of conduct

Any of the behaviors described in [point 4 "Unacceptable Behavior" of the Citizens Code of Conduct](https://github.com/stumpsyn/policies/blob/master/citizen_code_of_conduct.md#4-unacceptable-behavior) are not welcome here and might get you banned. If anyone, including maintainers and moderators of the project, fail to respect these/your limits, you are entitled to call them out.

## License

[Unlicence](https://unlicense.org/)

