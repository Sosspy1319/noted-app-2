use std::{fs, process::Command};

#[tauri::command]
async fn run_code(lang: String, code: String) -> Result<String, String> {
    let d = std::env::temp_dir().join("noted_run");
    fs::create_dir_all(&d).map_err(|e| e.to_string())?;
    let exe = d.join(if cfg!(windows) { "out.exe" } else { "out" }).to_string_lossy().to_string();
    let s = |v: &[&str]| v.iter().map(|x| x.to_string()).collect::<Vec<_>>();
    let (file, steps): (&str, Vec<Vec<String>>) = match lang.as_str() {
        "py" => ("main.py", vec![s(&["python3", "main.py"])]),
        "js" => ("main.js", vec![s(&["node", "main.js"])]),
        "cpp" => ("main.cpp", vec![s(&["g++", "-std=c++17", "main.cpp", "-o", "out"]), vec![exe]]),
        "java" => ("Main.java", vec![s(&["java", "Main.java"])]),
        "rust" => ("main.rs", vec![s(&["rustc", "main.rs", "-o", "out"]), vec![exe]]),
        "go" => ("main.go", vec![s(&["go", "run", "main.go"])]),
        _ => return Err("unsupported language".into()),
    };
    fs::write(d.join(file), code).map_err(|e| e.to_string())?;
    let mut out = String::new();
    for st in steps {
        let r = Command::new(&st[0]).args(&st[1..]).current_dir(&d).output()
            .map_err(|e| format!("couldn't start {} ({}). Is it installed?", st[0], e))?;
        out.push_str(&String::from_utf8_lossy(&r.stdout));
        out.push_str(&String::from_utf8_lossy(&r.stderr));
        if !r.status.success() { out.push_str(&format!("\n[exited with {}]", r.status)); break; }
    }
    Ok(out)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_code])
        .run(tauri::generate_context!())
        .expect("error while running NOTED");
}
