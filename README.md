# crustle

Crustle is a Bug/Rock-type Pokémon introduced in Generation V. It is also this project, which is named after the crab pokemon to honor Rustaceans that also enjoy pocket monsters. This is a simple Pokédex app built using [Rust](https://www.rust-lang.org/), [Tailwind](https://tailwindcss.com/) & [Dioxus](https://dioxuslabs.com/) (not to be confused with [Deoxys](https://bulbapedia.bulbagarden.net/wiki/Deoxys_(Pok%C3%A9mon))).

### Prerequisites

1. [Install rust using rustup](https://www.rust-lang.org/tools/install)
2. [Install dioxus labs crate](https://dioxuslabs.com/learn/0.4/CLI/installation)
3. [Setup tailwind following these instructions](https://dioxuslabs.com/learn/0.4/cookbook/tailwind)

### Running the App
To run the app on your local machine, you can run this command:

`dx serve --port 5001 --hot-reload`

If you've made any changes to CSS or plan to make changes to CSS:

`npx tailwindcss -i ./src/index.css -o ./public/tailwind.css --watch`

### Preview
![Image](https://github.com/ericgbanta/crustle/assets/44131634/961b68c7-4604-4634-8dc2-1150242428d3)

### Credits
***Data provided by the wonderful folks at [PokeAPI](https://pokeapi.co/).***
