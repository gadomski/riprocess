# riprocess

Rust library and executable for generating configuration files for Riegl's RiPROCESS software.

**NOTE: This software is not developed by Riegl.
Do not contact Riegl with any questions or comments about this software.**

## Installation

To download and install the included executable, you'll need [cargo](https://www.rustup.rs/).
Then, install from Github:

```bash
cargo install --git https://github.com/gadomski/RiPROCESS
```

## Usage

For now, the only capability provided by the `riprocess` executable is `image-list`, used as such:

```bash
riprocess image-list <config>
```

This will print a semicolon-delimited list of timestamps and image paths, which can be imported into RiPROCESS's camera data wizard to create camera records.

The `<config>` argument is a configuration file in the [TOML](https://github.com/toml-lang/toml) format.
You'll need the following information:

- The path to the directory that holds the images you're
- The image number of the first image.
  If the first image in the directory is the first image you'd like to use, you can omit this argument.
- The image number of the last image.
  Same as above, if the last image in the directory is your last image, you can omit this information.
- The path to the directory that holds the timestamp files.
- The file names for the first and last timestamp files you'd like to use.
  Same as with the image numbers, you can omit these if the first/last filename in the directory is your first/last name.
- The record start times for each record you're creating camera records for.
  This information can be found in the `90_CAMERA_DATA_WIZARD\########-######\Records.csv` file in your RiPROCESS project tree (assuming you've run the camera data wizard at least once).
  Use the "Start(stamp)" field.

See `data/config.toml` in the source directory of this project for how you'll want to lay out this information.
Once you've set up your config file, you can run the process and pipe the output to a text file, for import to RiPROCESS:

```bash
riprocess image-list my-config-file.toml > my-image-list.txt
```

The process will exit with an error if there's a mismatch of any sort, e.g. the number of timestamps doesn't correspond to the number of images.
