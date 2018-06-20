Diary of rustdroid-native
========================

-------------------------
### 2018.04.13 c4augustus

__18:45: install rustup__
  * `curl https://sh.rustup.rs -sSf | sh`
~~~
info: downloading installer
warning: it looks like you have an existing installation of Rust
warning: rustup cannot be installed alongside Rust. Please uninstall first
warning: run `/usr/local/lib/rustlib/uninstall.sh` as root to uninstall Rust
error: cannot install while Rust is installed
~~~
  * `sudo /usr/local/lib/rustlib/uninstall.sh`
  * `curl https://sh.rustup.rs -sSf | sh`
~~~
info: downloading installer
warning: it looks like you have existing rustup.sh metadata
warning: rustup cannot be installed while rustup.sh metadata exists
warning: delete `/Users/?????/.rustup` to remove rustup.sh
warning: or, if you already rustup installed, you can run
warning: `rustup self update` and `rustup toolchain list` to upgrade
warning: your directory structure
error: cannot install while rustup.sh is installed
~~~
  * `sudo rm -fR ~/.rustup`
  * `curl https://sh.rustup.rs -sSf | sh`
~~~
info: downloading installer

Welcome to Rust!

This will download and install the official compiler for the Rust programming
language, and its package manager, Cargo.

It will add the cargo, rustc, rustup and other commands to Cargo's bin
directory, located at:

  /Users/?????/.cargo/bin

This path will then be added to your PATH environment variable by modifying the
profile files located at:

  /Users/?????/.profile
  /Users/?????/.bash_profile

You can uninstall at any time with rustup self uninstall and these changes will
be reverted.

Current installation options:

   default host triple: x86_64-apple-darwin
     default toolchain: stable
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
~~~
  * `1`
~~~
warning: tool `rustfmt` is already installed, remove it from `/Users/?????/.cargo/bin`, then run `rustup update` to have rustup manage this tool.
warning: tool `cargo-fmt` is already installed, remove it from `/Users/?????/.cargo/bin`, then run `rustup update` to have rustup manage this tool.
info: syncing channel updates for 'stable-x86_64-apple-darwin'
info: latest update on 2018-03-29, rust version 1.25.0 (84203cac6 2018-03-25)
info: downloading component 'rustc'
 46.5 MiB /  46.5 MiB (100 %)   1.9 MiB/s ETA:   0 s
info: downloading component 'rust-std'
 38.7 MiB /  38.7 MiB (100 %)   2.5 MiB/s ETA:   0 s
info: downloading component 'cargo'
  2.7 MiB /   2.7 MiB (100 %)   1.1 MiB/s ETA:   0 s
info: downloading component 'rust-docs'
  5.7 MiB /   5.7 MiB (100 %)   2.5 MiB/s ETA:   0 s
info: installing component 'rustc'
info: installing component 'rust-std'
info: installing component 'cargo'
info: installing component 'rust-docs'
info: default toolchain set to 'stable'

  stable installed - rustc 1.25.0 (84203cac6 2018-03-25)


Rust is installed now. Great!

To get started you need Cargo's bin directory ($HOME/.cargo/bin) in your PATH
environment variable. Next time you log in this will be done automatically.

To configure your current shell run source $HOME/.cargo/env
~~~

__19:00: install rust android target__
  * `rustup target add arm-linux-androideabi`
~~~
info: downloading component 'rust-std' for 'arm-linux-androideabi'
 13.8 MiB /  13.8 MiB (100 %)   5.9 MiB/s ETA:   0 s
info: installing component 'rust-std' for 'arm-linux-androideabi'
~~~

-------------------------
### 2018.04.15 c4augustus

__11:10: made local clone of github repo rustdroid-native__
  * `cd rustdroid/repo`
  * `git clone https://github.com/rustdroid/rustdroid-native.git`

__11:20: forked tomaka/android-rs-glue.git__
  * firefox: `https://github/tomaka/android-rs-glue`
  * `[fork]` `[@rustdroid]`

__11:30: cloned rustdroid/android-rs-glue.git__
  * `cd rustdroid/repo
  * `git clone https://github.com/rustdroid/android-rs-glue.git`

__11:50: installed cargo apk subcommand__
  * vim ~/.cargo/config
~~~
#replace-with = 'local-registry'
~~~
__12:00: installed cargo apk subcommand__
  * `cd android-rs-glue/cargo-apk
  * `cargo install`
~~~
  Installing cargo-apk v0.4.1 (file:///Users/?????/v/dev/copyleft/rustdroid/repo/android-rs-glue/cargo-apk)
   Compiling rustc-demangle v0.1.4
   Compiling semver-parser v0.7.0
   Compiling itoa v0.3.1
   Compiling log v0.3.7
   Compiling num-traits v0.1.40
   Compiling utf8-ranges v1.0.0
   Compiling foreign-types v0.2.0
   Compiling openssl v0.9.11
   Compiling matches v0.1.4
   Compiling void v1.0.2
   Compiling term v0.4.5
   Compiling dtoa v0.4.1
   Compiling pkg-config v0.3.9
   Compiling bitflags v0.7.0
   Compiling rustc-serialize v0.3.24
   Compiling lazy_static v0.2.8
   Compiling strsim v0.6.0
   Compiling libc v0.2.21
   Compiling unicode-xid v0.0.4
   Compiling serde v1.0.10
   Compiling unicode-normalization v0.1.4
   Compiling shell-escape v0.1.3
   Compiling cfg-if v0.1.0
   Compiling crossbeam v0.2.10
   Compiling glob v0.2.11
   Compiling gcc v0.3.45
   Compiling quote v0.3.15
   Compiling regex-syntax v0.4.1
   Compiling bitflags v0.8.2
   Compiling scoped-tls v0.1.0
   Compiling unicode-bidi v0.2.5
   Compiling unreachable v1.0.0
   Compiling synom v0.11.3
   Compiling jobserver v0.1.6
   Compiling rand v0.3.15
   Compiling fs2 v0.4.2
   Compiling filetime v0.1.10
   Compiling num_cpus v1.3.0
   Compiling memchr v1.0.1
   Compiling backtrace v0.3.2
   Compiling thread_local v0.3.4
   Compiling syn v0.11.11
   Compiling idna v0.1.1
   Compiling toml v0.1.30
   Compiling tempdir v0.3.5
   Compiling tar v0.4.11
   Compiling miniz-sys v0.1.9
   Compiling openssl-sys v0.9.11
   Compiling curl-sys v0.3.10
   Compiling libz-sys v1.0.13
   Compiling cmake v0.1.22
   Compiling aho-corasick v0.6.3
   Compiling error-chain v0.10.0
   Compiling toml v0.4.2
   Compiling serde_json v1.0.2
   Compiling serde_ignored v0.0.3
   Compiling semver v0.7.0
   Compiling url v1.4.0
   Compiling regex v0.2.2
   Compiling libssh2-sys v0.2.5
   Compiling libgit2-sys v0.6.7
   Compiling serde_derive_internals v0.15.1
   Compiling flate2 v0.2.19
   Compiling curl v0.4.6
   Compiling serde_derive v1.0.10
   Compiling docopt v0.7.0
   Compiling env_logger v0.4.3
   Compiling git2 v0.6.4
   Compiling git2-curl v0.7.0
   Compiling crates-io v0.9.0
   Compiling cargo v0.20.0
   Compiling cargo-apk v0.4.1 (file:///Users/?????/v/dev/copyleft/rustdroid/repo/android-rs-glue/cargo-apk)
    Finished release [optimized] target(s) in 116.52 secs
   Replacing /Users/?????/.cargo/bin/cargo-apk
~~~
  * `cargo local-registry --sync Cargo.lock ~/u/run/rust/cargo/registry`
  * vim ~/.cargo/config
~~~
replace-with = 'local-registry'
~~~

-------------------------
### 2018.04.17 c4augustus

__11:00: created github repo rustdroid-native__
  * firefox: `https://github/organizations/rustdroid`
  * [New Repository]
  * Repository name: `rustdroid-native`
  * Description: `Rust integration into Android Studio`
  * `X` Public (default)
  * `X` Initialize this repository with a README
  * [Add a license]: [Mozilla Public License 2.0]
  * [Create repository]

-------------------------
### 2018.06.20 c4augustus

__09:00: upgrade rust from 1.25.0 to 1.26.2, cargo from 0.26.0 to 1.26.0
  * `rustup update`
~~~
info: syncing channel updates for 'stable-x86_64-apple-darwin'
info: latest update on 2018-06-05, rust version 1.26.2 (594fb253c 2018-06-01)
info: downloading component 'rustc'
 57.4 MiB /  57.4 MiB (100 %)   8.5 MiB/s ETA:   0 s
info: downloading component 'rust-std'
 45.9 MiB /  45.9 MiB (100 %)   8.7 MiB/s ETA:   0 s
info: downloading component 'cargo'
info: downloading component 'rust-docs'
  6.8 MiB /   6.8 MiB (100 %)   6.2 MiB/s ETA:   0 s
info: downloading component 'rust-std' for 'arm-linux-androideabi'
 18.1 MiB /  18.1 MiB (100 %)   7.6 MiB/s ETA:   0 s
info: removing component 'rustc'
info: removing component 'rust-std'
info: removing component 'cargo'
info: removing component 'rust-docs'
info: removing component 'rust-std' for 'arm-linux-androideabi'
info: installing component 'rustc'
info: installing component 'rust-std'
info: installing component 'cargo'
info: installing component 'rust-docs'
info: installing component 'rust-std' for 'arm-linux-androideabi'
info: checking for self-updates

  stable-x86_64-apple-darwin updated - rustc 1.26.2 (594fb253c 2018-06-01)

warning: tool `rustfmt` is already installed, remove it from `/Users/admin/.cargo/bin`, then run `rustup update` to have rustup manage this tool.
warning: tool `cargo-fmt` is already installed, remove it from `/Users/admin/.cargo/bin`, then run `rustup update` to have rustup manage this tool.
~~~

-----------------
### (end of file)
