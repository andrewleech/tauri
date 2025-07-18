# [DRAFT] Add MSIX packaging support for Windows

## Summary

This PR adds comprehensive MSIX (.msix) package support to tauri-bundler, addressing the long-standing feature request in #4818. MSIX packages provide modern Windows app packaging with better security, installation, and update experiences compared to traditional installers.

## Key Features

### 🎯 Complete MSIX Integration
- **New PackageType**: `WindowsMsix` with CLI support `--format msix`
- **Native Bundler**: Full integration with Tauri's bundler architecture
- **Cross-platform**: Build Windows MSIX packages from Linux/macOS
- **Updater Support**: Works with Tauri's update system

### 🛠️ Comprehensive Package Creation
- **Package Identity**: Automatic identity generation from bundle configuration
- **Visual Elements**: App tiles, splash screens, logos with proper scaling
- **Capabilities**: Support for Windows app capabilities (`internetClient`, `runFullTrust`, etc.)
- **Asset Bundling**: Automatic inclusion of icons, resources, and external binaries
- **Code Signing**: Built-in signing with debug certificates (configurable)

### 🔧 Developer Experience
- **Familiar CLI**: `cargo tauri build --format msix` works alongside existing formats
- **Configuration Integration**: Uses existing `tauri.conf.json` bundle settings
- **Asset Pipeline**: Integrates with Tauri's icon and resource system
- **Error Handling**: Clear error messages and validation

## Usage

```bash
# Build MSIX package
cargo tauri build --format msix

# Build multiple formats including MSIX
cargo tauri build --format msi,msix,nsis

# Generated output: 
# target/release/bundle/msix/AppName_1.0.0_x86_64.msix
```

## Technical Implementation

### Architecture
- **Follows Tauri Patterns**: Implemented as `windows/msix/mod.rs` alongside `msi` and `nsis`
- **Settings Integration**: Extends existing `PackageType` enum and configuration system
- **Build Pipeline**: Integrates with patching, signing, and updater artifact generation

### Dependencies
- **msix crate**: Enhanced fork with builder pattern for ergonomic package creation
- **Cross-platform Tools**: No additional Windows-only dependencies required
- **Asset Processing**: Leverages existing Tauri asset and icon processing

### Code Organization
```
crates/tauri-bundler/src/bundle/windows/msix/
└── mod.rs              # Main MSIX bundler implementation

Changes to existing files:
├── settings.rs         # Add WindowsMsix PackageType
├── mod.rs             # Bundle dispatch and integration
└── lib.rs             # Updated documentation
```

## Example Generated Package

For a typical Tauri app, this generates a complete MSIX package with:
- **App Manifest**: Proper AppxManifest.xml with identity and capabilities
- **Visual Assets**: All required icon sizes (44x44, 150x150, 310x150, etc.)
- **Application Files**: Main executable and resources
- **Package Structure**: Valid MSIX format with proper signing

## Benefits Over Existing Solutions

### vs MSI (WiX)
- ✅ **Modern Windows experience** - Better installation UX
- ✅ **Automatic updates** - Built-in Windows update mechanism  
- ✅ **Security sandbox** - Enhanced security model
- ✅ **Microsoft Store ready** - Can be distributed through store

### vs NSIS
- ✅ **Native Windows integration** - First-class Windows 10+ support
- ✅ **Cleaner uninstall** - No registry artifacts left behind
- ✅ **Faster installation** - Optimized installation process
- ✅ **Differential updates** - Smaller update packages

## Compatibility

- **Windows 10+**: MSIX is supported on Windows 10 version 1709 and later
- **Cross-platform Building**: Build MSIX packages from any platform
- **Existing Workflows**: Works alongside existing bundle formats
- **Configuration**: Uses existing bundle configuration options

## Testing

- ✅ Compiles successfully on all platforms
- ✅ Generates valid MSIX packages
- ✅ Integration with existing Tauri bundler
- ✅ CLI compatibility with other formats
- ✅ Asset pipeline integration
- 🔄 **Needs**: Full integration testing with real Tauri apps

## Files Changed

### New Files
- `crates/tauri-bundler/src/bundle/windows/msix/mod.rs` - MSIX bundler implementation

### Modified Files
- `crates/tauri-bundler/src/bundle/settings.rs` - Add WindowsMsix PackageType
- `crates/tauri-bundler/src/bundle.rs` - Bundle dispatch and updater integration  
- `crates/tauri-bundler/src/bundle/windows/mod.rs` - Export msix module
- `crates/tauri-bundler/src/lib.rs` - Updated documentation
- `crates/tauri-bundler/Cargo.toml` - Add msix dependency

## Future Enhancements

- **Configuration Options**: More granular MSIX-specific configuration
- **Microsoft Store Integration**: Helpers for store submission
- **Enterprise Features**: Support for enterprise distribution
- **Custom Signing**: Integration with customer certificates

## Resolves

- Closes #4818 - Add MSIX packaging support

---

**Note**: This is a draft PR for testing purposes. The implementation provides a solid foundation for MSIX packaging in Tauri and follows established bundler patterns. Comprehensive testing with real applications is recommended before final merge.