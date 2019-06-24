# Promptly

## Getting started

Clone the repo to get started. I have copied a lot of ideas and code from [pista](https://github.com/NerdyPepper/pista) so that might be a
better place to look.

## Installing

Inside the directory, run `cargo install --path .` to install it in your cargo binaries directory. Alternatively, just add the project
folder to your `$PATH`. There probably is a better way to do local installs, so feel free and correct me on this.

## Usage

Inside your `.bashrc` ( or shell of choice ):

```bash
export PS1="\`promptly\` "
```

### Configuration

The following `ENV` vars are configurable:

- `CWD_COLOR`
- `BRANCH_COLOR`
- `COMMIT_COLOR`
- `GIT_CLEAN_COLOR`
- `GIT_WT_MODIFIED_COLOR`
- `GIT_INDEX_MODIFIED_COLOR`
- `GIT_WT_MODIFIED` -> Icon for when the working tree is modified
- `GIT_INDEX_MODIFIED` -> Icon for when the index is modified
