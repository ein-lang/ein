#!/bin/sh

set -ex

valgrind --leak-check=full "$@" 2>valgrind.log &
pid=$!

sleep 5

kill $pid
wait $pid || :

grep 'definitely lost: 0 bytes in 0 blocks' valgrind.log
grep 'indirectly lost: 0 bytes in 0 blocks' valgrind.log
