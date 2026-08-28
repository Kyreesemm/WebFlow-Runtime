# Changelog

All notable changes to the **WebFlow Runtime** project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed
- Fixed the manual update check button being disabled when startup update checks are turned off.
- Showed a manager notification when an update is found during an enabled startup check.
- Prevented duplicate startup update notifications when manager settings are loaded more than once.

---

## [v0.2.0-dev] - August 24th, 2026

### Added
- Added a manager setting to confirm and enable update checks on every GUI startup.
- Added the project information tab with branding, version, runtime component information, update status, and project links.
- Added public GitHub release checks, compatible platform asset selection, SHA-256 verification, download progress, and cross-platform self-update installation.
- Changed the update action to become a gray “Check for Updates” button when the installed version is current, with in-app notifications for manual check results.
- Fixed Windows updater-helper compilation with the current `windows-sys` API.

### Fixed
- Corrected project and CLI author metadata to `KRM Tech Software`.

### Changed
- Restored the original vertical spacing of settings rows while keeping toggles centered.
- Centered settings toggles relative to each option's title and description.
- Split manager tray minimization and application tray minimization into separate settings, with application minimization dependent on application tray icons.
- The minimum size of the manager window has been expanded to 850 pixels.

---

## [v0.1.4-dev] - August 22nd, 2026

### Added
- Added developer settings for persistent manager and application file logging.
- Added detailed manager debug logging for UI events, IPC requests and responses, JavaScript errors, and backend timings.
- Added `--debug-verbose` for high-volume background debug logging.
- Added English CLI documentation covering all supported options and execution modes.
- Added `--debug-file` for detailed per-session debug logs next to the executable.
- Added application-level debug logging for direct `--app` launches, including UI events, JavaScript diagnostics, and console messages.

### Changed
- Reduced idle noise in the default `--debug` output and added timestamps with color-coded log categories.
- File debug logging includes high-volume background events without enabling terminal output.

### Fixed
- Fixed the CLI version output to follow the package version.

---

## [v0.1.3-dev] - August 21st, 2026

### Added
- Added system autostart integration for the manager on Linux and Windows.
- Added an option to start the manager minimized to the system tray during autostart.

### Changed
- The descriptions of some functions in the manager's interface have been minimally changed.

### Fixed
- Fixed the startup-minimized option availability to follow both manager autostart and tray minimization settings.
- Added the manager icon to the Linux desktop autostart entry.
- Fixed `engine_settings.json` being duplicated in the application root on Windows.
- Disabled manager tray minimization on GNOME Linux sessions and forced it off by default there.
- Fixed new applications ignoring the default isolated storage setting.
- Fixed storage cleanup buttons having different sizes and corrected the Russian cache label.
- Fixed User-Agent strings overflowing their cards at the manager's minimum width.

---

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
