#!/bin/bash

usage() {
  echo "exactly two arguments required: [pub|sub] [go|rust]"
  echo "                                  MODE      IMPL"
  exit 1
}

if [[ -z $1 ]]; then
  usage
fi

if [[ -z $2 ]]; then
  usage
fi

mode=$1
impl=$2

rust_publish() {
  natti publish --server nats-server-01.greymatter.services \
    --subject "test.topic" \
    --message $1
  echo "rust_publish $1"
}

rust_subscribe() {
  natti subscribe --server nats-server-01.greymatter.services \
    --subject "test.topic"
}

go_publish() {
  nats-pub -s nats-server-01.greymatter.services:4222 test.topic $1
}

go_subscribe() {
  nats-sub -s nats-server-01.greymatter.services:4222 test.topic
}

case $mode in
pub)
  i=1
  while true; do
    case $impl in
    rust)
      rust_publish $i
      ;;
    go)
      go_publish $i
      ;;
    *)
      echo "unknown mode"
      exit 1
      ;;
    esac
    ((i = i + 1))
  done
  ;;
sub)
  echo "sub"
  case $impl in
  rust)
    rust_subscribe
    ;;
  go)
    go_subscribe
    ;;
  *)
    echo "unknown mode"
    exit 1
    ;;
  esac
  ;;
esac
