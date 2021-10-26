# s2-dbpf

A Rust crate for efficient parsing and writing of data formats of The Sims 2.

Currently heavily work in progress and not ready for usage.

## Goals

- Provide a "close to format" method of working with DBPF files and all the data formats contained within them
- Automate away as much of the potential vectors of producing malformed files as possible
- Be as stack friendly and efficient as possible while still retaining readability (avoid "memory magic")
- Act as a form of documentation of the data formats

## License

s2-dbpf is licensed under the Mozilla Public License 2.0 (MPL 2.0)

According to tl;drLegal, MPL is:

> a copyleft license that is easy to comply with. You must make the source code for any of your changes
> available under MPL, but you can combine the MPL software with proprietary code, as long as you keep the MPL
> code in separate files. Version 2.0 is, by default, compatible with LGPL and GPL version 2 or greater. You
> can distribute binaries under a proprietary license, as long as you make the source available under MPL.

See [MPL 2.0 on tl;dr legal](https://tldrlegal.com/license/mozilla-public-license-2.0-(mpl-2)) for a bit more human
readable detail, and the full license for the actual legally binding stuff.

