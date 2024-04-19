use std::fs;

fn read_contents(file: &String) -> String
{
    println!("reading file {file}"); //make this debug maybe

    let contents = fs::read_to_string(file) //reads file
        .expect("Error"); //if eror then suicide
    // match env::current_dir() {
    //     Ok(path) => println!("Current directory: {:?}", path),
    //     Err(e) => eprintln!("Failed to get current directory: {}", e),
    // }
    // println!("file contents : \n{contents}") //print
    contents
}



pub fn read_journals(dir_path: &str) -> Vec<String>
{
    let mut result: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(dir_path) { //i have no clue why this works
        // Iterate over the directory entries
        for entry in entries {
            if let Ok(entry) = entry
            {
                // Get the file name as a string
                if let Some(file_name) = entry.file_name().to_str()
                {
                    // println!("{}", file_name);
                    result.push(String::from(file_name));
                }

                else
                {
                    eprintln!("Error converting file name to string");
                }
            }

            else
            {
                eprintln!("Error reading directory entry");
            }
        }
    }

    else
    {
        eprintln!("Error reading directory");
    }
    result
}