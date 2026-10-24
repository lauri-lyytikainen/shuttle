use shuttle_core::Node;
use tauri::{Manager, State};

#[tauri::command]
fn node_id(node: State<'_, Node>) -> String {
    node.id()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let node = tauri::async_runtime::block_on(Node::start())?;
            app.manage(node);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![node_id])
        .run(tauri::generate_context!())
        .expect("error while running shuttle");
}
