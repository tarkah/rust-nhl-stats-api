#!/bin/bash

rm -rf out/
docker run --rm --user "$(id -u)":"$(id -g)" -v ${PWD}:/local openapitools/openapi-generator-cli generate -i /local/nhl-openapi-3.0.yaml -g rust -o /local/out/hyper
docker run --rm --user "$(id -u)":"$(id -g)" -v ${PWD}:/local openapitools/openapi-generator-cli generate -i /local/nhl-openapi-3.0.yaml -g rust -o /local/out/reqwest --library reqwest
rm -rf src/async src/sync src/models docs/
cp -R out/reqwest/src/apis src/sync
cp -R out/hyper/src/apis src/async
cp -R out/hyper/src/models src/models
cp -R out/hyper/docs docs
./updates.sh