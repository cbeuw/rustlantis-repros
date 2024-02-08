#!/usr/bin/env bash

set -eu

TRIPLE=$(rustc -vV | sed -n 's|host: ||p')
mkdir -p $TRIPLE/solved
ls $TRIPLE/repros-*/*.rs | xargs -I{} -P4 sh -c 'echo {}; timeout 20 difftest {} 2> /dev/null || exit;  echo "{} solved"; mv {} solved'
