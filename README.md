#rust-imgtype [![Build Status](https://travis-ci.org/abustin/rust-imgtype.svg)](https://travis-ci.org/abustin/rust-imgtype.svg)

Finds your image type by looking at the files header.  No external dependencies.

###Supported Types
```rust
pub enum ImageType { JPEG, PNG, WEBP, GIF, TIFF }
```

###Example
```rust
extern crate imgtype;

use imgtype;
use imgtype::ImageType;

fn print_imgtype(file:&[u8]) {

    let img_type:Option<ImageType> = imgtype::from_buffer(file);

    match img_type {
        Some(img_type) => println!("Image Type is {}!", img_type),
        None           => println!("Unknown Image Type.")
    };

}
```

###Cargo.toml

```toml
[dependencies.imgtype]
git = "https://github.com/abustin/rust-imgtype"
```