# crustle

Crustle is a Bug/Rock-type Pokémon introduced in Generation V. It is also this project, which is named after the crab pokemon to honor Rustaceans that also enjoy pocket monsters. This is a simple Pokédex app built using [Rust](https://www.rust-lang.org/), [Tailwind](https://tailwindcss.com/) & [Dioxus](https://dioxuslabs.com/) (not to be confused with [Deoxys](https://bulbapedia.bulbagarden.net/wiki/Deoxys_(Pok%C3%A9mon))).

### Prerequisites

1. [Install rust using rustup](https://www.rust-lang.org/tools/install)
2. Install the Dioxus CLI: `cargo install dioxus-cli`
3. Install Node.js (for Tailwind CSS)

### Running the App

One-time setup:

```
npm install
npm run css
```

Start the dev server:

```
dx serve --port 5001 --hot-reload true
```

If you're actively editing CSS, run this in a separate terminal to watch for changes:

```
npm run css:watch
```

### Linting with Clippy
This project uses clippy for [linting](https://github.com/rust-lang/rust-clippy). While CI/CD will run `cargo build` and `cargo test` commands, clippy linting is not required. If you want to lint using clippy, follow these steps:

1. Update rustup: `rustup update`
2. Install clippy: `rustup component add clippy`
3. Run the linter: `cargo clippy`

### Preview
![Image](https://github.com/ericgbanta/crustle/assets/44131634/961b68c7-4604-4634-8dc2-1150242428d3)

### Credits
***Data provided by the wonderful folks at [PokeAPI](https://pokeapi.co/).***
