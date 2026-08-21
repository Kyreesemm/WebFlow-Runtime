# Changelog

All notable changes to the **WebFlow Runtime** project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

### Changed

### Fixed
- Fixed storage cleanup buttons having different sizes and corrected the Russian cache label.
- Fixed User-Agent strings overflowing their cards at the manager's minimum width.

## [v0.1.2-dev] - August 20th, 2026

### Added
- Added confirmation and data migration options when changing the userdata directory.
- Added a stable userdata bootstrap configuration so all derived paths follow the active directory.

### Fixed
- Fixed derived userdata paths and folder-opening actions continuing to use the previous directory after migration.
- Fixed the userdata migration dialog layout at the manager's minimum window size.
- Preserved existing engine settings when saving partial settings updates.
- Prevented multiple WebFlow Runtime Manager processes and duplicate tray icons from being created.
- Optimized Linux tray minimize/restore and close handling while keeping the manager WebView unloaded in the tray.
- Fixed cross-platform cache and user-data size calculation and cleanup for isolated and shared WebView storage.

---

## [v0.1.1-dev] - August 15th, 2026

### Changed
- The logic of working with memory in all usage scenarios has been changed.

### Fixed
- Fixed custom scrollbar styles not being applied to launched applications on Linux and Windows.
- Fixed incomplete English localization of the Engine Settings tab, folder controls, and language switcher tooltip.
- Added minimum and maximum window size limits for the manager and applications on Linux and Windows.
- Fixed the WebFlow Runtime Manager window icon on Linux Wayland sessions by using the working XWayland fallback when available.
- Fixed Windows executable icon resource embedding for native Windows builds.
- Resolved rendering lifecycle issues and improved engine performance and resource efficiency.
- Minor bugs have been fixed in the Windows version.

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
