#!/bin/sh

set -ex

valgrind --log-file=valgrind.log "$@" >/dev/null &
pid=$!

sleep 5

kill $pid
wait $pid || :

grep 'definitely lost: 0 bytes in 0 blocks' valgrind.log
grep 'indirectly lost: 0 bytes in 0 blocks' valgrind.log
grep '0 errors from 0 contexts' valgrind.log
