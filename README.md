# PrintAll

Rust CLI that recursively finds PDF and DOCX files and prints them in name order.

## Development with Podman

The repository includes a Rust Dev Container with LibreOffice and CUPS tools:

```sh
podman build -f .devcontainer/Containerfile -t printall-dev .devcontainer
podman run --rm -v "$PWD:/workspace:Z" -w /workspace printall-dev cargo test
```

## Run

```sh
cargo run -- --dry-run /path/to/documents
cargo run -- /path/to/documents
```

DOCX files are converted to temporary PDFs through `soffice` before printing.
Use `--keep-converted` to retain PDFs beside their source files. Printing uses
`lp` or `lpr`; select a printer with `--printer NAME` or `PRINTALL_PRINTER`.
Set `--converter` or `PRINTALL_CONVERTER` if LibreOffice uses another command.

## Releases

Push a semantic-version tag to build and publish platform archives automatically:

```sh
git tag v1.0.0
git push origin v1.0.0
```

The release workflow builds Linux x64/ARM64, macOS Intel/Apple Silicon, and
Windows x64 artifacts, adds SHA-256 checksum files, and uploads them to the
tagged GitHub Release. Dependabot monitors Cargo and GitHub Actions updates,
while CodeQL scans the Rust source on pushes, pull requests, and weekly.
