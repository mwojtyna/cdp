# cdp

Quickly **cd** into your **p**rojects.

## How it works

Searches recursively every folder from the specified `ROOT_DIR` until finding a `STOPPER` (`.git` folder by default).
When the `STOPPER` has been found, adds its parent directory to a filterable list and continues searching.

![example](https://preview.redd.it/bew321bsf97c1.png?width=1030&format=png&auto=webp&s=7309f46cd918d8bcd7ca27505bf92c0b30c38c76)

## Usage

1. Download the binary.
2. Add the following to your `.bashrc` (or the equivalent file in other shells).

```bash
cdp() {
	cd "$(/path/to/cdp /path/to/ROOT_DIR "$@")"
}
```

3. Restart your terminal or source `.bashrc`.
4. Run `cdp` or run `cdp folder`, where `folder` is a string that will match one of the paths in the list.
   This will immediately cd into that folder without bringing up the list UI.

> [!TIP]
> You can use `cd -` to go back to the previous directory after running `cdp`.

## Key bindings

-   Go up: `Up arrow`/`Ctrl+K`
-   Go down: `Down arrow`/`Ctrl+J`
-   Go to first entry: `Home`
-   Go to last entry: `End`
-   Exit: `Escape`/`Ctrl+C`
-   Accept: `Enter`

## Arguments

> [!IMPORTANT]
> If you want to run `--help` or `--version` commands, use the full path to the binary eg.
> `/path/to/cdp --version`<br />
> otherwise the output will be mangled. This is due to a limitation in `clap` rust library, which can't print the output of these flags to stderr.

```
Usage: cdp [OPTIONS] <ROOT_DIR> [SEARCH_QUERY]

Arguments:
  <ROOT_DIR>      Search recursively from this directory
  [SEARCH_QUERY]  Jump into the first directory matching this string (optional)

Options:
      --stopper <STOPPER>  Search for directories containing this file [default: .git]
      --greedy             Continue searching in a directory subtree when a stopper file is found
      --cpus <CPUS>        Amount of logical cores to use for searching the root_dir, defaults to half available
      --case-sensitive
  -h, --help               Print help
  -V, --version            Print version
```
