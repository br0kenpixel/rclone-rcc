use std::{fs, io, path::PathBuf};

pub fn size(target: PathBuf) -> i32 {
    if !target.is_dir() {
        eprintln!("Invalid directory");
        return 1;
    }

    let Ok(result) = walk_dir(target) else {
        eprintln!("An I/O error occured while perfoming the calculation");
        return 1;
    };
    let nobjects = result.len() as u64;
    let total_size: u64 = result.iter().sum();

    println!(
        "Total objects: {} ({nobjects})",
        pretty_format(nobjects, None)
    );
    println!(
        "Total size: {} ({total_size} bytes)",
        pretty_format(total_size, Some('B'))
    );

    0
}

fn walk_dir(dir: PathBuf) -> io::Result<Vec<u64>> {
    let mut results = Vec::new();
    let iter = fs::read_dir(&dir)?.map(Result::unwrap);

    for result in iter {
        let ftype = result.file_type()?;
        if ftype.is_dir() {
            results.extend(walk_dir(result.path())?);
            continue;
        }

        if !ftype.is_file() {
            continue;
        }

        results.push(result.metadata()?.len());
    }

    Ok(results)
}

fn pretty_format(count: u64, unit: Option<char>) -> String {
    let exp = |n| (10.0f32).powi(n);

    let mut result = match count {
        0..=999 => format!("{count}"),
        1000..=999_999 => format!("{:.3}k", count as f32 / 1000.0),
        1_000_000..=999_999_999 => format!("{:.3}M", count as f32 / exp(6)),
        1_000_000_000..=999_999_999_999 => format!("{:.3}G", count as f32 / exp(9)),
        _ => format!("{:.3}T", count as f32 / exp(12)),
    };

    if let Some(suffix) = unit {
        result.push(suffix);
    }

    result
}
