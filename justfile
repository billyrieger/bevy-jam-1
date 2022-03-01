title := "Ad Out"
crate := "bevy-unfair-tennis"
web-target := "wasm32-unknown-unknown"
web-dir := "dist"

clean:
    cargo clean
    rm -rf dist

run:
    cargo run --features bevy/dynamic --release

run-debug:
    cargo run --features bevy/dynamic

build-web:
    mkdir -p {{web-dir}}
    cp index.html {{web-dir}}
    cp -r assets {{web-dir}}
    cargo build --target {{web-target}} --release
    wasm-bindgen --out-name game --out-dir dist/target --target web target/{{web-target}}/release/{{crate}}.wasm
    zip -r "{{title}}".zip {{web-dir}}

serve:
    python3 -m http.server --directory dist