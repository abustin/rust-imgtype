#[deriving(Show)]
pub enum FileFormat { JPEG, PNG, WEBP, GIF, TIFF }

static JPEG_SPEC:[Option<u8>, ..11] = [Some(0xFF), Some(0xD8), Some(0xFF), Some(0xE0), 
                                       None,       None,       Some(0x4A), Some(0x46), 
                                       Some(0x49), Some(0x46), Some(0x00)];

static PNG_SPEC:[Option<u8> ,..8]   = [Some(0x89), Some(0x50), Some(0x4E), Some(0x47), 
                                       Some(0x0D), Some(0x0A), Some(0x1A), Some(0x0A)];

static WEBP_SPEC:[Option<u8>, ..12] = [Some(0x52), Some(0x49), Some(0x46), Some(0x46), 
                                       None,       None,       None,       None, 
                                       Some(0x57), Some(0x45), Some(0x42), Some(0x50)];

static GIF_SPEC:[Option<u8>, ..6]   = [Some(0x47), Some(0x49), Some(0x46), Some(0x38), 
                                       None,       Some(0x61)];

static TIFF_SPEC:[Option<u8>, ..3]   = [Some(0x49), Some(0x20), Some(0x49)];

static FILE_TESTS:[(FileFormat, &'static [Option<u8>]), ..5] = [
    (FileFormat::JPEG, &JPEG_SPEC),
    (FileFormat::PNG , &PNG_SPEC ),
    (FileFormat::WEBP, &WEBP_SPEC),
    (FileFormat::GIF , &GIF_SPEC ),
    (FileFormat::TIFF, &TIFF_SPEC)
];

pub fn from_buffer(file:&[u8]) -> Option<FileFormat> {
    match FILE_TESTS.iter().filter(|aspec| {
        file.iter().zip(aspec.val1().iter()).all(|file_spec| {
            let (file,spec) = file_spec;
            match spec {
                &Some(bit) => (bit == *file),
                &None => true
            }
        })
    }).next() {
        Some(found) => Some(found.val0()),
        None => None
    }    
}

#[test]
fn it_works() {
    let file = [0xFFu8, 0xD8, 0xFF, 0xE0, 0xA1, 0xA2, 0x4A, 0x46, 0x49, 0x46, 0x00];
    println!("{}", from_buffer(&file));
}
