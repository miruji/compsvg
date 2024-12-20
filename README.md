# compsvg

For now it just compresses the SVG into a .zst file so that it can be transferred more easily from the server to the client. It is assumed that your SVG file has already been optimized and shortened as much as possible for the look you need.

Can be used `cargo run --release -- <input file>.svg <output file>`

Or you can `cargo build` and use `compsvg <input file>.svg <output file>`
