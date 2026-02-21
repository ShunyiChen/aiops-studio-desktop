// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::{Command, Stdio};
use std::path::Path;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

fn main() {
    let grafana_exe = r"./grafana-v12.0.0/bin/grafana-server.exe";
    
    // Check if grafana-server.exe is already running
    // if !is_process_running("grafana-server.exe") {
    //     match start_grafana(grafana_exe) {
    //         Ok(_) => {
    //             // Wait a short moment to ensure it doesn't immediately exit
    //             std::thread::sleep(std::time::Duration::from_millis(1500));
    //             if !is_process_running("grafana-server.exe") {
    //                 rfd::MessageDialog::new()
    //                     .set_title("Grafana 启动失败")
    //                     .set_description("Grafana 服务器启动后意外关闭。\n请检查端口 3000 是否被占用，或者配置文件是否正确。")
    //                     .set_level(rfd::MessageLevel::Error)
    //                     .show();
    //             } else {
    //                 aiops_studio_desktop_lib::run()
    //             }
    //         }
    //         Err(e) => {
    //             rfd::MessageDialog::new()
    //                 .set_title("Grafana 启动失败")
    //                 .set_description(&format!("无法运行 Grafana 服务器: {}\n请确认执行文件路径是否正确: {}", e, grafana_exe))
    //                 .set_level(rfd::MessageLevel::Error)
    //                 .show();
    //         }
    //     }
    // } else {
    //     aiops_studio_desktop_lib::run()
    // }

    // 如果时debug模式，则直接aiops_studio_desktop_lib::run()
    #[cfg(debug_assertions)]
    aiops_studio_desktop_lib::run();
}

fn is_process_running(process_name: &str) -> bool {
    let mut cmd = Command::new("tasklist");
    cmd.args(["/FI", &format!("IMAGENAME eq {}", process_name), "/NH"]);
    
    #[cfg(windows)]
    cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW

    let output = cmd.output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            stdout.contains(process_name)
        }
        Err(_) => false,
    }
}

fn start_grafana(exe_path: &str) -> std::io::Result<()> {
    let path = Path::new(exe_path);
    if !path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("找不到文件: {}", exe_path),
        ));
    }
    // Usually Grafana should be started from its root directory (one level up from bin)
    // so it can find its configuration files (conf/) and static assets (public/).
    let working_dir = path.parent()
        .and_then(|bin_dir| bin_dir.parent())
        .unwrap_or_else(|| Path::new("."));

    let mut cmd = Command::new(exe_path);
    cmd.current_dir(working_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    #[cfg(windows)]
    cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW

    cmd.spawn()?;
    
    Ok(())
}

