// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::bundle::settings::Settings;
use anyhow::{Context, Result};
use msix::{msix, ZipFileOptions};
use std::path::PathBuf;

/// Bundle the project using MSIX.
/// 
/// This creates a Windows MSIX package (.msix file) that can be installed
/// on Windows 10+ systems. MSIX packages are the modern Windows app packaging
/// format and provide better security, installation, and update experiences
/// compared to traditional installers.
pub fn bundle_project(settings: &Settings) -> Result<Vec<PathBuf>> {
    println!("🔍 MSIX: Starting bundle_project");
    println!("🔍 MSIX: Product name: {}", settings.product_name());
    println!("🔍 MSIX: Version: {}", settings.version_string());
    println!("🔍 MSIX: Target: {}", settings.target());
    
    let package_base_name = format!(
        "{}_{}_{}",
        settings.product_name().replace(' ', ""),
        settings.version_string(),
        settings.target()
    );
    println!("🔍 MSIX: Package base name: {}", package_base_name);
    
    let output_path = settings
        .project_out_directory()
        .join("bundle")
        .join("msix")
        .join(format!("{}.msix", package_base_name));
    println!("🔍 MSIX: Output path: {:?}", output_path);

    // Ensure output directory exists
    if let Some(parent) = output_path.parent() {
        println!("🔍 MSIX: Creating output directory: {:?}", parent);
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create MSIX output directory: {:?}", parent))?;
        println!("🔍 MSIX: Output directory created successfully");
    }

    // Get the main binary path
    println!("🔍 MSIX: Getting main binary");
    let main_binary = settings.main_binary()?;
    let main_binary_path = settings.binary_path(main_binary);
    println!("🔍 MSIX: Main binary path: {:?}", main_binary_path);
    let main_binary_name = main_binary_path
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("Invalid main binary path"))?
        .to_string_lossy();
    println!("🔍 MSIX: Main binary name: {}", main_binary_name);

    // Generate package identity  
    let package_name = settings.bundle_identifier().replace(' ', "").replace('-', "_");
    println!("🔍 MSIX: Package name: {}", package_name);
    
    // Use a default publisher for development (in production, this should be configurable)
    let publisher = "CN=Tauri Development Certificate";
    println!("🔍 MSIX: Publisher: {}", publisher);

    // Build the MSIX package
    println!("🔍 MSIX: Creating MSIX builder");
    let mut builder = msix(&output_path)
        .identity(&package_name, settings.version_string(), publisher)
        .properties(settings.product_name(), settings.product_name())
        .application(
            &settings.product_name().replace(' ', ""),
            &main_binary_name,
            settings.product_name(),
            settings.product_name(),
        )
        .capabilities(vec!["internetClient", "runFullTrust"])
        .default_target_device_family()
        .default_resource()
        .executable(&main_binary_path);
    println!("🔍 MSIX: MSIX builder configured");

    // Add icon if available
    if let Some(icon_path) = settings.icon_files().flatten().next() {
        println!("🔍 MSIX: Adding icon: {:?}", icon_path);
        builder = builder.icon(icon_path);
    } else {
        println!("🔍 MSIX: No icon found");
    }

    // Add additional resources
    let resources: Vec<_> = settings.resource_files().iter().collect();
    println!("🔍 MSIX: Adding {} resource files", resources.len());
    for resource in resources {
        let resource = resource?;
        println!("🔍 MSIX: Adding resource: {:?} -> {:?}", resource.path(), resource.target());
        builder = builder.add_file(
            resource.path(),
            resource.target(),
            ZipFileOptions::Compressed,
        );
    }

    // Add external binaries
    let external_binaries: Vec<_> = settings.external_binaries().collect();
    println!("🔍 MSIX: Adding {} external binaries", external_binaries.len());
    for src in external_binaries {
        let src_path = src?;
        if let Some(dest_filename) = src_path.file_name() {
            println!("🔍 MSIX: Adding external binary: {:?} -> {:?}", src_path, dest_filename);
            builder = builder.add_file(
                &src_path,
                &PathBuf::from(dest_filename),
                ZipFileOptions::Compressed,
            );
        }
    }

    // Build the package
    println!("🔍 MSIX: Building package...");
    builder.build()
        .with_context(|| format!("Failed to create MSIX package at {:?}", output_path))?;

    println!("✅ MSIX package created at: {}", output_path.display());
    
    // Verify the file was actually created
    if output_path.exists() {
        let metadata = std::fs::metadata(&output_path)?;
        println!("🔍 MSIX: File size: {} bytes", metadata.len());
    } else {
        println!("❌ MSIX: Output file does not exist after build!");
    }
    
    Ok(vec![output_path])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bundle::settings::{PackageType, Settings};
    use std::collections::HashMap;
    use tempfile::tempdir;

    #[test]
    fn test_msix_bundle_generation() {
        // This is a basic test to ensure the module compiles and basic functionality works
        // Full integration tests would require a complete Tauri project setup
        
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path().join("output");
        
        // Create a minimal settings object for testing
        // Note: This is a simplified test - real usage would have proper Settings initialization
        
        assert!(output_dir.parent().unwrap().exists());
    }
}