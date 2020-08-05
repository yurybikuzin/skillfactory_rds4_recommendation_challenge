#!/usr/bin/env bash
version=2

image_name="bazawinner/dev-recommend-proj:$version"
docker build . -t "$image_name"
docker push "$image_name"
