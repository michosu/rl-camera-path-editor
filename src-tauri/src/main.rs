#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::fs;

// Camera data structures
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Position {
    #[serde(rename = "X")]
    x: f64,
    #[serde(rename = "Y")]
    y: f64,
    #[serde(rename = "Z")]
    z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Rotation {
    #[serde(rename = "Pitch")]
    pitch: i32,
    #[serde(rename = "Roll")]
    roll: i32,
    #[serde(rename = "Yaw")]
    yaw: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CameraKeyframe {
    #[serde(rename = "FOV")]
    fov: f64,
    #[serde(rename = "Frame")]
    frame: i32,
    #[serde(rename = "Position")]
    position: Position,
    #[serde(rename = "Rotation")]
    rotation: Rotation,
    #[serde(rename = "Timestamp")]
    timestamp: f64,
    #[serde(rename = "Weight")]
    weight: f64,
}

// Type alias for camera data
type CameraData = HashMap<String, CameraKeyframe>;

// ========================================
// FILE OPERATIONS
// ========================================

#[tauri::command]
fn load_camera_file(path: String) -> Result<String, String> {
    fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read file: {}", e))
}

#[tauri::command]
fn save_camera_file(path: String, content: String) -> Result<(), String> {
    fs::write(&path, content)
        .map_err(|e| format!("Failed to write file: {}", e))
}

// ========================================
// FOV TRANSFORMATIONS
// ========================================

#[tauri::command]
fn transform_fov_add(data: String, add_value: f64) -> Result<String, String> {
    let mut camera_data: CameraData = serde_json::from_str(&data)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    
    for (_, keyframe) in camera_data.iter_mut() {
        keyframe.fov += add_value;
    }
    
    serde_json::to_string_pretty(&camera_data)
        .map_err(|e| format!("Failed to serialize: {}", e))
}

#[tauri::command]
fn transform_fov_multiply(data: String, multiplier: f64) -> Result<String, String> {
    let mut camera_data: CameraData = serde_json::from_str(&data)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    
    for (_, keyframe) in camera_data.iter_mut() {
        keyframe.fov *= multiplier;
    }
    
    serde_json::to_string_pretty(&camera_data)
        .map_err(|e| format!("Failed to serialize: {}", e))
}

#[tauri::command]
fn transform_fov_set(data: String, fov_value: f64) -> Result<String, String> {
    let mut camera_data: CameraData = serde_json::from_str(&data)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    
    for (_, keyframe) in camera_data.iter_mut() {
        keyframe.fov = fov_value;
    }
    
    serde_json::to_string_pretty(&camera_data)
        .map_err(|e| format!("Failed to serialize: {}", e))
}

// ========================================
// POSITION TRANSFORMATIONS
// ========================================

#[tauri::command]
fn transform_position_offset(data: String, x: f64, y: f64, z: f64) -> Result<String, String> {
    let mut camera_data: CameraData = serde_json::from_str(&data)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    
    for (_, keyframe) in camera_data.iter_mut() {
        keyframe.position.x += x;
        keyframe.position.y += y;
        keyframe.position.z += z;
    }
    
    serde_json::to_string_pretty(&camera_data)
        .map_err(|e| format!("Failed to serialize: {}", e))
}

#[tauri::command]
fn transform_position_scale(data: String, x: f64, y: f64, z: f64) -> Result<String, String> {
    let mut camera_data: CameraData = serde_json::from_str(&data)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    
    for (_, keyframe) in camera_data.iter_mut() {
        keyframe.position.x *= x;
        keyframe.position.y *= y;
        keyframe.position.z *= z;
    }
    
    serde_json::to_string_pretty(&camera_data)
        .map_err(|e| format!("Failed to serialize: {}", e))
}

// ========================================
// ROTATION TRANSFORMATIONS
// ========================================

#[tauri::command]
fn transform_rotation_offset(data: String, pitch: i32, yaw: i32, roll: i32, use_degrees: bool) -> Result<String, String> {
    let mut camera_data: CameraData = serde_json::from_str(&data)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    
    // Convert degrees to Unreal units if needed (1 degree = 182.04 UU)
    let (p, y, r) = if use_degrees {
        ((pitch as f64 * 182.04) as i32,
         (yaw as f64 * 182.04) as i32,
         (roll as f64 * 182.04) as i32)
    } else {
        (pitch, yaw, roll)
    };
    
    for (_, keyframe) in camera_data.iter_mut() {
        keyframe.rotation.pitch += p;
        keyframe.rotation.yaw += y;
        keyframe.rotation.roll += r;
    }
    
    serde_json::to_string_pretty(&camera_data)
        .map_err(|e| format!("Failed to serialize: {}", e))
}

// ========================================
// MIRROR OPERATION
// ========================================

#[tauri::command]
fn transform_mirror(
    data: String,
    axis: String,
    flip_pitch: bool,
    flip_yaw: bool,
    flip_roll: bool,
    bounded: bool
) -> Result<String, String> {
    let mut camera_data: CameraData = serde_json::from_str(&data)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    
    // Calculate bounds if bounded mirror
    let (min_pos, max_pos) = if bounded {
        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut min_z = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;
        let mut max_z = f64::MIN;
        
        for (_, kf) in camera_data.iter() {
            min_x = min_x.min(kf.position.x);
            min_y = min_y.min(kf.position.y);
            min_z = min_z.min(kf.position.z);
            max_x = max_x.max(kf.position.x);
            max_y = max_y.max(kf.position.y);
            max_z = max_z.max(kf.position.z);
        }
        
        (Position { x: min_x, y: min_y, z: min_z },
         Position { x: max_x, y: max_y, z: max_z })
    } else {
        (Position { x: 0.0, y: 0.0, z: 0.0 },
         Position { x: 0.0, y: 0.0, z: 0.0 })
    };
    
    for (_, keyframe) in camera_data.iter_mut() {
        match axis.as_str() {
            "x" => {
                if bounded {
                    let center = (min_pos.x + max_pos.x) / 2.0;
                    keyframe.position.x = 2.0 * center - keyframe.position.x;
                } else {
                    keyframe.position.x = -keyframe.position.x;
                }
            }
            "y" => {
                if bounded {
                    let center = (min_pos.y + max_pos.y) / 2.0;
                    keyframe.position.y = 2.0 * center - keyframe.position.y;
                } else {
                    keyframe.position.y = -keyframe.position.y;
                }
            }
            "z" => {
                if bounded {
                    let center = (min_pos.z + max_pos.z) / 2.0;
                    keyframe.position.z = 2.0 * center - keyframe.position.z;
                } else {
                    keyframe.position.z = -keyframe.position.z;
                }
            }
            _ => return Err(format!("Invalid axis: {}", axis)),
        }
        
        // Flip rotations if requested
        if flip_pitch {
            keyframe.rotation.pitch = -keyframe.rotation.pitch;
        }
        if flip_yaw {
            keyframe.rotation.yaw = -keyframe.rotation.yaw;
        }
        if flip_roll {
            keyframe.rotation.roll = -keyframe.rotation.roll;
        }
    }
    
    serde_json::to_string_pretty(&camera_data)
        .map_err(|e| format!("Failed to serialize: {}", e))
}

// ========================================
// TIME TRANSFORMATIONS
// ========================================

#[tauri::command]
fn transform_speed(data: String, multiplier: f64) -> Result<String, String> {
    let mut camera_data: CameraData = serde_json::from_str(&data)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    
    for (_, keyframe) in camera_data.iter_mut() {
        keyframe.timestamp /= multiplier;
        keyframe.frame = (keyframe.timestamp * 30.0).round() as i32; // Maintain 30 FPS sync
    }
    
    serde_json::to_string_pretty(&camera_data)
        .map_err(|e| format!("Failed to serialize: {}", e))
}

#[tauri::command]
fn transform_time_offset(data: String, offset_seconds: f64) -> Result<String, String> {
    let mut camera_data: CameraData = serde_json::from_str(&data)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    
    for (_, keyframe) in camera_data.iter_mut() {
        keyframe.timestamp += offset_seconds;
        keyframe.frame = (keyframe.timestamp * 30.0).round() as i32; // Maintain 30 FPS sync
    }
    
    serde_json::to_string_pretty(&camera_data)
        .map_err(|e| format!("Failed to serialize: {}", e))
}

// ========================================
// PATH OPERATIONS
// ========================================

#[tauri::command]
fn reverse_path(data: String) -> Result<String, String> {
    let camera_data: CameraData = serde_json::from_str(&data)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    
    let mut timestamps: Vec<f64> = camera_data.values().map(|kf| kf.timestamp).collect();
    timestamps.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let max_time = timestamps.last().copied().unwrap_or(0.0);
    let min_time = timestamps.first().copied().unwrap_or(0.0);
    
    let mut reversed: CameraData = HashMap::new();
    
    for (key, mut keyframe) in camera_data {
        keyframe.timestamp = max_time - keyframe.timestamp + min_time;
        keyframe.frame = (keyframe.timestamp * 30.0).round() as i32;
        reversed.insert(key, keyframe);
    }
    
    serde_json::to_string_pretty(&reversed)
        .map_err(|e| format!("Failed to serialize: {}", e))
}

#[tauri::command]
fn smooth_path(data: String, window_size: usize) -> Result<String, String> {
    let camera_data: CameraData = serde_json::from_str(&data)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    
    // Sort keyframes by timestamp
    let mut keyframes: Vec<(String, CameraKeyframe)> = camera_data.into_iter().collect();
    keyframes.sort_by(|a, b| a.1.timestamp.partial_cmp(&b.1.timestamp).unwrap());
    
    let mut smoothed: CameraData = HashMap::new();
    
    for (i, (key, keyframe)) in keyframes.iter().enumerate() {
        let start = i.saturating_sub(window_size / 2);
        let end = (i + window_size / 2 + 1).min(keyframes.len());
        
        let mut avg_pos = Position { x: 0.0, y: 0.0, z: 0.0 };
        let mut avg_fov = 0.0;
        let count = (end - start) as f64;
        
        for j in start..end {
            avg_pos.x += keyframes[j].1.position.x;
            avg_pos.y += keyframes[j].1.position.y;
            avg_pos.z += keyframes[j].1.position.z;
            avg_fov += keyframes[j].1.fov;
        }
        
        let mut smoothed_kf = keyframe.clone();
        smoothed_kf.position = Position {
            x: avg_pos.x / count,
            y: avg_pos.y / count,
            z: avg_pos.z / count,
        };
        smoothed_kf.fov = avg_fov / count;
        
        smoothed.insert(key.clone(), smoothed_kf);
    }
    
    serde_json::to_string_pretty(&smoothed)
        .map_err(|e| format!("Failed to serialize: {}", e))
}

// ========================================
// UTILITY FUNCTIONS
// ========================================

#[tauri::command]
fn get_path_stats(data: String) -> Result<String, String> {
    let camera_data: CameraData = serde_json::from_str(&data)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    
    let mut timestamps: Vec<f64> = camera_data.values().map(|kf| kf.timestamp).collect();
    timestamps.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let duration = timestamps.last().copied().unwrap_or(0.0) - timestamps.first().copied().unwrap_or(0.0);
    let keyframe_count = camera_data.len();
    
    let stats = serde_json::json!({
        "keyframes": keyframe_count,
        "duration": duration,
        "min_time": timestamps.first().copied().unwrap_or(0.0),
        "max_time": timestamps.last().copied().unwrap_or(0.0)
    });
    
    Ok(stats.to_string())
}

// ========================================
// MAIN
// ========================================

#[tauri::command]
fn open_url(url: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(&["/C", "start", &url])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&url)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&url)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            // File operations
            load_camera_file,
            save_camera_file,
            // FOV
            transform_fov_add,
            transform_fov_multiply,
            transform_fov_set,
            // Position
            transform_position_offset,
            transform_position_scale,
            // Rotation
            transform_rotation_offset,
            // Mirror
            transform_mirror,
            // Time
            transform_speed,
            transform_time_offset,
            // Path operations
            reverse_path,
            smooth_path,
            // Utilities
            get_path_stats,
            open_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}