#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"

cd $SCRIPT_DIR

npm install tailwindcss @tailwindcss/cli

npx @tailwindcss/cli \
    -i ./packages/ui/tailwind.css \
    -o ./packages/ui/assets/css/tailwind.css \
    --watch
