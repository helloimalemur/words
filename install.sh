#!/bin/sh
# shellcheck shell=dash
# shellcheck disable=SC2039
git clone https://github.com/helloimalemur/words;
cd words/ || exit;
pnpm install;
export NO_STRIP=true;
pnpm tauri build;
