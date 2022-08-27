use std::{path::Path, fs};
use std::env;
use mime_guess::mime;

fn main() {        
    let file_path = env::args().nth(1).unwrap();
    let queried_file_path = Path::new(&file_path);
    let file_name = env::args().nth(2)
        .unwrap_or(queried_file_path.file_name()
            .unwrap().to_os_string()
            .into_string().unwrap());
    let guess = mime_guess::from_path(queried_file_path);
    dbg!(guess.first().unwrap());
    let mime = guess.first_or_text_plain();    
    
    match mime.type_() {
        mime::TEXT => {
            println!("Saving text file");
            let destination_path = format!("{}/{}","files/text", file_name);
            let destination_file_path = Path::new(&destination_path);
            copy_file(queried_file_path, destination_file_path);
        },
        mime::IMAGE => {
            println!("Saving image file");
            let destination_path = format!("{}/{}","files/image", file_name);
            let destination_file_path = Path::new(&destination_path);
            copy_file(queried_file_path, destination_file_path);
        },
        _ => {},
    }
}

fn copy_file(from: &Path, to: &Path) {    
    let prefix = to.parent().unwrap();
    fs::create_dir_all(prefix).unwrap();
    fs::copy(from, to).unwrap();
}
