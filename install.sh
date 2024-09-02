#!/usr/bin/env sh
git clone https://github.com/helloimalemur/words
cd words/ || echo "clone failed, dir missing" && exit 1
pnpm install
export NO_STRIP=true
pnpm tauri build
