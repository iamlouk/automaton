# Simulation of charged particles

This Project was created in order to play around with [WebAssembly](https://webassembly.org/). It uses (wasm-pack and wasm-bindgen)[https://rustwasm.github.io/].

## Build/Run:
```sh
# needed to generate wasm
cargo install wasm-pack 

# get this project
git clone https://gitlab.com/iamlouk/simulations.git
cd simulations/inverse-square-law

# build pkg/ and target/
# wasm-pack will run cargo for you
wasm-pack build --target web

# optional: run webserver on local machine
python3 -m http.server --directory . 8080

# open the simulation in your browser
firefox ./index.html
```
