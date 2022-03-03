title := "Bevy Sports: Tennis Coliseum"
crate := "bevy-unfair-tennis"
web-target := "wasm32-unknown-unknown"
web-dir := "dist"

clean:
    cargo clean
    rm -rf {{web-dir}}

run *FLAGS:
    cargo run --features bevy/dynamic {{FLAGS}}

build *FLAGS:
    cargo build --features bevy/dynamic {{FLAGS}}

package:
    mkdir -p {{web-dir}}
    cp index.html {{web-dir}}
    cp -r assets {{web-dir}}
    cargo build --target {{web-target}} --release
    wasm-bindgen --out-name game --out-dir {{web-dir}}/target --target web target/{{web-target}}/release/{{crate}}.wasm
    zip -r "{{title}}".zip {{web-dir}}

serve:
    python3 -m http.server --directory {{web-dir}}