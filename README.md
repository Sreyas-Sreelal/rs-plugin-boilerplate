# rs-plugin-boilerplate
Boilerplate for developing SA-MP plugins in Rust

## Installation

### Using cargo generate
* Install cargo generate (requires git,open-ssl)

	`cargo install cargo generate`
* Setup project using cargo generate

	`cargo generate --git https://github.com/Sreyas-Sreelal/rs-plugin-boilerplate.git --name YourProjectName`
### Alternate method
* Clone this repo

	`git clone https://github.com/Sreyas-Sreelal/rs-plugin-boilerplate.git`

* Rename the directory and project configuration in **Cargo.toml**

## Setup compiler
* Download [rustup](https://win.rustup.rs)
* Install nightly compiler 
	
	* **Windows**

		`rustup install nightly-i686-pc-windows-msvc`
	* **Linux**

		`rustup install nightly-i686-unknown-linux-gnu`
## Building
* **Windows**
	
	`cargo +nightly-i686-pc-windows-msvc build --release`
* **Linux**
	
	`cargo +nightly-i686-unknown-linux-gnu build --release`

## Notes
* Rust requires  [Microsoft Visual C++ Build Tools 2017](https://www.visualstudio.com/downloads/#build-tools-for-visual-studio-2017) to build in Windows

* Install `libgcc_32` libs for linux   

