# ggorg.xyz

This is the code for my website, [ggorg.xyz](https://ggorg.xyz).

Of course, featuring the Catppuccin Mocha theme!

It's my first Yew.rs project ever!
For those who don't know it's a React-like framework for building websites in Rust.

It also contains a Nix flake for reproducible results.

Made with ❤️ in Neovim.

## Building

### Using Nix

```bash
nix build # build the project into `./result/`

nix run # build and serve the project with `darkhttpd` on port `8080`
```

### The normal way

Install [Rust](https://rustup.rs/) and [Trunk](https://trunkrs.dev/#install).

After that, run:

```bash
trunk serve # to build the project in debug mode and serve it with live reloading

trunk build --release # to build the project in release mode into `./dist/`
```
