#!/usr/bin/env bash

set -eu

TRIPLE=$(rustc -vV | sed -n 's|host: ||p')
cd $TRIPLE
mkdir -p solved
ls repros-*/*.rs | xargs -I{} -P4 sh -c 'echo {}; timeout 20 difftest {} 2> /dev/null || exit;  echo "{} solved"; mv {} solved'

