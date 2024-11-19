mod index_gfa_file;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version = "v0.1.0", about, long_about = None)]
struct Cli {
    /// The path to the GFA file
    file_path: String,
    /// Extract only the paths with the names given in the file.
    #[arg(short = 'n', long = "names")]
    names_file: Option<String>,
}

fn main() {
    /*
    Reconstruct the paths in the graph
    One argument must be given in the command line:
    - The path to the GFA file
    One optional argument can be given in the command line:
    - The path to a list of path names to reconstruct
    It will print to standard information about the paths
    */
    // Get the file path from command line arguments
    let args: Cli = Cli::parse();

    let path_name_vector: Vec<String> = match args.names_file {
        Some(path_names_file) => match index_gfa_file::read_pathlist_file(&path_names_file) {
            Ok(result) => result,
            Err(error) => {
                eprintln!("Failed to read path names file: {}", error);
                return;
            }
        },
        None => Vec::new(),
    };

    let () = match index_gfa_file::reconstruct_paths(&args.file_path, path_name_vector) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("Failed to read GFA file: {}", error);
            return;
        }
    };
}
