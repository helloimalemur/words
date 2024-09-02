#!/bin/bash
git clone https://github.com/helloimalemur/words;
cd words/;
pnpm install;
export NO_STRIP=true;
pnpm tauri build;
