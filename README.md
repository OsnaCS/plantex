# :seedling: Plantex: open-world game about plants :evergreen_tree: :leaves: :herb: :palm_tree:

[![Build Status](https://travis-ci.org/OsnaCS/plantex.svg?branch=master)](https://travis-ci.org/OsnaCS/plantex)
[![License](https://img.shields.io/github/license/OsnaCS/plantex.svg)](http://www.apache.org/licenses/LICENSE-2.0)
[![license](https://img.shields.io/github/license/mashape/apistatus.svg?maxAge=2592000)](http://opensource.org/licenses/MIT)

This game was developed in a three week programming practical at the university of Osnabrück :-)

<p align="center"><a href="http://www.youtube.com/watch?feature=player_embedded&v=X1E-dWKzm-Y
" target="_blank"><img src="http://i.imgur.com/pXSWW5f.jpg" 
alt="Plantex Trailer" width="558" height="315" border="10"></img></a></p>


## Run the game

### Windows binaries

Precompiled binaries for Windows x64 can be downloaded on the [releases page](https://github.com/OsnaCS/plantex/releases). Latest build: [v0.1.0](https://github.com/OsnaCS/plantex/releases/download/v0.1.0/plantex-win.zip).

### Compile the game

For all other platforms you have to compile the game yourself. Clone this repository (or download it as ZIP file) and execute:

```bash
$ cargo build --release
```

After the compilation has finished, you can run the game by either executing the binary in `./target/release/` or executing:

```bash
$ cargo run --release --bin plantex
```

## Images

*TODO*

## Documentation

- [**`base`**](https://osnacs.github.io/plantex/base/index.html)
- [**`client`**](https://osnacs.github.io/plantex/client/index.html)
- [**`server`**](https://osnacs.github.io/plantex/server/index.html)

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

Development will probably stop after the practical has ended. If there is enough interest in the game idea, the game is probably rewritten from scratch (the code in this repository often is far from optimal). Don't hesitate to make suggestions or file PRs, though! Just keep in mind the status of this project...
