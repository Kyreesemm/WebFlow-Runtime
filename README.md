# WebFlow Runtime Project (now on Rust)

<a href="LICENSE"><img alt="License" src="https://img.shields.io/badge/License-MIT-blue.svg"></a>
<img alt="Rust" src="https://img.shields.io/badge/Rust-1.94.1-000000?logo=rust&logoColor=white">
<img alt="Wry" src="https://img.shields.io/badge/GUI-Wry%20%2F%20Tao-333333?logo=rust&logoColor=white">
<img alt="Platform" src="https://img.shields.io/badge/Platform-Linux_%7C_Windows-0078D6?logo=linux&logoColor=white">
<img alt="Status" src="https://img.shields.io/badge/Status-In_Development-E65100">

<p align="center">
  <img src="docs/resources/webflow_banner.png" alt="Profile Banner" width="100%">
</p>

<p align="center">
  <b>This fork</b> of my WebFlow Runtime project has been <b>completely rewritten in Rust language</b>, instead of the heavy and slow <b>Python language</b>!
</p>

---

## Disclaimer & Warning

> [!WARNING]
> **Project Under Development!**
> 
> This software is not yet intended for daily use — only **dev builds** are currently available. 
> 
> If you are testing the application and encounter any bugs or issues, please report them by opening a GitHub [Issue](https://github.com/Kyreesemm/WebFlow-Runtime/issues).

This project is quite difficult to rewrite from Python technologies (PyQt and QtWebEngine) to the Rust language, so the project has not yet been fully developed. But work on it is in full swing, you can still download and test it. And your readings about bugs and problems will greatly help to finish the project faster.

---

## Quick Start & Build from Source Code

WebFlow Runtime is distributed as a portable application. No installer is
required: download the archive for your operating system, extract it, and run
the executable from the extracted directory.

### Option 1: Download a Release

#### Linux

1. Download the Linux release archive from the [Releases](https://github.com/Kyreesemm/WebFlow-Runtime/releases) page.
2. Extract the archive to a directory where you want to keep WebFlow Runtime.
3. Make sure the archive contains both `webflow-runtime` and the `materials/`
   directory next to it.
4. Install the required system libraries if they are not already installed:
   GTK3, WebKitGTK 4.1, and AppIndicator support.
5. Start the manager:

```bash
./webflow-runtime
```

The release archive is portable, but WebKitGTK and the other Linux desktop
libraries are provided by the operating system and are not bundled into the
archive.

#### Windows

1. Download the Windows release archive from the [Releases](https://github.com/Kyreesemm/WebFlow-Runtime/releases) page.
2. Download and install the **Microsoft Edge WebView2 Runtime** from the
   [official Microsoft page](https://developer.microsoft.com/ru-ru/microsoft-edge/webview2?form=MA13LH&cs=2647592484).
3. Extract the archive to a directory where you want to keep WebFlow Runtime.
4. Make sure the archive contains both `webflow-runtime.exe` and the
   `materials/` directory next to it.
5. Start `webflow-runtime.exe`.

WebFlow Runtime does not include a Windows installer. WebView2 is a separate
Windows runtime dependency and must be installed before the manager is started.

### Option 2: Build from Source Code

Clone the repository and open its directory:

```bash
git clone https://github.com/Kyreesemm/WebFlow-Runtime.git
cd WebFlow-Runtime
```

#### Linux

Install Rust with [rustup](https://rustup.rs/), then install the native
development packages required by Wry/WebKitGTK. On Arch Linux, for example:

```bash
sudo pacman -S --needed base-devel rust gtk3 webkit2gtk-4.1 libappindicator-gtk3
```

Build and run the release version:

```bash
cargo build --release
./target/release/webflow-runtime
```

The build script copies `materials/` next to the release executable.

#### Windows

Install the following prerequisites:

- Rust through [rustup](https://rustup.rs/);
- Visual Studio Build Tools with the **Desktop development with C++** workload;
- the **Microsoft Edge WebView2 Runtime** from the
  [official Microsoft page](https://developer.microsoft.com/ru-ru/microsoft-edge/webview2?form=MA13LH&cs=2647592484).

From PowerShell in the repository directory, build the release version:

```powershell
cargo build --release
```

The executable is created at `target\release\webflow-runtime.exe`, and the
build script copies `materials\` next to it. Keep both the executable and the
`materials\` directory together when moving the build to another Windows
computer. Start the manager with:

```powershell
.\target\release\webflow-runtime.exe
```

The same portable layout can be archived and distributed without creating an
installer.

---

## *Development by KRM Tech Software*
