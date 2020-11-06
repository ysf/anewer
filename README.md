# anewer [![@ysfr][twitter-img]][twitter]
anewer appends lines from stdin to a file if they don't already exist in the file. This is a rust version of
[tomnomnom/anew](https://github.com/tomnomnom/anew).

[twitter-img]:  https://img.shields.io/badge/twitter-@ysfr-blue.svg
[twitter]:      https://twitter.com/ysfr


## Usage

```
$ anewer
USAGE:
    anewer [FLAGS] [filename]

FLAGS:
    -n, --dry-run    Dry run, will leave the file as it is
    -h, --help       Prints help information
    -q, --quiet      Quiet, won't print to stdout
    -V, --version    Prints version information

ARGS:
    <filename>
```


```
$ cat things.txt
Zero
One
Two

$ cat newthings.txt
One
Two
Three
Four

$ cat newthings.txt | anewer things.txt
Three
Four

$ cat things.txt
Zero
One
Two
Three
Four

$ cat unique_this_list.txt
One
One
Two
Two
Three
Four
Three
Four

$ cat unique_this_list.txt | anewer
One
Two
Three
Four
```


## Installation

```
cargo install anewer
```

# License
GPLv3+
