#!/usr/bin/env bash
version=7

dir=$(dirname "$0")

echo pushd "$dir" . . .
pushd "$dir" || { echo ERR: failed to pushd "$dir"; exit 1; }

image_name="bazawinner/dev-recommend-proj:$version"
docker build . -t "$image_name" && docker push "$image_name"

popd || { echo ERR: failed to popd; exit 1; }
