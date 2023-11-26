# Oxide
*A decentralized, monorepo & polyglot friendly package management solution from
the future*

The idea of oxide is to provide one package manager for as many languages and
ecosystems as possible, while focusing on being monorepo friendly.

## Project Structure
Oxide projects are intended to be laid out like this:
```
// An example project with a web server and a client interface
[example]
|-oxide.toml
|-[server]
| |-.oxide
| | |-jvm.toml
| |-oxide.toml
| |-build.gradle.kts
| |-settings.gradle.kts
|
|-[web]
  |-.oxide
  | |-node.toml
  | |-python.toml
  |-oxide.toml
  |-package.json
  |-requirements.txt
```

## Commands

### Sync

When you run `oxide sync` in any of the [project]s the client will look up the
tree to the top level of the workspace, in this case: `[example]`, and it will
send a signal to the daemon process to resynchronize its project graph to match
the configurations stored in the `oxide.toml` files.

### Install [// TODO]
When you run `oxide install` in any of the [project]s the client will send an 
install signal to the local daemon, which will then dispatch the message to all
loaded plugins. Plugins are then responsible for checking for their own config
under the project `.oxide` folder and adding tasks to the daemon's task queue
that when ran should result in a full installation for the target implemented by
the plugin.

### TO-DO
- [ ] Decentralized package management system
  - [ ] Home server
    - [ ] Package registry
      - [ ] NPM
      - [ ] Maven
      - [ ] Ruby
      - [ ] Python
    - [ ] Web dashboard
    - [ ] Audit log
      - [ ] Hosted admin UI
- [ ] Cli
  - [ ] Installation packages
    - [ ] NPM
    - [ ] Cargo
    - [ ] RubyGems
    - [ ] MacOS
      - [ ] .pkg
      - [ ] Homebrew
    - [ ] Windows (exe, msi)
    - [ ] Linux
      - [ ] AppImage
      - [ ] Snap
      - [ ] Flatpak
      - [ ] Distro packages
        - [ ] Deb
        - [ ] Arch
        - [ ] Void
        - [ ] Rpm
        - [ ] Suse
        - [ ] Nix
  - [ ] Project init
    - [ ] Config creation
- [ ] System daemon
  - [ ] Local repository server
  - [ ] Plugin system
    - [x] Plugin loading
      - [ ] Plugin hot load
    - [ ] Plugin hooks & functionality
      - [ ] Install hook (WIP)
  - [ ] Audit log
    - [ ] Sqlite storage
    - [ ] Hosted admin UI
- [ ] Core tooling
  - [ ] Node
    - [ ] Plugin
      - [ ] Dependency fetching
      - [ ] Package publishing
  - [ ] Python
    - [ ] Plugin
      - [ ] Dependency fetching
      - [ ] Package publishing
      - [ ] Venv management
  - [ ] Ruby
    - [ ] Plugin
      - [ ] Dependency Fetching
      - [ ] Package publishing
  - [ ] JVM
    - [ ] Gradle Plugin
      - [ ] Fetch & enable repositories from daemon
      - [ ] Maven-publish DSL integration
    - [ ] Maven Plugin
      - [ ] Fetch & enable repositories from daemon
      - [ ] Publish features
- [ ] Configuration
  - [ ] Global daemon config
    - [x] Locally enabled repositories for each lang
    - [ ] Specify which local repositories to host if any
  - [ ] Per-user cli config
    - [ ] User authorized repositories
    - [x] Allow user to specify daemon address

*This list isn't exhaustive!*

## Licensing
This project is currently licensed under the [Apache 2.0 License](LICENSE).