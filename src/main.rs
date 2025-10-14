use clap::Parser;
use std::fs;
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

/// Command-line arguments for the WAV files validator.
#[derive(Parser, Debug)]
#[command(name = "wav-files-validate")]
#[command(
    about = "Validate the integrity of WAV audio files in a directory (recursively) and copy invalid files to an output directory."
)]
struct Args {
    /// Input directory containing WAV files to validate (processed recursively).
    input_dir: PathBuf,

    /// Output directory where invalid WAV files will be copied.
    output_invalid_dir: PathBuf,

    /// If set, simulate validation and copying without actually copying files.
    #[arg(long, default_value_t = false)]
    dry_run: bool,
}

/// Validates the integrity of a single WAV file by reading its header and all samples.
fn validate_wav(path: &Path) -> Result<bool, Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let mut reader = hound::WavReader::new(BufReader::new(file))?;

    // Read the spec to check header integrity.
    let _spec = reader.spec();

    // Attempt to read all samples to verify data integrity.
    // This will fail if the file is truncated or corrupted.
    for sample_result in reader.samples::<i16>() {
        let _ = sample_result?; // Ignore sample value; just check for errors.
    }

    Ok(true)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Create output directory if it doesn't exist (harmless for dry-run).
    fs::create_dir_all(&args.output_invalid_dir)?;

    let mut valid_count = 0u32;
    let mut invalid_count = 0u32;

    for entry in WalkDir::new(&args.input_dir)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let entry_path = entry.path();
        if entry.file_type().is_file()
            && entry_path.extension().and_then(|ext| ext.to_str()) == Some("wav")
        {
            // Compute relative path to preserve directory structure.
            let rel_path = entry_path
                .strip_prefix(&args.input_dir)
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid path"))?
                .to_path_buf();

            match validate_wav(entry_path) {
                Ok(_) => {
                    valid_count += 1;
                    println!("Valid (not copied): {:?}", entry_path);
                }
                Err(e) => {
                    invalid_count += 1;
                    eprintln!("Validation error for {:?}: {}", entry_path, e);
                    if !args.dry_run {
                        let target_path = args.output_invalid_dir.join(&rel_path);
                        if let Some(parent) = target_path.parent() {
                            fs::create_dir_all(parent)?;
                        }

                        fs::copy(entry_path, &target_path)?;
                        println!("Copied invalid: {:?}", target_path);
                    } else {
                        let target_path = args.output_invalid_dir.join(&rel_path);
                        println!("Would copy invalid: {:?}", target_path);
                    }
                }
            }
        }
    }

    println!(
        "Validation complete.\nValid files: {}\nInvalid files: {}\nTotal files processed: {}.",
        valid_count,
        invalid_count,
        valid_count + invalid_count
    );

    Ok(())
}
