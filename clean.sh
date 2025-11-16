#!/usr/bin/env bash
set -eu

export TRIPLE=$(rustc -vV | sed -n 's|host: ||p')

mkdir -p $TRIPLE/solved

check_repo() {
  timeout 60 difftest $1 2> /dev/null
  CODE=$?
  if [ $CODE == 0 ]; then
    echo "$1 fixed"
    mv $1 $TRIPLE/solved/
  elif [ $CODE == 124 ]; then
    echo "$1 timed out"
  else
    echo "$1 still buggy"
  fi
}

export -f check_repo
find $TRIPLE -name '*.rs' ! -path '*reported*' ! -path '*solved*' | xargs -I{} -P4 bash -c 'check_repo {}'
