# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.7](https://github.com/sdwilsh/jellyfin-remote-proxy/compare/v0.1.6...v0.1.7) - 2025-12-08

### Fixed

- update changelog that somehow got very broken
- pin checkout action the same in all workflows for real
- pin checkout action the same in all workflows

### Other

- *(deps)* update actions/checkout action to v6 ([#94](https://github.com/sdwilsh/jellyfin-remote-proxy/pull/94))

## [0.1.6](https://github.com/sdwilsh/jellyfin-remote-proxy/compare/v0.1.5...v0.1.6) - 2025-12-07

### Fixed

- *(deps)* update rust-deps ([#75](https://github.com/sdwilsh/jellyfin-remote-proxy/pull/75))

### Other

- Merge pull request #93 from sdwilsh/renovate/actions-checkout-digest
- *(deps)* update dependency casey/just to v1.44.0
- *(deps)* update dependency renovatebot/renovate to v42.39.0
- *(deps)* update rust crate log to v0.4.29
- *(deps)* update release-plz
- *(deps)* update dependency renovatebot/renovate to v42.26.11
- *(deps)* update release-plz
- Merge pull request #89 from sdwilsh/renovate/debian-bookworm-slim
- Merge pull request #91 from sdwilsh/renovate/docker-metadata-action-digest
- Merge pull request #92 from sdwilsh/renovate/actions-checkout-5.x
- Merge pull request #87 from sdwilsh/renovate/casey-just-1.x
- Merge pull request #86 from sdwilsh/renovate/hadolint-hadolint-2.x
- *(deps)* update dependency renovatebot/renovate to v42.19.3
- *(deps)* update rust:1.91-slim-bookworm docker digest to c406439
- *(deps)* update dependency renovatebot/renovate to v42.11.0
- *(deps)* update rust
- *(deps)* update dependency renovatebot/renovate to v42
- *(deps)* update dependency renovatebot/renovate to v41.173.1
- *(deps)* update sozu to v1.1.1
- *(deps)* update rust:1.91-slim-bookworm docker digest to 81c9759
- *(deps)* update sozu to v1.1.0
- *(deps)* update dependency renovatebot/renovate to v41.168.1
- *(deps)* update rust
- *(deps)* update dependency renovatebot/renovate to v41.159.3
- *(deps)* update rust:1.90-slim-bookworm docker digest to 64232e6
- *(deps)* update dependency renovatebot/renovate to v41.152.2
- *(deps)* update dependency renovatebot/renovate to v41.146.0
- *(deps)* update release-plz
- *(deps)* update dependency renovatebot/renovate to v41.135.3 ([#90](https://github.com/sdwilsh/jellyfin-remote-proxy/pull/90))
- *(deps)* update rust:1.90-slim-bookworm docker digest to 3bee83b
- *(deps)* update dependency renovatebot/renovate to v41.131.6
- *(deps)* update rust
- *(deps)* update dependency renovatebot/renovate to v41.119.5
- *(deps)* update release-plz
- *(deps)* update debian:bookworm-slim docker digest to df52e55
- Merge pull request #82 from sdwilsh/renovate/debian-bookworm-slim
- Merge pull request #84 from sdwilsh/renovate/hadolint-hadolint-2.x
- *(deps)* update dependency marinatedconcrete/config to renovate-config-2.2.0
- *(deps)* update dependency renovatebot/renovate to v41.97.7
- *(deps)* update release-plz
- *(deps)* update dependency renovatebot/renovate to v41.82.6
- Merge pull request #81 from sdwilsh/renovate/actions-checkout-5.x
- *(deps)* update release-plz
- *(deps)* update dependency renovatebot/renovate to v41.75.0
- *(deps)* update rust:1.89-slim-bookworm docker digest to 21e2ac3
- *(deps)* update dependency renovatebot/renovate to v41.60.2
- *(deps)* update rust
- *(deps)* update dependency renovatebot/renovate to v41.52.1

## [0.1.5](https://github.com/sdwilsh/jellyfin-remote-proxy/compare/v0.1.4...v0.1.5) - 2025-08-05

### Fixed

- build by default on tags for release

## [0.1.4](https://github.com/sdwilsh/jellyfin-remote-proxy/compare/v0.1.3...v0.1.4) - 2025-08-05

### Fixed

- also build manifest for tags

## [0.1.3](https://github.com/sdwilsh/jellyfin-remote-proxy/compare/v0.1.2...v0.1.3) - 2025-08-05

### Added

- make a manifest for multiplatform images
- tag more sensibly
- build container images for development
- build a container
- group release-plz updates together
- switch to using marinatedconcrete renovate configs

### Fixed

- drop date from manifest tags
- fix yamllint
- yamllint
- push the container to the right path
- actually pass in the `Containerfile`
- rename build from copypasta
- be more loose about line length of yaml
- use semver for versioning
- `crate` is the name of the datasource, even though the issues shows it as `cargo`
- switch to cargo datasource for renovate for release-plz
- do not include v in release-plz version
- maybe fix release-plz install issue
- group rust updates
- *(deps)* update automerge
- *(deps)* update automerge
- *(deps)* update automerge
- *(deps)* update automerge
- *(deps)* update sozu to v1.0.6 ([#57](https://github.com/sdwilsh/jellyfin-remote-proxy/pull/57))
- *(deps)* update automerge
- *(deps)* update sozu to v1
- *(deps)* update rust crate time to v0.3.37
- *(deps)* update rust crate serde to 1.0.198
- *(deps)* update rust crate serde_json to 1.0.115
- *(deps)* update rust crate serde_yaml to 0.9.33

### Other

- rework how renovate updates rust deps
- *(deps)* run cargo update to get past issues renovate is having
- *(deps)* update rust
- *(deps)* pin debian docker tag to 2424c18
- *(deps)* update dependency release-plz to v0.3.139
- Merge pull request #74 from sdwilsh/renovate/docker-login-action-digest
- *(deps)* pin dependencies
- Merge pull request #70 from sdwilsh/renovate/marcoieni-release-plz-action-0.x
- Merge pull request #65 from sdwilsh/renovate/yamllint-1.x
- Merge pull request #61 from sdwilsh/renovate/devcontainers-ci-0.x
- *(deps)* update devcontainers/ci action to v0.3.1900000417
- *(deps)* update dependency renovatebot/renovate to v41
- *(deps)* update dependency renovatebot/renovate to v40
- *(deps)* update dependency renovatebot/renovate to v39.185.0
- *(deps)* update dependency renovatebot/renovate to v39.170.1
- *(deps)* update dependency renovatebot/renovate to v39.164.0
- *(deps)* update dependency rust-lang/rust to v1.84.1
- Set timezone for renovate
- Migrate renovate config
- Replace deprecated `serde_yaml`
- Setup automerge for a bunch of things
- Fix formatting
- Auto-merge sozu updates
- *(deps)* update dependency renovatebot/renovate to v39.103.0
- Fix broken merge that resulted in duplicate workflow definitions
- Move build/test CI to devcontainer
- Fix lint issues
- Add linting using `just` instead of Earthly
- Add a devcontainer with associated CI
- *(deps)* update earthly/earthly docker tag to v0.8.12
- *(deps)* update earthly/actions-setup action to v1.0.12
- Merge pull request #35 from sdwilsh/renovate/earthly-earthly-0.x
- *(deps)* update earthly/earthly docker tag to v0.8.6
- Merge pull request #32 from sdwilsh/renovate/anyhow-1.x
- *(deps)* update earthly/earthly docker tag to v0.8.5

## [0.1.2](https://github.com/sdwilsh/jellyfin-remote-proxy/compare/v0.1.1...v0.1.2) - 2024-03-06

### Fixed
- *(deps)* update rust crate log to 0.4.21
- *(deps)* update rust crate env_logger to 0.11.3

### Other
- Merge pull request [#27](https://github.com/sdwilsh/jellyfin-remote-proxy/pull/27) from sdwilsh/dependabot/cargo/mio-0.8.11

## [0.1.1](https://github.com/sdwilsh/jellyfin-remote-proxy/compare/v0.1.0...v0.1.1) - 2024-02-26

### Other
- Do not use sozu default features
- Merge pull request [#23](https://github.com/sdwilsh/jellyfin-remote-proxy/pull/23) from sdwilsh/renovate/earthly-earthly-0.x
- Merge pull request [#18](https://github.com/sdwilsh/jellyfin-remote-proxy/pull/18) from sdwilsh/renovate/serde_yaml-0.x
- remove Cargo.lock from .gitignore
- do release pr and release
- one-off cargo update to hopefully make release-plz happier
- Merge pull request [#19](https://github.com/sdwilsh/jellyfin-remote-proxy/pull/19) from sdwilsh/renovate/serde-monorepo
- Merge pull request [#17](https://github.com/sdwilsh/jellyfin-remote-proxy/pull/17) from sdwilsh/renovate/anyhow-1.x
