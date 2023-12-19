# cdp

Quickly **cd** into your **p**rojects.

## How it works

Searches recursively every folder from the specified `ROOT_DIR` until finding a `STOPPER` (`.git` folder by default).
When the `STOPPER` has been found, adds its parent directory to a filterable list and continues searching.

For example, when `ROOT_DIR` is `~/.local/share` and all folders in `~/.local/share/nvim/lazy` contain `.git` folders:

![example](https://i.imgur.com/M57wLmL.png)

## Usage

1. Download the binary.
2. Add the following to your `.bashrc` (or the equivalent file in other shells).

```bash
cdp() {
	cd "$(/path/to/cdp /path/to/ROOT_DIR)"
}
```

3. Restart your terminal or source `.bashrc`.
4. Run `cdp`.

> [!TIP]
> You can use `cd -` to go back to the previous directory after running `cdp`.

## Arguments

> [!IMPORTANT]
> These arguments only work when specified in the `$()` block in `.bashrc` as shown above.

```
Usage: cdp [OPTIONS] <ROOT_DIR>

Arguments:
  <ROOT_DIR>

Options:
      --stopper <STOPPER>  Search for directories containing this file [default: .git]
      --greedy             Continue searching in a directory subtree when a stopper file is found
      --cpus <CPUS>        Amount of logical cores to use for searching the root_dir, defaults to half available
  -h, --help               Print help
  -V, --version            Print version
```
