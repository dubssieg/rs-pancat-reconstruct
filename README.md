# Reconstruct linear sequences from a GFA graph

> [!NOTE]\
>  [pancat reconstruct](https://github.com/dubssieg/pancat) is a tool, originally written in Python, designed to extract linear sequences from the paths of a graph. For performance, it has been reimplemented in Rust.

## Install instructions:

Requires rust and cargo.

```bash
git clone 
cd rs-pancat-reconstruct
cargo build --release
```

## Usage

```bash
rs-pancat-reconstruct graph.gfa > output.fa
```

Alternatively, you can specify a file with one path name per line to specify specific paths to extract.

```bash
rs-pancat-reconstruct graph.gfa -n name_file.txt > output.fa
```

> [!NOTE]\
> Want to contribute? Feel free to open a PR on an issue about a missing, buggy or incomplete feature! **Please do bug reports in the issue tracker!**.
