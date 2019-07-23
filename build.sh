#!/usr/bin/env bash

export SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"

echo "Remove ${SCRIPTPATH}/build/apps/opz/debug"
rm -rf ${SCRIPTPATH}/build/apps/opz/debug

echo "Remove ${SCRIPTPATH}/build/target/x86_64-apple-darwin"
rm -rf ${SCRIPTPATH}/build/target/x86_64-apple-darwin

pyoxidizer build
