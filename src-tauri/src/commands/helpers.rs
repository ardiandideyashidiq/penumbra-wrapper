use crate::error::AppError;
use crate::models::{OperationCompleteEvent, OperationOutputEvent};
use crate::services::antumbra::AntumbraExecutor;
use chrono::Utc;
use std::fs::OpenOptions;
use std::path::Path;
use tauri::{AppHandle, Emitter};
use uuid::Uuid;

pub fn emit_operation_output(
    app: &AppHandle,
    operation_id: &str,
    line: &str,
    is_stderr: bool,
) {
    let event = OperationOutputEvent {
        operation_id: operation_id.to_string(),
        line: line.to_string(),
        timestamp: Utc::now().to_rfc3339(),
        is_stderr,
    };
    let _ = app.emit("operation:output", event);
}

pub fn emit_operation_complete(
    app: &AppHandle,
    operation_id: &str,
    success: bool,
    error: Option<String>,
) {
    let event = OperationCompleteEvent {
        operation_id: operation_id.to_string(),
        success,
        error,
    };
    let _ = app.emit("operation:complete", event);
}

pub fn result_with_emit<T>(
    result: Result<T, AppError>,
    app: &AppHandle,
    op_id: &str,
) -> Result<T, AppError> {
    match result {
        Ok(val) => Ok(val),
        Err(err) => {
            emit_operation_output(app, op_id, &err.message(), true);
            emit_operation_complete(app, op_id, false, Some(err.message()));
            Err(err)
        }
    }
}

pub fn validate_da_preloader_paths(
    da_path: &str,
    preloader_path: Option<&str>,
) -> Result<(), AppError> {
    validate_input_file(da_path, "DA file")?;
    if let Some(path) = preloader_path {
        validate_input_file(path, "Preloader file")?;
    }
    Ok(())
}

pub fn validate_input_file(path: &str, label: &str) -> Result<(), AppError> {
    let target = Path::new(path);
    if !target.is_file() {
        return Err(AppError::command(format!("{} not found: {}", label, path)));
    }
    validate_readable_file(target, label)?;
    Ok(())
}

pub fn validate_output_dir(path: &str, label: &str) -> Result<(), AppError> {
    let target = Path::new(path);
    if !target.is_dir() {
        return Err(AppError::command(format!("{} not found: {}", label, path)));
    }
    validate_writable_dir(target, label)?;
    Ok(())
}

pub fn validate_output_parent(path: &str, label: &str) -> Result<(), AppError> {
    let parent = Path::new(path)
        .parent()
        .ok_or_else(|| AppError::command(format!("{} has no parent: {}", label, path)))?;

    if !parent.is_dir() {
        return Err(AppError::command(format!(
            "{} parent directory not found: {}",
            label,
            parent.display()
        )));
    }

    validate_writable_dir(parent, label)?;
    Ok(())
}

pub fn is_dir_writable(path: &Path) -> bool {
    let test_name = format!(".penumbra-write-test-{}", Uuid::new_v4());
    let test_path = path.join(test_name);
    if let Ok(file) = std::fs::OpenOptions::new().write(true).create_new(true).open(&test_path) {
        drop(file);
        let _ = std::fs::remove_file(&test_path);
        return true;
    }
    false
}

fn validate_readable_file(path: &Path, label: &str) -> Result<(), AppError> {
    OpenOptions::new().read(true).open(path).map_err(|err| {
        AppError::command(format!("{} not readable: {} ({})", label, path.display(), err))
    })?;
    Ok(())
}

fn validate_writable_dir(path: &Path, label: &str) -> Result<(), AppError> {
    if !is_dir_writable(path) {
        return Err(AppError::command(format!(
            "{} not writable: {}",
            label,
            path.display()
        )));
    }
    Ok(())
}

pub async fn execute_antumbra_command(
    app: AppHandle,
    operation_id: String,
    da_path: &str,
    preloader_path: Option<&str>,
    args: Vec<String>,
) -> Result<(), AppError> {
    validate_da_preloader_paths(da_path, preloader_path)?;
    let executor = AntumbraExecutor::new(&app)?;

    let mut cmd_args = args;
    cmd_args.push("-d".to_string());
    cmd_args.push(da_path.to_string());

    if let Some(pl) = preloader_path {
        cmd_args.push("-p".to_string());
        cmd_args.push(pl.to_string());
    }

    executor
        .execute_streaming(app, operation_id, cmd_args)
        .await
        .map_err(|e| AppError::command(e.to_string()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_is_dir_writable_on_temp() {
        assert!(is_dir_writable(&std::env::temp_dir()));
    }

    #[test]
    fn test_is_dir_writable_on_nonexistent() {
        assert!(!is_dir_writable(&PathBuf::from("/nonexistent/path")));
    }

    #[test]
    fn test_is_dir_writable_on_file() {
        let temp = std::env::temp_dir();
        let file_path = temp.join(".penumbra-write-test-file");
        fs::write(&file_path, "test").unwrap();
        assert!(!is_dir_writable(&file_path));
        let _ = fs::remove_file(&file_path);
    }

    #[test]
    fn test_validate_input_file_nonexistent() {
        assert!(validate_input_file("/nonexistent/file.txt", "Test").is_err());
    }

    #[test]
    fn test_validate_da_preloader_paths_nonexistent() {
        assert!(validate_da_preloader_paths("/nonexistent/da.bin", None).is_err());
    }
}
