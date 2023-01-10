# CITA CLOUD COMMON

cita cloud common for rust


## use

- update protos

```git
git submodule deinit --all -f
git submodule update --force --init --remote --recursive
```

- build

```shell
cargo clippy --all --all-features
cargo build --release --all --all-features
```

- or run the script

```shell
sh update_proto.sh
```
