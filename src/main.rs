use anyhow::{Context, Result, bail};
use clap::Parser;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::{TempDir, tempdir};
use walkdir::WalkDir;

#[derive(Debug, Parser)]
#[command(
    name = "printall",
    about = "Find PDF and DOCX files recursively and print them"
)]
struct Cli {
    #[arg(default_value = ".")]
    directory: PathBuf,
    #[arg(long)]
    dry_run: bool,
    #[arg(long)]
    keep_converted: bool,
    #[arg(short, long, env = "PRINTALL_PRINTER")]
    printer: Option<String>,
    #[arg(long, env = "PRINTALL_CONVERTER", default_value = "soffice")]
    converter: String,
}

struct Documents {
    files: Vec<PathBuf>,
    _temporary_directory: Option<TempDir>,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    let directory = cli
        .directory
        .canonicalize()
        .with_context(|| format!("cannot access directory {}", cli.directory.display()))?;
    if !directory.is_dir() {
        bail!("not a directory: {}", directory.display());
    }
    let documents = discover(&directory, &cli)?;
    if documents.files.is_empty() {
        println!("No PDF or DOCX files found in {}", directory.display());
        return Ok(());
    }
    for file in &documents.files {
        println!("{}", file.display());
    }
    if !cli.dry_run {
        print_files(&documents.files, cli.printer.as_deref())?;
    }
    Ok(())
}

fn discover(directory: &Path, cli: &Cli) -> Result<Documents> {
    let mut sources: Vec<PathBuf> = WalkDir::new(directory)
        .follow_links(false)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.into_path())
        .filter(|path| matches_extension(path, "pdf") || matches_extension(path, "docx"))
        .collect();
    sources.sort_by_key(|path| path.to_string_lossy().to_lowercase());

    let temporary_directory =
        if cli.keep_converted || !sources.iter().any(|p| matches_extension(p, "docx")) {
            None
        } else {
            Some(tempdir().context("cannot create temporary conversion directory")?)
        };
    let output_directory = temporary_directory.as_ref().map(TempDir::path);
    let mut files = Vec::with_capacity(sources.len());
    for source in sources {
        if matches_extension(&source, "pdf") {
            files.push(source);
        } else {
            files.push(convert_docx(&source, output_directory, &cli.converter)?);
        }
    }
    Ok(Documents {
        files,
        _temporary_directory: temporary_directory,
    })
}

fn convert_docx(
    source: &Path,
    output_directory: Option<&Path>,
    converter: &str,
) -> Result<PathBuf> {
    let destination =
        output_directory.unwrap_or_else(|| source.parent().unwrap_or_else(|| Path::new(".")));
    let status = Command::new(converter)
        .args(["--headless", "--convert-to", "pdf", "--outdir"])
        .arg(destination)
        .arg(source)
        .status()
        .with_context(|| format!("cannot run DOCX converter '{converter}'"))?;
    if !status.success() {
        bail!("DOCX converter failed for {}", source.display());
    }
    let output = destination
        .join(source.file_stem().context("DOCX file has no name")?)
        .with_extension("pdf");
    if !output.is_file() {
        bail!("DOCX converter did not create {}", output.display());
    }
    Ok(output)
}

fn print_files(files: &[PathBuf], printer: Option<&str>) -> Result<()> {
    let command = if command_exists("lp") {
        "lp"
    } else if command_exists("lpr") {
        "lpr"
    } else {
        bail!("neither 'lp' nor 'lpr' is installed; use --dry-run to list files")
    };
    for file in files {
        let mut print = Command::new(command);
        if let Some(printer) = printer {
            print.args(["-d", printer]);
        }
        let status = print
            .arg(file)
            .status()
            .with_context(|| format!("cannot run {command}"))?;
        if !status.success() {
            bail!("printing failed for {}", file.display());
        }
    }
    Ok(())
}

fn command_exists(command: &str) -> bool {
    std::env::var_os("PATH")
        .into_iter()
        .flat_map(|paths| std::env::split_paths(&paths).collect::<Vec<_>>())
        .map(|path| path.join(command))
        .any(|path| path.is_file())
}

fn matches_extension(path: &Path, extension: &str) -> bool {
    path.extension()
        .is_some_and(|value| value.eq_ignore_ascii_case(extension))
}

#[cfg(test)]
mod tests {
    use super::matches_extension;
    use std::path::Path;

    #[test]
    fn extensions_are_case_insensitive() {
        assert!(matches_extension(Path::new("report.PDF"), "pdf"));
        assert!(matches_extension(Path::new("report.DocX"), "docx"));
        assert!(!matches_extension(Path::new("report.doc"), "docx"));
    }
}
