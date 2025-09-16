use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

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
fn get_file_size(path: String) -> Result<u64, String> {
    let metadata = std::fs::metadata(&path).map_err(|e| e.to_string())?;
    Ok(metadata.len())
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
    
    // 根据系统架构选择注册表路径
    // 检查是否为64位系统
    let is_64_bit = std::env::consts::ARCH == "x86_64" || std::env::consts::ARCH == "aarch64";
    
    let rockstar_reg_path = if is_64_bit {
        // 64位系统使用WOW6432Node路径
        "SOFTWARE\\WOW6432Node\\Rockstar Games\\Grand Theft Auto IV"
    } else {
        // 32位系统直接使用Rockstar Games路径
        "SOFTWARE\\Rockstar Games\\Grand Theft Auto IV"
    };
    
    // 尝试从Rockstar Games注册表路径读取
    if let Ok(rockstar_key) = hklm.open_subkey(rockstar_reg_path) {
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
        "mod_folders": [],
    });
    
    // 检查常见的模组文件夹
    let existing_folders: Vec<String> = Vec::new();
    mod_info["mod_folders"] = serde_json::json!(existing_folders);
    
    Ok(mod_info)
}

#[tauri::command]
fn scan_jc3_path() -> Result<Vec<String>, String> {
    let mut jc3_paths = Vec::new();
    
    // 如果没有找到，尝试扫描注册表（Windows）
    if jc3_paths.is_empty() {
        if let Ok(registry_path) = scan_registry_for_jc3() {
            jc3_paths.push(registry_path);
        }
    }
    
    Ok(jc3_paths)
}

#[cfg(target_os = "windows")]
fn scan_registry_for_jc3() -> Result<String, String> {
    use winreg::enums::*;
    use winreg::RegKey;
    
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    
    // 尝试Steam注册表路径
    if let Ok(steam_key) = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Steam App 225540") {
        if let Ok(install_location) = steam_key.get_value::<String, _>("InstallLocation") {
            return Ok(install_location);
        }
    }
    
    // 尝试Epic Games注册表路径
    if let Ok(epic_key) = hklm.open_subkey("SOFTWARE\\WOW6432Node\\Epic Games\\EpicGamesLauncher\\Apps\\JustCause3") {
        if let Ok(install_location) = epic_key.get_value::<String, _>("InstallLocation") {
            return Ok(install_location);
        }
    }
    
    Err("未在注册表中找到正当防卫3安装路径".to_string())
}

#[cfg(not(target_os = "windows"))]
fn scan_registry_for_jc3() -> Result<String, String> {
    Err("非Windows系统不支持注册表扫描".to_string())
}

#[tauri::command]
fn get_jc3_mod_info(game_path: String) -> Result<serde_json::Value, String> {
    let jc3_path = PathBuf::from(&game_path);
    
    if !jc3_path.exists() {
        return Err("正当防卫3路径不存在".to_string());
    }
    
    let mut mod_info = serde_json::json!({
        "game_name": "Just Cause 3",
        "game_path": game_path,
        "mod_folders": [],
    });
    
    // 检查常见的模组文件夹
    let existing_folders: Vec<String> = Vec::new();
    mod_info["mod_folders"] = serde_json::json!(existing_folders);
    
    Ok(mod_info)
}

#[tauri::command]
fn launch_game(game_path: String, launch_options: String) -> Result<String, String> {
    let game_dir = PathBuf::from(&game_path);
    
    // 查找游戏可执行文件
    let exe_path = find_game_executable(&game_dir)?;
    
    let mut cmd = Command::new(&exe_path);
    
    // 解析启动选项
    if !launch_options.trim().is_empty() {
        let args: Vec<&str> = launch_options.split_whitespace().collect();
        cmd.args(&args);
    }
    
    // 设置工作目录为游戏目录
    cmd.current_dir(&game_dir);
    
    match cmd.spawn() {
        Ok(_) => Ok("游戏启动成功".to_string()),
        Err(e) => Err(format!("游戏启动失败: {}", e))
    }
}

fn find_game_executable(game_dir: &Path) -> Result<PathBuf, String> {
    // 常见的游戏可执行文件名
    let possible_exes = vec![
        "JustCause3.exe",
        "JC3.exe", 
        "GTAIV.exe",
        "LaunchGTAIV.exe",
        "game.exe"
    ];
    
    for exe_name in possible_exes {
        let exe_path = game_dir.join(exe_name);
        if exe_path.exists() {
            return Ok(exe_path);
        }
    }
    
    // 如果没找到，尝试查找任何.exe文件
    if let Ok(entries) = fs::read_dir(game_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if extension == "exe" {
                        if let Some(file_name) = path.file_name() {
                            let name = file_name.to_string_lossy().to_lowercase();
                            // 跳过一些明显不是游戏主程序的exe
                            if !name.contains("uninstall") && !name.contains("setup") && !name.contains("launcher") {
                                return Ok(path);
                            }
                        }
                    }
                }
            }
        }
    }
    
    Err("未找到游戏可执行文件".to_string())
}

#[tauri::command]
fn open_url_in_browser(url: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        // 在Windows上使用start命令打开默认浏览器
        Command::new("cmd")
            .args(["/c", "start", &url])
            .creation_flags(0x08000000) // CREATE_NO_WINDOW flag
            .spawn()
            .map_err(|e| format!("Failed to open browser: {}", e))?;
    }
    
    #[cfg(target_os = "macos")]
    {
        // 在macOS上使用open命令
        Command::new("open")
            .arg(&url)
            .spawn()
            .map_err(|e| format!("Failed to open browser: {}", e))?;
    }
    
    #[cfg(target_os = "linux")]
    {
        // 在Linux上使用xdg-open命令
        Command::new("xdg-open")
            .arg(&url)
            .spawn()
            .map_err(|e| format!("Failed to open browser: {}", e))?;
    }
    
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler!(
            greet,
            get_app_dir,
            scan_directory,
            get_all_files,
            create_directory,
            file_exists,
            get_file_size,
            copy_file,
            delete_file,
            write_file,
            read_file,
            scan_gta4_path,
            get_gta4_mod_info,
            scan_jc3_path,
            get_jc3_mod_info,
            launch_game,
            open_url_in_browser
        ))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
