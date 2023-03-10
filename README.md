# RustyNes
Rust/WASM NES Emulator using HTML5 Canvas. Only compatible with very simple ROMs.

# Usage/Testing

* The test used to make sure the CPU works is the [nestest] (https://wiki.nesdev.com/w/index.php/Emulator_tests) rom from kevtris.
* Few if any other tests will work, as they are all more rigorous and use other mappers/are concerned with timing/etc
* Few actual games will work. I recommend the original Super Mario game if one wishes to test something that works for sure (original DK works too)

## To build
* Make sure you have Rust
* install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) you can use cargo `cargo install wasm-pack`
* install `nodejs/npm`
* run `wasm-pack build --release`
* run `npm install`
* run `npm run build`
* run `npm run serve`
* open browser to `127.0.0.1:4444` this can be changed in webpack.config.js


## Sources
* [The Nes Ebook](https://bugzmanov.github.io/nes_ebook/chapter_1.html)
* [Nesdev Wiki](https://wiki.nesdev.com)
* [NES documentation](http://nesdev.com/NESDoc.pdf)
* [6502 Opcodes](http://www.6502.org/tutorials/6502opcodes.html)
* [More on opcodes](https://www.masswerk.at/6502/6502_instruction_set.html)
* [Good 6502 reference](http://www.obelisk.me.uk/6502/)