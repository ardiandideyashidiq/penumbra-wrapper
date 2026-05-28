/*
    SPDX-License-Identifier: AGPL-3.0-or-later
    SPDX-FileCopyrightText: 2025 Shomy
*/

use crate::error::AppError;
use crate::models::scatter::{ScatterFile, ScatterPartition};

pub struct ScatterParser;

impl ScatterParser {
    /// Parse scatter file using the mtk-scatter-parser library.
    /// Auto-detects XML vs YAML format internally.
    pub fn parse(file_path: &str) -> Result<ScatterFile, AppError> {
        let scatter = mtk_scatter_parser::parse_scatter(file_path)
            .map_err(|e| AppError::Parse(format!("Failed to parse scatter file: {e}")))?;

        let storage_type = scatter
            .layouts
            .keys()
            .find(|k| k.to_uppercase() == "UFS")
            .or_else(|| scatter.layouts.keys().find(|k| k.to_uppercase() == "EMMC"))
            .or_else(|| scatter.layouts.keys().next())
            .cloned()
            .unwrap_or_default();

        let selected = scatter.selected_layouts(mtk_scatter_parser::StorageSelect::Auto);
        let partitions: Vec<ScatterPartition> = selected.into_values().flatten()
            .into_iter()
            .map(|p| ScatterPartition {
                index: p.index.unwrap_or_default(),
                partition_name: p.name,
                file_name: p.file_name,
                is_download: p.is_download,
                partition_type: p.image_type.unwrap_or_default(),
                linear_start_addr: format!("{:#x}", p.linear_start),
                physical_start_addr: format!("{:#x}", p.physical_start),
                partition_size: format!("{:#x}", p.size),
                region: p.region,
                storage: p.storage.unwrap_or_default(),
                operation_type: p.operation_type.unwrap_or_default(),
            })
            .collect();

        Ok(ScatterFile {
            platform: scatter.platform.unwrap_or_default(),
            project: scatter.project.unwrap_or_default(),
            storage_type,
            partitions,
            file_path: file_path.to_string(),
        })
    }
}
