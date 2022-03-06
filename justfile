crate := "bevy-sports-unfair-tennis"
web-target := "wasm32-unknown-unknown"
out-dir := "dist"

clean:
    cargo clean
    rm -r {{out-dir}}

build *FLAGS:
    cargo build --features bevy/dynamic {{FLAGS}}

run *FLAGS:
    cargo run --features bevy/dynamic {{FLAGS}}

package:
    mkdir -p {{out-dir}}
    cp index.html {{out-dir}}
    cp -r assets {{out-dir}}
    cargo build --target {{web-target}} --release
    wasm-bindgen --target web --out-name {{crate}} --out-dir {{out-dir}}/target target/{{web-target}}/release/{{crate}}.wasm
    wasm-opt -Oz --output {{out-dir}}/target/{{crate}}_bg.wasm dist/target/{{crate}}_bg.wasm
    zip -r {{crate}}.zip {{out-dir}}

serve:
    python3 -m http.server --directory {{out-dir}}
