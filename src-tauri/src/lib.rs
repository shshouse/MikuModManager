use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::io::Read;
use serde::{Deserialize, Serialize};
use zip::ZipArchive;

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
fn delete_directory(path: String) -> Result<(), String> {
    match fs::remove_dir_all(&path) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to delete directory: {}", e))
    }
}

#[tauri::command]
fn copy_directory(from: String, to: String) -> Result<(), String> {
    let source = Path::new(&from);
    let destination = Path::new(&to);
    
    if !source.exists() {
        return Err(format!("源目录不存在: {}", from));
    }
    
    if !source.is_dir() {
        return Err(format!("源路径不是目录: {}", from));
    }
    
    copy_dir_recursive(source, destination)?;
    
    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), String> {
    if !dst.exists() {
        fs::create_dir_all(dst)
            .map_err(|e| format!("创建目标目录失败: {}", e))?;
    }
    
    for entry in fs::read_dir(src)
        .map_err(|e| format!("读取源目录失败: {}", e))? 
    {
        let entry = entry.map_err(|e| format!("读取目录条目失败: {}", e))?;
        let path = entry.path();
        let file_name = entry.file_name();
        let dst_path = dst.join(&file_name);
        
        if path.is_dir() {
            copy_dir_recursive(&path, &dst_path)?;
        } else {
            fs::copy(&path, &dst_path)
                .map_err(|e| format!("复制文件失败: {}", e))?;
        }
    }
    
    Ok(())
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
fn open_directory(path: String) -> Result<(), String> {
    let dir_path = Path::new(&path);
    
    // 验证路径存在
    if !dir_path.exists() {
        return Err(format!("目录不存在: {}", path));
    }
    
    // 验证是一个目录
    if !dir_path.is_dir() {
        return Err(format!("路径不是一个目录: {}", path));
    }
    
    // 获取绝对路径
    let absolute_path = match dir_path.canonicalize() {
        Ok(p) => p,
        Err(e) => return Err(format!("无法获取绝对路径: {}", e))
    };
    
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        
        // 在Windows上，使用绝对路径并确保路径格式正确
        let path_str = absolute_path.to_string_lossy().to_string();
        
        match Command::new("explorer")
            .arg(&path_str)
            .spawn() {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Failed to open directory: {}", e))
            }
    }
    
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        
        match Command::new("open")
            .arg(absolute_path.to_str().unwrap_or(&path))
            .spawn() {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Failed to open directory: {}", e))
            }
    }
    
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        
        match Command::new("xdg-open")
            .arg(absolute_path.to_str().unwrap_or(&path))
            .spawn() {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Failed to open directory: {}", e))
            }
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Err("Unsupported operating system".to_string())
    }
}

#[tauri::command]
fn read_file(path: String) -> Result<String, String> {
    match fs::read_to_string(&path) {
        Ok(content) => Ok(content),
        Err(e) => Err(format!("Failed to read file: {}", e))
    }
}

// GameStatus 结构体定义
#[derive(Debug, Serialize, Deserialize)]
struct GameStatus {
    game_name: String,
    game_path: String,
    launch_options: String,
    installed_mods: Vec<String>,
    play_time: u64, // 游玩时长，单位：秒
    last_updated: String,
}

#[tauri::command]
fn create_game_status(
    game_name: String,
    game_path: String,  // 游戏管理目录（存放game_status.json的位置，如：app_dir/game/游戏名）
    launch_options: String,
    game_install_path: Option<String>,  // 游戏实际安装路径（可选）
) -> Result<String, String> {
    let status = GameStatus {
        game_name: game_name.clone(),
        game_path: game_install_path.unwrap_or_default(), // 游戏实际安装路径
        launch_options,
        installed_mods: Vec::new(),
        play_time: 0, // 初始游玩时长为0秒
        last_updated: chrono::Local::now().to_rfc3339(),
    };
    
    let status_path = PathBuf::from(&game_path).join("game_status.json");
    
    match serde_json::to_string_pretty(&status) {
        Ok(content) => {
            match fs::write(&status_path, content) {
                Ok(_) => Ok(status_path.to_string_lossy().to_string()),
                Err(e) => Err(format!("Failed to write game status: {}", e)),
            }
        }
        Err(e) => Err(format!("Failed to serialize game status: {}", e)),
    }
}

#[tauri::command]
fn read_game_status(game_path: String) -> Result<String, String> {
    let status_path = PathBuf::from(&game_path).join("game_status.json");
    
    if !status_path.exists() {
        return Err("game_status.json not found".to_string());
    }
    
    match fs::read_to_string(&status_path) {
        Ok(content) => Ok(content),
        Err(e) => Err(format!("Failed to read game status: {}", e)),
    }
}

#[tauri::command]
fn update_game_status(
    game_path: String,
    launch_options: Option<String>,
    installed_mods: Option<Vec<String>>,
) -> Result<String, String> {
    let status_path = PathBuf::from(&game_path).join("game_status.json");
    
    // 读取现有状态
    let mut status: GameStatus = if status_path.exists() {
        let content = fs::read_to_string(&status_path)
            .map_err(|e| format!("Failed to read existing status: {}", e))?;
        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse existing status: {}", e))?
    } else {
        // 如果文件不存在，创建新的状态
        GameStatus {
            game_name: PathBuf::from(&game_path)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            game_path: game_path.clone(),
            launch_options: String::new(),
            installed_mods: Vec::new(),
            play_time: 0,
            last_updated: chrono::Local::now().to_rfc3339(),
        }
    };
    
    // 更新字段
    if let Some(options) = launch_options {
        status.launch_options = options;
    }
    
    if let Some(mods) = installed_mods {
        status.installed_mods = mods;
    }
    
    status.last_updated = chrono::Local::now().to_rfc3339();
    
    // 写回文件
    match serde_json::to_string_pretty(&status) {
        Ok(content) => {
            match fs::write(&status_path, content) {
                Ok(_) => Ok("Game status updated successfully".to_string()),
                Err(e) => Err(format!("Failed to write updated status: {}", e)),
            }
        }
        Err(e) => Err(format!("Failed to serialize updated status: {}", e)),
    }
}

#[tauri::command]
fn scan_games_for_status(app_dir: String) -> Result<Vec<String>, String> {
    let games_dir = PathBuf::from(&app_dir).join("game");
    
    if !games_dir.exists() {
        return Ok(Vec::new());
    }
    
    let mut games_without_status = Vec::new();
    
    match fs::read_dir(&games_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let game_path = entry.path();
                    if game_path.is_dir() {
                        let status_path = game_path.join("game_status.json");
                        if !status_path.exists() {
                            games_without_status.push(game_path.to_string_lossy().to_string());
                        }
                    }
                }
            }
            Ok(games_without_status)
        }
        Err(e) => Err(format!("Failed to scan games directory: {}", e)),
    }
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
fn calculate_file_md5(path: String) -> Result<String, String> {
    let mut file = fs::File::open(&path)
        .map_err(|e| format!("Failed to open file: {}", e))?;
    
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    let hash = md5::compute(&buffer);
    Ok(format!("{:x}", hash))
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

/// 解压模组压缩包到指定目录
#[tauri::command]
fn extract_mod_archive(archive_path: String, target_dir: String) -> Result<String, String> {
    let archive_path = Path::new(&archive_path);
    let target_dir = Path::new(&target_dir);
    
    // 检查文件是否存在
    if !archive_path.exists() {
        return Err("压缩包文件不存在".to_string());
    }
    
    // 获取压缩包文件名（不含扩展名）作为文件夹名
    let folder_name = archive_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or("无法获取文件名")?;
    
    // 创建目标目录
    fs::create_dir_all(target_dir)
        .map_err(|e| format!("创建目标目录失败: {}", e))?;
    
    // 最终解压目录
    let extract_dir = target_dir.join(folder_name);
    
    // 检查目标文件夹是否已存在
    if extract_dir.exists() {
        return Err(format!("模组文件夹 '{}' 已存在，请先删除或重命名", folder_name));
    }
    
    // 创建解压目录
    fs::create_dir_all(&extract_dir)
        .map_err(|e| format!("创建解压目录失败: {}", e))?;
    
    // 打开zip文件
    let file = fs::File::open(archive_path)
        .map_err(|e| format!("无法打开压缩包: {}", e))?;
    
    let mut archive = ZipArchive::new(file)
        .map_err(|e| format!("无法读取压缩包: {}", e))?;
    
    // 解压所有文件
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .map_err(|e| format!("读取压缩包条目失败: {}", e))?;
        
        let outpath = match file.enclosed_name() {
            Some(path) => extract_dir.join(path),
            None => continue,
        };
        
        if file.name().ends_with('/') {
            // 这是一个目录
            fs::create_dir_all(&outpath)
                .map_err(|e| format!("创建目录失败: {}", e))?;
        } else {
            // 这是一个文件
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)
                        .map_err(|e| format!("创建父目录失败: {}", e))?;
                }
            }
            
            let mut outfile = fs::File::create(&outpath)
                .map_err(|e| format!("创建文件失败: {}", e))?;
            
            std::io::copy(&mut file, &mut outfile)
                .map_err(|e| format!("写入文件失败: {}", e))?;
        }
    }
    
    Ok(extract_dir.to_string_lossy().to_string())
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
            get_file_size,
            copy_file,
            delete_file,
            delete_directory,
            copy_directory,
            write_file,
            read_file,
            open_directory,
            launch_game,
            calculate_file_md5,
            open_url_in_browser,
            create_game_status,
            read_game_status,
            update_game_status,
            scan_games_for_status,
            extract_mod_archive
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
