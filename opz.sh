#!/usr/bin/env bash

export SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
export PATH=$PATH:$SCRIPTPATH/build/apps/opz/debug

opz "$@"
