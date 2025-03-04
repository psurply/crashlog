// Copyright (C) 2025 Intel Corporation
// SPDX-License-Identifier: MIT

use super::{CollateralManager, CollateralTree, ItemPath, PVSS};
use crate::Error;
use std::io::Read;
use std::path::{Path, PathBuf};

/// Provides access to a collateral tree stored in the file system.
///
/// The directories in the collateral tree must follow this structure:
/// `PRODUCT/VARIANT/STEPPING/SECURITY/crashlog/ITEM_PATH`
///
/// A [`CollateralManager`] that uses a collateral tree stored in the file system can be created
/// using the [`CollateralManager::file_system_tree`] function.
pub struct FileSystemTree {
    root: PathBuf,
}

impl FileSystemTree {
    fn new(root: &Path) -> Self {
        Self {
            root: root.to_path_buf(),
        }
    }

    fn build_path(&self, pvss: &PVSS, item: &ItemPath) -> Option<PathBuf> {
        let mut relative_base = self.root.clone();
        let pvss: PathBuf = pvss.into();
        relative_base.extend(&pvss);
        relative_base.push("crashlog");

        let base = relative_base.canonicalize().ok()?;

        let mut path = base.clone();
        let item: PathBuf = item.into();
        path.extend(&item);

        path.canonicalize()
            .ok()
            .filter(|path| path.starts_with(&base))
    }
}

impl CollateralTree for FileSystemTree {
    fn get(&self, pvss: &PVSS, item: &ItemPath) -> Result<Vec<u8>, Error> {
        let Some(path) = self.build_path(pvss, item) else {
            return Err(Error::MissingCollateral(pvss.clone(), item.clone()));
        };

        let mut file = std::fs::File::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        Ok(buf)
    }

    fn search(&self, item: &ItemPath) -> Result<Vec<PVSS>, Error> {
        let mut hits = Vec::new();

        for product in std::fs::read_dir(&self.root)? {
            let product = product?;
            for variant in std::fs::read_dir(product.path())? {
                let variant = variant?;
                for stepping in std::fs::read_dir(variant.path())? {
                    let stepping = stepping?;
                    for security in std::fs::read_dir(stepping.path())? {
                        let security = security?;
                        let mut path = security.path();
                        path.push("crashlog");
                        path.push::<PathBuf>(item.into());
                        if path.exists() {
                            hits.push(PVSS {
                                product: product.file_name().into_string()?,
                                variant: variant.file_name().into_string()?,
                                stepping: stepping.file_name().into_string()?,
                                security: security.file_name().into_string()?,
                            });
                        }
                    }
                }
            }
        }

        Ok(hits)
    }
}

impl CollateralManager<FileSystemTree> {
    /// Creates a [`CollateralManager`] that uses a collateral tree stored in the file system.
    /// The collateral items will be loaded at run-time from the collateral tree located at the
    /// path specified in the `root` argument.
    ///
    /// # Examples
    ///
    /// ```
    /// use intel_crashlog::prelude::*;
    /// use std::path::Path;
    ///
    /// let collateral_manager = CollateralManager::file_system_tree(Path::new("collateral"));
    /// assert!(collateral_manager.is_ok());
    /// ```
    pub fn file_system_tree(root: &Path) -> Result<Self, Error> {
        Self::new(FileSystemTree::new(root))
    }
}
