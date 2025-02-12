# Stockinfo Backend

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

- Or, Run `cargo build`

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
