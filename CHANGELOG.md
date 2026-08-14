# Changelog

All notable changes to the **WebFlow Runtime** project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

### Changed
- The logic of working with memory in all usage scenarios has been changed.

### Fixed
- Fixed the WebFlow Runtime Manager window icon on Linux Wayland sessions by using the working XWayland fallback when available.
- Fixed Windows executable icon resource embedding for native Windows builds.
- Resolved rendering lifecycle issues and improved engine performance and resource efficiency.
- Minor bugs have been fixed in the Windows version.

---

## [v0.1.0-dev] - August 14th, 2026

### Added
- Added system tray support and manager minimization without closing the process.
- Added the WebFlow Runtime Manager executable icon for Linux and Windows.

### Changed
- The project was completely rewritten from Python to Rust.
- Updated CLI command arguments and interactive option handling.
- Some manager functions are temporarily unavailable.

### Fixed
- Fixed minor UI issues in WebFlow Runtime Manager.
- Fixed recalculation and display of cache and application data sizes.
- Fixed display of cookie sizes and storage data.
- Fixed User-Agent display and selection.
- Fixed shared storage cleanup so cache and cookies are removed independently.
- Fixed application icon display in the manager and individual application windows.
- Fixed Windows compilation and related Windows build issues.
