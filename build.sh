#!/usr/bin/env bash

export SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"

echo "Remove ${SCRIPTPATH}/build/apps/opz/debug"
rm -rf ${SCRIPTPATH}/build/apps/opz/debug

echo "Remove ${SCRIPTPATH}/build/target/x86_64-apple-darwin"
rm -rf ${SCRIPTPATH}/build/target/x86_64-apple-darwin

# https://github.com/indygreg/PyOxidizer/issues/72
#cd ${SCRIPTPATH}/build/target/x86_64-apple-darwin/debug/pyoxidizer
#pyoxidizer build-artifacts ${SCRIPTPATH}/build/target/x86_64-apple-darwin/debug/pyoxidizer ${SCRIPTPATH}

cd ${SCRIPTPATH}
pyoxidizer build
