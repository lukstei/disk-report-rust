# disk-report-rust

## Usage

```
An application for displaying the relative size of the file contents of a directory tree

USAGE:
    disk-report [DIRECTORY]

ARGS:
    <DIRECTORY>    Directory to scan [default: .]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

## Download

See [Releases](https://github.com/lukstei/disk-report-rust/releases/latest)

## Example

```
$ disk-report ~/Downloads/aoc-2021

Disk report for ~/Downloads/aoc-2021

aoc-2021 - 100%, 773MB total / 0MB own
  target - 96%, 745MB total / 0MB own
    debug - 74%, 571MB total / 1MB own
      deps - 39%, 304MB total / 304MB own
      incremental - 31%, 243MB total / 0MB own
        aoc-35bmbcinuvbe5 - 4%, 29MB total / 0MB own
          s-g5h8mcn8g5-1mpi2ej-213vasfh3cmhk - 4%, 29MB total / 29MB own
        aoc-3ey1hj4wnf7ld - 4%, 28MB total / 0MB own
          s-g5a33grnbj-dwjt5o-working - 1%, 10MB total / 10MB own
          s-g5a33h25b1-yg0pt0-working - 1%, 10MB total / 10MB own
        aoc-161d5rg6tkrhg - 3%, 27MB total / 0MB own
          s-g5edvrs5pz-yi26ek-working - 1%, 10MB total / 10MB own
          s-g5edvrgfvf-1cvjqql-working - 1%, 10MB total / 10MB own
        aoc-2x3h53rbdfssf - 3%, 25MB total / 0MB own
          s-g5edv4dzep-8wjsg6-h1fyt1me42s3 - 3%, 25MB total / 25MB own
        aoc-zenr7og8z6n0 - 3%, 23MB total / 0MB own
          s-g5b2f4717s-1xjcnxw-ilm0px8dup46 - 3%, 23MB total / 23MB own
        aoc-33ard6t1elvey - 3%, 23MB total / 0MB own
          s-g5ayggrmpb-1cmcao-13wwsew7zpbxk - 3%, 23MB total / 23MB own
        aoc-2ca4bp507sxj7 - 3%, 20MB total / 0MB own
          s-g5edvrs5py-rpnqg7-working - 1%, 12MB total / 12MB own
          s-g5edvs97ip-6bran-2yf6uukv7zwqh - 1%, 9MB total / 9MB own
        aoc-3qchjejs02f0t - 2%, 17MB total / 0MB own
          s-g554uees7j-1ur19t1-jlm4z1sqyqcb - 2%, 17MB total / 17MB own
        aoc-1tfggojbnav8a - 1%, 8MB total / 0MB own
        aoc-fn8giad6kkqs - 1%, 8MB total / 0MB own
          s-g5az48zzgo-b6vqd7-1vxcf10xb0umi - 1%, 8MB total / 8MB own
      build - 3%, 21MB total / 0MB own
    release - 22%, 172MB total / 1MB own
      deps - 20%, 158MB total / 158MB own
      build - 2%, 14MB total / 0MB own
  js - 3%, 27MB total / 0MB own```
    node_modules - 3%, 27MB total / 0MB own
```