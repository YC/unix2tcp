#!/bin/sh

path=$1
addr=$2
options=$3

rm -f "$path"
exec socat UNIX-LISTEN:$path,fork,$options TCP-CONNECT:$addr
