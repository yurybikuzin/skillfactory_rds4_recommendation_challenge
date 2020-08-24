#!/usr/bin/env bash
_migrate() {
    diesel migration revert 
    diesel migration run || return $?
}
pushd db
if _migrate; then
    popd
    cargo run --release -p db
else
    popd
fi

