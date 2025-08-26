HewDraw-Remix (referred to as "HDR" from here on out) is written in [Rust](https://www.rust-lang.org/), which means that we use the `rustc` and `cargo` toolchains.

We actually use a variation of `cargo`, called [`cargo-skyline`](https://github.com/jam1garner/cargo-skyline), which has been modified to target the Nintendo Switch when using the [Skyline](https://github.com/skyline-dev/skyline) modloader.

Here is a list of the tools/dependencies that we recommend to assist you in the development process:
* [Visual Studio Code](https://code.visualstudio.com/)
* [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer), an extension for Visual Studio Code
* [Python 3.9 or greater](https://www.python.org/downloads/release/python-390/) (used for a few scripts)
* [git-cli](https://git-scm.com/book/en/v2/Getting-Started-The-Command-Line) or [GitHub Desktop](https://desktop.github.com/)
* [sys-ftpd-light](https://github.com/cathery/sys-ftpd-light)
* [Ryujinx](https://github.com/Ryujinx/Ryujinx)


## Setting up `cargo-skyline`
First, you will need to have installed [rustup](https://rustup.rs/). **NOTE**: If you are on Linux or any other operating system which has it's own package manager, PLEASE install via the link provided, not your package manager (such as `apt-get` or `pacman`), otherwise you WILL run into major, difficult to resolve issues during the build process.

NOTE: As of right now, there is an issue with the latest rustup versions that cause cargo-skyline to no longer function properly. You can download the Windows installer for rustup 1.24.3 [here](https://static.rust-lang.org/rustup/archive/1.24.3/x86_64-pc-windows-msvc/rustup-init.exe).

After you have installed and set up rustup, it would be in your best interest to at least familiarize yourself with the `cargo` package manager and to do some simple applications to also familiarize yourself with the language if you have not already. If you are familiar with programming concepts and computer science, skimming the [rust book](https://doc.rust-lang.org/book/) will likely help get you up to speed with the language and it's paradigms.

Once you have done all of that, go ahead and go to your terminal and enter the following command:

```cargo install cargo-skyline```

This will begin compiling and installing the `cargo-skyline` toolchain to your system, which should include the modified standard library which is required to build HDR and other skyline plugins.

## Cloning HewDraw-Remix
Since HDR does not have any submodule dependencies, it is as simple as running

```
git clone https://github.com/HDR-Development/HewDraw-Remix
```
in a directory of your choice. This will clone the full HDR repository and should get you all set up to begin working with the source.

If you are using GitHub desktop, nagivate to the `File` menu and then click on `Clone repository...` which will take you to the Clone Repository menu. From there, click on the `URL` tab on the far right and copy `https://github.com/HDR-Development/HewDraw-Remix` into the `Repository URL` field. Under the `Local Path` field, choose an empty directory to clone the repo into and then click the `Clone` button and let GitHub clone the files to your device.

## Smashline
HDR uses a toolchain for writing code for Smash known as [smashline](https://github.com/blu-dev/smashline). If you are not yet familiar with smashline, please feel free to familiarize yourself with its concepts and capabilities over at the [smashline wiki](https://github.com/blu-dev/smashline/wiki).

In conjunction with smashline, we also use [smash_script](https://github.com/blu-dev/smash-script), which helps keep our compile times lower than using the ACMD language that smashline supports.

## Package Structure
The HDR package structure is split into numerous different crates in order to keep our compile times lower.

### `utils`
The `utils` crate contains HDR's custom module system, which we use to enable access to instance-specific information for each of Ultimate's `BattleObject`'s at runtime. It also contains code for offset-searching and development utilities (such as stack trace dumping).

### `hdr-macros`
The `hdr-macros` crate allows us (perhaps to the dismay of some) to reference files from the repository root at compile time. This enables us to modify files in the romfs, for example, and recompile HDR to reflect those changes (only if needed).

### `fighters`
The `fighters` crate contains all of the sub-crates for every fighter, and also an additional one for `common`. Every fighter crate is designed to have their code compiled either into the parent/standalone plugin and/or the [smashline development plugin](https://github.com/blu-dev/smashline/wiki/The-Development-Plugin). The intent of this is to allow for reloading of in-development code at runtime without having to restart the match (which can build up to a very large amount of time). The exception to this is the `common` crate, as it contains some code which cannot be reloaded at runtime (things such as common status script replacements or other symbol hooks).

More information on how to build targeting both the standalone and the development plugin coming soon.

### `romfs`
HDR's `romfs` is what [ARCropolis](https://github.com/Raytwo/ARCropolis) loads instead of the game's files. There is a `source` directory, which contains all of the files that we are storing on the repository, meaning they can easily be diffed in text form to track changes across different commits and prevent merge hell with all of the binary formats that smash uses. You'll notice things like `xml` and `yaml` files in there for param data and motion list data, respectively.

### `plugin`
The plugin is what actually gets built. It doesn't hold a lot of code itself, just some of the feature configuration to separate between standalone and development plugins.

## Building
While you can just build inside of the `plugin` folder, we recommend using the build scripts which can be fund under `scripts`. These scripts include build steps for running on console, as well as on PC through Ryujinx. To build and send the files to console, just run `build-nx.sh <ip>`, which will automatically build the plugin (and all romfs files) and send them to the Switch over FTP.

For building on WSL, we recommend that you have [Ryujinx](https://github.com/Ryujinx/Ryujinx) installed on your local Windows installation as it will attempt to install it through your Ryujinx path in `%AppData%`. After running `build-wsl.sh`, launching Smash in Ryujinx (with Skyline and ARCropolis dependencies properly configured) should launch what you just built.

Alternatively, if you have python3 installed, you can use the `build.py` and `make_dist.py` scripts to build you the `hdr-ryujinx` and `hdr-switch` packages in the `build` and `distribution` folders. Using `build.py help` will display the usage for the scripts. These scripts allow you to specify characters to build into a `development.nro` instead of the main `libhdr.nro`, which can be used for smashline development runtime reloading. Any characters you build into the `development.nro` instead can be reloaded using utaunt. This means you can work on individual character acmd, opff, etc by rebuilding a new `development.nro`, copying it to your switch/ryujinx, and then just taunting. The plugin should reload, and you should see your changes instantly, **without having to restart the game.**