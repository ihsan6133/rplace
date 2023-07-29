# rplace
 
(Currently a work in progress)

A data analysis library for r/place.
This crate contains both a binary and library.
The binary application contains useful 
utilities and also is necessary for installing
and processing the place dataset for use by
the library.

## Installing the dataset

1. Install the rplace binary via `cargo install rplace`
2. Run `cargo rplace --build-dataset <year>`

Year represents which year of r/place you want to download,
(2017/2022/2023). Currently only 2023 is supported.
The data will be installed to `./dataset`. Make sure
the directory has write permissions and there is enough
remaining storage in the drive. An internet connection is
required. This command may take a long time, depending
upon the speed of your internet connection, since the
dataset is huge.
