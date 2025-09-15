use std::fs;
use std::path::PathBuf;

/// 尝试加载外部 task_types.json，如果不存在则回退到内置
fn load_task_types() -> String {
    // 获取 exe 所在目录
    let exe_dir: PathBuf = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();

    // 构造外部 JSON 路径
    let external_path = exe_dir.join("task_types.json");

    // 优先读取外部 JSON
    if external_path.exists() {
        match fs::read_to_string(&external_path) {
            Ok(content) => {
                println!("✅ 使用外部 task_types.json: {:?}", external_path);
                return content;
            }
            Err(e) => {
                eprintln!("⚠️ 外部 task_types.json 读取失败: {}, 改用内置", e);
            }
        }
    }

    // 回退到内置 JSON
    // 注意: 这里路径是从 src-tauri/src/lib.rs 出发 → 指向项目根目录 public
    println!("📦 使用内置 task_types.json");
    include_str!("../../public/task_types.json").to_string()
}

/// 前端可调用的命令
#[tauri::command]
fn get_task_types() -> String {
    load_task_types()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // 注册命令，允许前端 invoke
        .invoke_handler(tauri::generate_handler![get_task_types])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
