$env:SOURCE_DATE_EPOCH = "1767078440"

$rustflags = @(
    "--remap-path-prefix {0}=." -f (Get-Location).Path,
    "--remap-path-prefix {0}=.cargo" -f $env:CARGO_HOME,
    "--remap-path-prefix {0}=.home" -f $env:USERPROFILE,
    "--remap-path-prefix target=target",
    "-C debuginfo=0",
    "-C link-arg=/Brepro"
)
$env:CARGO_ENCODED_RUSTFLAGS = $rustflags -join ([char]0x1F)

cargo build --release
