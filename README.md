# saucers

A command line utility to interface with the saucelabs API.

## Help

```text
saucers --help
args

USAGE:
    saucers [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    apistatus    Get the current saucelabs API status.
    assetfile    Get a file asset associated with a particular job ID. (logs, screenshots, etc.) See the 'assetlist'
                 subcommand.
    assetlist    Get the asset list associated with a particular job ID
    help         Prints this message or the help of the given subcommand(s)
    job          Get data about a particular job ID.
    jobs         Get a list of jobs data.
    upload       Upload a file to your accounts sauce storage.
    uploads      Get a list of files that have been uploaded to sauce storage.
```

Written in Rust. #rustlang

## Downloads

See the bin/{OS_Name} directories. Click on the binary, then click the download button.
