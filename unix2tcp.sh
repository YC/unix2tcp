#!/bin/sh

path=$1
addr=$2

rm -f $path
exec socat UNIX-LISTEN:$path,fork TCP-CONNECT:$addr
