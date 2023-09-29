#!/bin/bash

cd ..

cache_volume="$(basename "$(pwd)")_cache"

docker run --rm \
  -v "$(pwd)":/contract \
  --mount type=volume,source="$cache_volume",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  enigmampc/secret-contract-optimizer

if [ $? -eq 0 ]; then
  echo "compile costom contract success!"
else
  echo "compile costom contract failed!"
fi
