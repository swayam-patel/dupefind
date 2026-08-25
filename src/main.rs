use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: dupefind <folder>");
        return;
    }

    let folder = &args[1];
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    walk_dir(Path::new(folder), &mut map);

    let mut duplicate_groups = 0;

    for (hash, files) in &map {
        if files.len() > 1 {
            duplicate_groups += 1;

            let size = fs::metadata(Path::new(&files[0]))
                .map(|m| m.len())
                .unwrap_or(0);

            println!("-----------------------------------");
            println!("Duplicates group {} ({} bytes, hash: {}):", duplicate_groups, size, &hash[..8]);

            for f in files {
                println!(" -> {}", f);
            }
        }
    }

    println!("-----------------------------------");
    if duplicate_groups == 0 {
        println!("No duplicates found.");
    } else {
        println!("Found {} duplicate groups.", duplicate_groups);
    }
}


fn walk_dir(dir: &Path, map: &mut HashMap<String, Vec<String>>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk_dir(&path, map);
            } else {
                if let Some(hash) = hash_file(&path) {
                    map.entry(hash)
                        .or_insert_with(Vec::new)
                        .push(path.display().to_string());
                }
            }
        }
    }
}

fn hash_file(path: &Path) -> Option<String> {
    let content = fs::read(path).ok()?;
    Some(format!("{:x}",md5::compute(content)))
}