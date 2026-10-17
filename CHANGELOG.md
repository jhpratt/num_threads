# 0.1.8

- Fall back to `/proc/self/status` on Linux when reading `/proc/self/stat` fails
- Report the correct thread count when running under Rosetta on macOS
- Drop support for 32-bit Apple targets

# 0.1.7

Fix memory leak on Apple platforms

# 0.1.6

- Avoid heap allocation on FreeBSD

# 0.1.5

- Add support for Android and iOS

# 0.1.4

- Modified MacOS implementation to be friendly towards Apple app store review guidelines.

# 0.1.3

- Avoid heap allocation on MacOS.

# 0.1.2

- Add support for MacOS

# 0.1.1

- Fix memory leak on FreeBSD
- Improve performance on Linux

# 0.1.0

- Initial release with support for Linux, FreeBSD
