use std::{env, error::Error, fs::exists, path::PathBuf};


pub fn get_exe_folder() -> PathBuf {
    let mut path_def = PathBuf::new();
    let exe_path = env::current_exe();

    if let Ok(exe_path_ok) = exe_path {
        path_def = exe_path_ok;
        path_def.pop();
    };

    path_def
}

// pub fn check_current_thread(context: &str) {
//     let current_thread = std::thread::current();
//     let thread_name = current_thread.name().unwrap_or("unknown-thread");
    
//     println!("[{}] Running on thread: {}", context, thread_name);
// }

pub fn simple_file_exist(path : &PathBuf)->bool{
    match exists(path) {
        Ok(e)=>e,
        _=>false
    }
}

pub type BoxError = Box<dyn Error>;