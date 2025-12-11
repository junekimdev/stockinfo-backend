# Stockinfo Backend

![release-version](https://img.shields.io/github/v/release/junekimdev/stockinfo-backend?display_name=tag)
[![Build Container Image](https://github.com/junekimdev/stockinfo-backend/actions/workflows/docker-publish.yaml/badge.svg)](https://github.com/junekimdev/stockinfo-backend/actions/workflows/docker-publish.yaml)

## Getting Started

### Prerequisite

- Install latest `Rust`

  <https://www.rust-lang.org/tools/install>

  ```shell
  # for linux
  curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
  rustup update
  rustc --version
  ```

- Install `Docker`
- Install `Make`
- Install `OpenSSL`
  - for Windows:
    - Use `vcpkg` to install `OpenSSL`
    - Set `OPENSSL_DIR` environment variable to its installed path
      - or set `OPENSSL_LIB_DIR` and `OPENSSL_INCLUDE_DIR` environment variables to its installed path
    - Set the `VCPKGRS_DYNAMIC` environment variable to `1` to instruct the `openssl-sys` crate to use dynamic linking
  - for Debian and Ubuntu:
    - `sudo apt-get install pkg-config libssl-dev`
  - for Alpine Linux:
    - `apk add pkgconf openssl-dev`
  - for Arch Linux:
    - `sudo pacman -S pkgconf openssl`
  - for macOS
    - Use Homebrew: `brew install openssl@3`
      - or use MacPorts: `sudo port install openssl`
      - or use pkgsrc: `sudo pkgin install openssl`

### Prep env and config

- Add a file `.env` that includes environmental variables

  Example:

  ```shell
  POSTGRES_USER=stockinfo
  POSTGRES_DB=stockinfo
  POSTGRES_PASSWORD=super-secret-password
  POSTGRES_INITDB_ARGS="--lc-collate=C --lc-ctype=en_US.UTF-8"
  ```

- Add a file `production.toml` under `/config` by modifying `default.toml` with proper values

### Build

- Build docker image by running `make` in your terminal
  - or run `cargo build`

### Initial DB Build

```shell
curl -X POST -d {} <URL_API>/v1/companies
curl -X POST -d {} <URL_API>/v1/dart/code
curl -X POST -d {} <URL_API>/v1/tickers
```

### Edit crontab

- Open crontab editor: `crontab -e`
- Schedule API executions for cleaning process

  ```shell
  0 0 * * * curl -X DELETE <URL_API>/v1/prices &> /dev/null
  0 5 * * * curl -X DELETE <URL_API>/v1/prices_us &> /dev/null
  0 18 * * 6 curl -X POST -d {} <URL_API>/v1/companies &> /dev/null
  30 18 * * 6 curl -X POST -d {} <URL_API>/v1/dart/code &> /dev/null
  0 19 * * 6 curl -X POST -d {} <URL_API>/v1/tickers &> /dev/null
  ```

## Authors

- **June Kim** - _Initial work_ - [Github](https://github.com/junekimdev)

## License

No license is given by the author. All rights are reserved.
