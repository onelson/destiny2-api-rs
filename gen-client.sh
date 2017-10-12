#!/bin/bash

docker run --rm --user="$(id -u):$(id -g)" \
    -v ${PWD}:/local swaggerapi/swagger-codegen-cli generate \
    -i https://raw.githubusercontent.com/Bungie-net/api/master/openapi-2.json \
    -l rust \
    -o /local/codegen
