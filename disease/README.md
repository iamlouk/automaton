# Simulation of the spread of a disease like Covid-19.

Idea taken from [here](https://www.washingtonpost.com/graphics/2020/world/corona-simulator/). This Project was created in order to play around with [WebAssembly](https://webassembly.org/). It uses (wasm-pack and wasm-bindgen)[https://rustwasm.github.io/].

## Build/Run:
```sh
# needed to generate wasm
cargo install wasm-pack 

# get this project
git clone https://gitlab.com/iamlouk/simulations.git
cd simulations/disease

# build pkg/ and target/
# wasm-pack will run cargo for you
wasm-pack build --target web

# optional: run webserver on local machine
python3 -m http.server --directory . 8080

# open the simulation in your browser
firefox ./index.html
```

## Todo:
- [ ] Settings: `scale`
