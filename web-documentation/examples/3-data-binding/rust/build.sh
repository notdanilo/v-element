#! /bin/sh
cargo web-component build
cp pkg/index.html.bkp pkg/index.html
mv pkg/web-status.js pkg/web-status/
mv pkg/web-status_bg.wasm pkg/web-status/