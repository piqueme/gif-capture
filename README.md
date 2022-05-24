# GIF Capture

This is a simple tool for capturing screen area over time as a GIF.

## Features + Roadmap
- [x] Select screen area for capture
- [x] Specify duration and frame rate for capture
- [x] Create GIF at CLI-specified output file
- [ ] Simple shortcut installation to desktop panel
- [ ] Use dialog for duration and frame rate if not specified
- [ ] Use file picker for output if not specified

## Tech Improvements
- [ ] Some sort of integration tests
- [ ] Clean up type assumptions and casting (e.g. u32 -> usize)
- [ ] Clean up error handling (avoid `unwrap`)
- [ ] Performance - Aggressive pixel sampling for GIF palette
- [ ] Performance - Stream editing of captured frames
