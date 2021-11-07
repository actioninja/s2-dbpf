# s2-dbpf

A Rust crate for efficient parsing and writing of data formats of The Sims 2.

Currently heavily work in progress and not ready for usage.

## Goals

- Provide a "close to format" method of working with DBPF files and all the data formats contained within them, as well
  as an "easy" interface that represents the logical usage much better.
- Be as stack friendly and efficient as possible while still retaining readability (avoid "memory magic")
- Act as a form of documentation of the data formats
- Serve as a base platform for later high performance usage

## Support Table

| Format | Raw Support | Integration Tests | Easy Types |
| ------ | ----------- | ----------------- | ---------- |
| DBPF   | No | No | No |
| UI   | No | No | No |
| WGRA | No | No | No |
| TRKS | No | No | No |
| DESC | No | No | No |
| BINX | No | No | No |
| POOL | No | No | No |
| TXTR | No | No | No |
| XA   | No | No | No |
| 5SC  | No | No | No |
| 3ARY | No | No | No |
| XTOL | No | No | No |
| POPS | No | No | No |
| SCOR | No | No | No |
| BCON | No | No | No |
| BHAV | Yes | Yes | No |
| BMP  | No | No | No |
| CATS | No | No | No |
| CIGE | No | No | No |
| CTSS | No | No | No |
| DGRP | No | No | No |
| FACE | No | No | No |
| FAMI | No | No | No |
| FAMh | No | No | No |
| FCNS | No | No | No |
| FWAV | No | No | No |
| GLOB | No | No | No |
| HOUS | No | No | No |
| TXMT | No | No | No |
| WRLD | No | No | No |
| LTTX | No | No | No |
| XSTN | No | No | No |
| CINE | No | No | No |
| NGBH | No | No | No |
| NREF | No | No | No |
| NMAP | No | No | No |
| OBJD | No | No | No |
| OBJF | No | No | No |
| OBJM | No | No | No |
| PALT | No | No | No |
| PERS | No | No | No |
| POSI | No | No | No |
| PTBP | No | No | No |
| SIMI | No | No | No |
| SLOT | No | No | No |
| SPR2 | No | No | No |
| STR# | No | No | No |
| TTAT | No | No | No |
| TPRP | No | No | No |
| TRCN | No | No | No |
| TREE | No | No | No |
| TTAB | No | No | No |
| TTAs | No | No | No |
| XMTO | No | No | No |
| XOBJ | No | No | No |
| 5EL  | No | No | No |
| 2ARY | No | No | No |
| LOT  | No | No | No |
| MOBJT | No | No | No |
| HLS  | No | No | No |
| GMND | No | No | No |
| LTMP | No | No | No |
| WLL  | No | No | No |
| UNK1 | No | No | No |
| JPG  | No | No | No |
| FAMt | No | No | No |
| PMAP | No | No | No |
| SFX  | No | No | No |
| UNK2 | No | No | No |
| PDAT | No | No | No |
| FPL  | No | No | No |
| ROOF | No | No | No |
| LOTG | No | No | No |
| NHTR | No | No | No |
| 5LF  | No | No | No |
| 5DS  | No | No | No |
| GMDC | No | No | No |
| 3IDR | No | No | No |
| NID  | No | No | No |
| WTHR | No | No | No |
| TSSG | No | No | No |
| LGHT | No | No | No |
| SMAP | No | No | No |
| VERT | No | No | No |
| UNK3 | No | No | No |
| SREL | No | No | No |
| UNK4 | No | No | No |
| LxNR | No | No | No |
| MATSHAD | No | No | No |
| SWAF | Yes | No | No |
| CREG | No | No | No |
| CRES | No | No | No |
| DIR  | No | No | No |
| FX   | No | No | No |
| GZPS | No | No | No |
| VERS | No | No | No |
| NHVW | No | No | No |
| LIFO | No | No | No |
| OBJT | No | No | No |
| ANIM | No | No | No |
| SHPE | No | No | No |

## License

s2-dbpf is licensed under the Mozilla Public License 2.0 (MPL 2.0)

According to tl;drLegal, MPL is:

> a copyleft license that is easy to comply with. You must make the source code for any of your changes
> available under MPL, but you can combine the MPL software with proprietary code, as long as you keep the MPL
> code in separate files. Version 2.0 is, by default, compatible with LGPL and GPL version 2 or greater. You
> can distribute binaries under a proprietary license, as long as you make the source available under MPL.

See [MPL 2.0 on tl;dr legal](https://tldrlegal.com/license/mozilla-public-license-2.0-(mpl-2)) for a bit more human
readable detail, and the full license for the actual legally binding stuff.

