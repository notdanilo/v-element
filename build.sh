wasm-pack build --release --target web --no-typescript --out-dir ../pkg/web-component --out-name web-component web-component -- --features "importer"

wasm-pack build --release --target web --no-typescript --out-dir ../../pkg/clocks --out-name clocks examples/clocks
mkdir -p pkg/clocks/analog-clock
cp examples/clocks/src/analog_clock/analog-clock.html pkg/clocks/analog-clock/
mkdir -p pkg/clocks/digital-clock
cp examples/clocks/src/digital_clock/digital-clock.html pkg/clocks/digital-clock/

wasm-pack build --release --target web --no-typescript --out-dir ../../pkg/finance --out-name finance examples/finance