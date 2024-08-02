# Inksmash

Command-line tool to compress [Inkscape](https://inkscape.org) '.svg' files containing embedded images without a visual loss of quality.

## Install

Binary available for download for both [Windows and Linux](https://github.com/envidera/inksmash/releases). To install, simply extract the binary to a folder within your environment PATH.


## Usage

First, you need to run the command `Clean Up Document` in Inkscape, and save it.
This cleans up the document, reorders its content, and saves it by overwriting the existing file.

You can do this in the Inkscape program:

    Open your svg Inkscape file
    File > Clean Up Document
    File > Save

or by command line:
```txt
inkscape --vacuum-defs --export-overwrite  <FILE>
```

Then you can run Inksmash on the file

```txt
inksmash [OPTIONS] <FILE>
```

example:
```
inkscape --vacuum-defs --export-overwrite  my-catalog.svg
inksmash my-catalog.svg
```

Inksmash will compress and return a new file with a .min.svg extension.

> You can compress this file as many times as you want; only new embedded images will be compressed, while already compressed images will be ignored by the compressor.

Created and tested with Inkscape v1.3.2 on Linux.

### Options
```txt
-q, --quality <QUALITY>  range 1-100, where 1 is the worst quality and 100 is the best [default: 90]
-m, --move-completed     move completed files to the 'compressed' folder
-e, --extract            extract compressed images to the 'extracted' folder
-h, --help               Print help
-V, --version            Print version
```


## How its works?

- It recompresses all embedded images using the WebP image format with lossy compression, while maintaining the alpha channel (transparency) if present.


- Extracted images do not contain duplicates; it computes and compares the MD5 hash of each one.

- Inksmash is created using [Rust](https://www.rust-lang.org/).


## Linux Fedora Usage Suggestion

On Linux Fedora with GNOME, you can right-click on .svg files and compress them using Script > Inksmash by creating and saving the following bash script with the name `Inksmash` in `$HOME/.local/share/nautilus/scripts`:

```bash
#!/usr/bin/env bash

for f in "$@"; do
    # Clean up, reorder the document, and save it by overwriting the existing file.
	inkscape --vacuum-defs --export-overwrite  "${f}"
	
	# Run Inksmash
    # This will create a compressed copy with a .min.svg extension.
	inksmash "${f}"	
done
```



## Why Inksmash?

Envidera is powered by open-source [operating systems](https://fedoraproject.org) and programs, and uses [Inkscape](https://inkscape.org) for all graphic design.

Embedded images in Inkscape files can increase their size and make them more resource-intensive to use. Inksmash compresses these files, reducing their size and resource usage.

## License

GPL-3.0