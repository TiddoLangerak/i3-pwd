#!/bin/bash
set -e

cargo build -r
cp target/release/i3-pwd ~/bin/i3-pwd
