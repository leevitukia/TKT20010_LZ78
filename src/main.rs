use std::path::Path;
mod lz78;


fn main() {
    lz78::encode(Path::new("./TestFiles/test1.txt"), Path::new("output.lz78"));
    lz78::decode(Path::new("output.lz78"), Path::new("./TestFiles/test1Decoded.txt"));
    //encode(Path::new("./Screenshot.bmp"), Path::new("Screenshot.lz78"));
    //decode(Path::new("Screenshot.lz78"), Path::new("./ScreenshotDecoded.bmp"));
}