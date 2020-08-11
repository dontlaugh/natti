#!/bin/bash

json=$(cat -)
image=$(echo $json | jq -r '.image')

echo "deploying image $image"

# do deploy stuff...

