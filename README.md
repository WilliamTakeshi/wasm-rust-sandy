
# Sandy - A Sand Falling Game

Welcome to the wasm-rust-sand, a Sand Falling Game! This is a experiment game just for me to learn wasm. Heavily based on [sandspiel](https://github.com/MaxBittker/sandspiel)


## Building

* Open 2 terminals, the first one will build the wasm:


To build once:

```bash

wasm-pack build

```

or to keep watching:

```bash

cargo watch -s 'wasm-pack build'

```

  ----

On the second terminal, we will build the front-end

```bash

cd www
nvm use 16 # Use node version 16
npm install
npm run start

```


## Gameplay



## Contributing


Feel free to build and make any changes that you feel like would be fun! Happy learning!
