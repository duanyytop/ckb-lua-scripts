# ckb-lua-scripts

Init submodules:

```sh
git submodule init && git submodule update -r --init
```

Build the shared binaries:

```sh
cd ckb-lua/
make all-via-docker
```

Build contracts:

``` sh
capsule build
```

Run tests:

``` sh
capsule test
```
