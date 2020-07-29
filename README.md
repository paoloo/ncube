# Ncube

> Enter the hypercube.

<div align="center">

![Ncube Screenshot](https://raw.githubusercontent.com/critocrito/ncube/master/resources/screenshots/ncube.png)

[Installation](#installation) • [Documentation](#documentation) • [Ncube Discovery](#ncube-discovery) • [Development](#development)

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/critocrito/ncube/Build%20Status?style=flat-square)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/critocrito/ncube?color=orange&style=flat-square)
![GitHub](https://img.shields.io/github/license/critocrito/ncube?color=blue&style=flat-square)

</div>

---

_This project is in an early stage and probably not working right now._

## Installation

### Linux

Currently there is only [a `deb` package for Ubuntu 18.04](https://github.com/critocrito/ncube/releases/latest). For any other Linux distribution build Ncube [from source](#from-source).

Install the provided `deb` package by either double-clicking it in the file browser or using a terminal:

```sh
sudo dpkg -i ncube_<version>_amd64.deb
```

### macOS

Ncube can be installed using the [DMG installer image](https://github.com/critocrito/ncube/releases/latest). You can install Ncube by double-clicking the `Ncube_<version>.dmg` file and dragging the application into your `Applications` folder. Newer versions of macOS might refuse to install the package since Ncube is not verified by Apple. To install Ncube follow the following steps:

1. Try to run Ncube.
2. On your Mac, choose Apple menu > System Preferences, click Security & Privacy, then click General.
3. Click the lock icon to unlock it, then enter an administrator name and password.
4. Click on Open Anyway to allow Ncube on your computer.

![macOS Security & Privacy preferences](https://raw.githubusercontent.com/critocrito/ncube/master/resources/screenshots/mac-preferences.png)

### From Source

The following prerequesites are required to build Ncube from source:

- A recent version of [Rust](https://www.rust-lang.org/).
  [Rustup](https://rustup.rs/) is a great way to do so. The minimum supported
  version is 1.40.0+.
- [NodeJS](https://nodejs.org/en/) to build and bundle the UI. Version 12 of
  NodeJS is tested, but it might work with other versions as well.
- NodeJS comes with `npm` as a package manager. This project prefers to use
  [`yarn`](https://classic.yarnpkg.com/en/). While it is possible to use `npm`
  all the build scripts assume `yarn`.
  [Installation](https://classic.yarnpkg.com/en/docs/install#mac-stable) is
  quite simple.
- Builds are orcehstrated using [`make`](https://www.gnu.org/software/make/).
  macOS and most Linux distributions have `make` installed out of the box.

Verify that all build dependencies are satisfied.

```sh
$ cargo --version
rustc 1.42.0 (b8cedc004 2020-03-09)

$ node --version
v12.16.1

$ yarn --version
1.22.0

$ make --version
GNU Make 3.81
```

Once all the dependencies are in place you can choose one of the following build targets. The build target that you most likely want can be simply build by running `make`:

- `make` :: Build a standalone binary of the desktop version of Ncube. The binary can be found in `target/release/ncube`.

There are more specialized build targets as well:

- `make server` :: Build the server version of Ncube. This produces two binaries `target/release/ncubed` and `target/release/ncubectl`.
- `make pkg-dmg` :: Build a DMG installer package of the desktop version of Ncube for macOS. The package can be found in `pkgs`.
- `make pkg-deb` :: Build a DEB package package of the desktop version of Ncube for Linux. The package can be found in `pkgs`.
- `make pkg-deb-ncubed` :: Build a DEB package of the server version of Ncube for Linux. The package can be found in `pkgs`.
- `make pkg-web-ext` :: Build a ZIP package of the Discovery Browser plugin. The package can be found in `pkgs`.

If something goes wrong or you want to make a clean build from scratch clean the old builds first:

- `make clean` :: Remove all previous build assets.

## Documentation

All documentation can be [found in the `doc`](doc) directory.

The HTTP endpoints of `ncubed` are described in the [HTTP API
documentation](doc/http-api.md).

The account authorization for remote workspaces is described in [a dedicated
document](doc/auth-workflow.pdf).

The high level architecture of Ncube is described with a series of [diagrams](doc/diagrams/ncube). To
re-generate the architecture diagrams install
[`fc4`](https://fundingcircle.github.io/fc4-framework/docs/get-started) and
regenerate the images:

```sh
fc4 -fsr doc/diagrams
```

## Ncube Discovery

Ncube is accompanied by a browser extension for the [Firefox browser](https://www.mozilla.org/en-US/firefox/new) for a smoother investigation workflow. The browser extension allows to store URL's from the browser as sources directly in Ncube.

![Discovery Browser Extension Screenshot](https://raw.githubusercontent.com/critocrito/ncube/master/resources/screenshots/discovery.png)

TODO: Publish extension to AMO and provide a download link.

The following instructions are to test and develop the browser extension locally. Make sure to install all required dependencies.

```sh
yarn install
```

To build the full extension and package it up simply run:

```sh
make pkg-web-ext
```

This will produce the package in the `pkgs` directory.

To develop locally run compilation of the web extension in one terminal:

```sh
yarn web-ext:watch
```

In another terminal start a development version of Firefox that contains the browser extension:

```sh
yarn web-ext
```

## Development

Ncube consists of several parts:

- `ncubed` is the backend of Ncube that exposes all functionality of Ncube.
- `ncubectl` is a CLI tool to manage server installations of Ncube.
- The frontend UI is a single page web app that communicates to `ncubed`.
- `ncube` is the full desktop app including `ncubed` and wrapping the frontend UI in a local browser window.

See the [installation from source](#from-source) section to setup all the build
dependencies.

Since the backend delivers the frontend using it's own HTTP server, the frontend
assets must be available when compiling the backend. there is an [open
issue](https://github.com/critocrito/ncube/issues/39) to resolve this but until
then before compiling the backend you need to run the following once, and
everytime you clean the `target` directory.:

```sh
make ui
make workspace
```

Open a terminal in the project root and start the backend:

```sh
cargo run --bin ncubed
```

In a different terminal start the webpack development server to build the UI:

```sh
yarn start
```

This will open a browser at `http://localhost:8080` that provides the UI.
Further this will start [React
Cosmos](https://github.com/react-cosmos/react-cosmos) on
`http://localhost:5000`.

### Tests

The tests can be run by executing the following command:

```sh
make test
```

This project provides [`devcards`](https://github.com/bhauman/devcards/) to
display its design system. They can be inspected at
`http://localhost:9500/figwheel-extra-main/devcards`.

A standalone version of `devcards` is build when compiling the production
distribution (`make/make build`). The output directory is `resources/dist`.

## License

All code is copyrighted by _christo@cryptodrunks.net_ and licensed under the [GPL3](https://www.gnu.org/licenses/gpl-3.0.html).
