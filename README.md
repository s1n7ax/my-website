# my-website (WIP)
my website written in web assembly

## DEMO

[https://s1n7ax.github.io/my-website/](https://s1n7ax.github.io/my-website/)

![screenshot](./screenshots/screenshot_1.png)

# Dev Setup
* Install `rustc <= 1.59`
* Add build target `rustup target add wasm32-unknown-unknown`
* Install trunk `cargo install trunk`
* Change directory to `node_dependencies` and run `yarn`
* Change directory back to project root
* Run the application `trunk serve`
* Access the application at `http://127.0.0.1:8080`
