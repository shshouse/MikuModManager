use std::fs;
use std::path::{Path, PathBuf};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_app_dir() -> Result<String, String> {
    // Get the current working directory (where the exe is located)
    match std::env::current_dir() {
        Ok(path) => Ok(path.to_string_lossy().to_string()),
        Err(e) => Err(format!("Failed to get current directory: {}", e))
    }
}

#[tauri::command]
fn scan_directory(path: String) -> Result<Vec<String>, String> {
    let dir_path = Path::new(&path);
    
    if !dir_path.exists() {
        return Ok(vec![]);
    }
    
    match fs::read_dir(dir_path) {
        Ok(entries) => {
            let mut folders = Vec::new();
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(metadata) = entry.metadata() {
                        if metadata.is_dir() {
                            if let Some(name) = entry.file_name().to_str() {
                                folders.push(name.to_string());
                            }
                        }
                    }
                }
            }
            Ok(folders)
        }
        Err(e) => Err(format!("Failed to read directory: {}", e))
    }
}

#[tauri::command]
fn get_all_files(path: String) -> Result<Vec<String>, String> {
    let mut files = Vec::new();
    collect_files(Path::new(&path), &mut files)?;
    Ok(files)
}

fn collect_files(dir: &Path, files: &mut Vec<String>) -> Result<(), String> {
    if !dir.exists() {
        return Ok(());
    }
    
    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        collect_files(&path, files)?;
                    } else {
                        files.push(path.to_string_lossy().to_string());
                    }
                }
            }
            Ok(())
        }
        Err(e) => Err(format!("Failed to read directory: {}", e))
    }
}

#[tauri::command]
fn create_directory(path: String) -> Result<(), String> {
    match fs::create_dir_all(&path) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to create directory: {}", e))
    }
}

#[tauri::command]
fn file_exists(path: String) -> bool {
    Path::new(&path).exists()
}

#[tauri::command]
fn copy_file(from: String, to: String) -> Result<(), String> {
    // Create parent directory if it doesn't exist
    if let Some(parent) = Path::new(&to).parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            return Err(format!("Failed to create parent directory: {}", e));
        }
    }
    
    match fs::copy(&from, &to) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to copy file from {} to {}: {}", from, to, e))
    }
}

#[tauri::command]
fn delete_file(path: String) -> Result<(), String> {
    match fs::remove_file(&path) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to delete file: {}", e))
    }
}

#[tauri::command]
fn write_file(path: String, content: String) -> Result<(), String> {
    // Create parent directory if it doesn't exist
    if let Some(parent) = Path::new(&path).parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            return Err(format!("Failed to create parent directory: {}", e));
        }
    }
    
    match fs::write(&path, content) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to write file: {}", e))
    }
}

#[tauri::command]
fn read_file(path: String) -> Result<String, String> {
    match fs::read_to_string(&path) {
        Ok(content) => Ok(content),
        Err(e) => Err(format!("Failed to read file: {}", e))
    }
}

#[tauri::command]
fn scan_gta4_path() -> Result<Vec<String>, String> {
    let mut gta4_paths = Vec::new();
    
    // 常见的GTAIV安装路径（已包含GTAIV子目录）
    let common_paths = vec![
        "C:\\Program Files\\Rockstar Games\\Grand Theft Auto IV\\GTAIV",
        "C:\\Program Files (x86)\\Rockstar Games\\Grand Theft Auto IV\\GTAIV",
        "C:\\Program Files\\Steam\\steamapps\\common\\Grand Theft Auto IV\\GTAIV",
        "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Grand Theft Auto IV\\GTAIV",
        "D:\\Steam\\steamapps\\common\\Grand Theft Auto IV\\GTAIV",
        "E:\\Steam\\steamapps\\common\\Grand Theft Auto IV\\GTAIV",
        "F:\\Steam\\steamapps\\common\\Grand Theft Auto IV\\GTAIV",
        "D:\\Program Files\\Steam\\steamapps\\common\\Grand Theft Auto IV\\GTAIV",
        "E:\\Program Files\\Steam\\steamapps\\common\\Grand Theft Auto IV\\GTAIV",
        "F:\\Program Files\\Steam\\steamapps\\common\\Grand Theft Auto IV\\GTAIV",
    ];
    
    // 检查常见路径
    for path in common_paths {
        if Path::new(path).exists() {
            gta4_paths.push(path.to_string());
        }
    }
    
    // 如果没有找到，尝试扫描注册表（Windows）
    if gta4_paths.is_empty() {
        if let Ok(registry_path) = scan_registry_for_gta4() {
            let registry_gta4_path = PathBuf::from(&registry_path);
            // 检查GTAIV子目录
            let registry_gtaiv_subdir = registry_gta4_path.join("GTAIV");
            if registry_gtaiv_subdir.exists() {
                gta4_paths.push(registry_gtaiv_subdir.to_string_lossy().to_string());
            } else {
                gta4_paths.push(registry_path);
            }
        }
    }
    
    Ok(gta4_paths)
}

#[cfg(target_os = "windows")]
fn scan_registry_for_gta4() -> Result<String, String> {
    use winreg::enums::*;
    use winreg::RegKey;
    
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    
    // 尝试Steam注册表路径
    if let Ok(steam_key) = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Steam App 12210") {
        if let Ok(install_location) = steam_key.get_value::<String, _>("InstallLocation") {
            return Ok(install_location);
        }
    }
    
    // 尝试Rockstar Games注册表路径
    if let Ok(rockstar_key) = hklm.open_subkey("SOFTWARE\\Rockstar Games\\Grand Theft Auto IV") {
        if let Ok(install_dir) = rockstar_key.get_value::<String, _>("InstallFolder") {
            return Ok(install_dir);
        }
    }
    
    Err("未在注册表中找到GTA4安装路径".to_string())
}

#[cfg(not(target_os = "windows"))]
fn scan_registry_for_gta4() -> Result<String, String> {
    Err("非Windows系统不支持注册表扫描".to_string())
}

#[tauri::command]
fn get_gta4_mod_info(game_path: String) -> Result<serde_json::Value, String> {
    let gta4_path = PathBuf::from(&game_path);
    
    if !gta4_path.exists() {
        return Err("GTA4路径不存在".to_string());
    }
    
    let mut mod_info = serde_json::json!({
        "game_name": "GTAIV",
        "game_path": game_path,
        "supported_mods": [
            "车辆模型替换",
            "武器模型替换", 
            "角色皮肤替换",
            "地图纹理替换",
            "脚本模组",
            "ENB图形增强"
        ],
        "mod_folders": [],
        "backup_recommended": true
    });
    
    // 检查常见的模组文件夹
    let mod_folders = vec![
        "models",
        "textures", 
        "scripts",
        "plugins",
        "ENBSeries"
    ];
    
    let mut existing_folders = Vec::new();
    for folder in mod_folders {
        let folder_path = gta4_path.join(folder);
        if folder_path.exists() {
            existing_folders.push(folder);
        }
    }
    
    mod_info["mod_folders"] = serde_json::json!(existing_folders);
    
    Ok(mod_info)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_app_dir,
            scan_directory,
            get_all_files,
            create_directory,
            file_exists,
            copy_file,
            delete_file,
            write_file,
            read_file,
            scan_gta4_path,
            get_gta4_mod_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
