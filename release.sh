#!/usr/bin/env bash

trunk clean;
trunk build --release;
cp -r ./css ./dist/