#!/usr/bin/env bash

set -eu

TRIPLE=$(rustc -vV | sed -n 's|host: ||p')
cd $TRIPLE
mkdir -p solved
ls repros-*/*.rs | xargs -I{} -P4 sh -c 'echo {}; ~/rustlantis/target/release/difftest {} 2> /dev/null || exit;  echo "{} solved"; mv {} solved'

