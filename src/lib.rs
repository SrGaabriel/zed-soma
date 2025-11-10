use zed_extension_api as zed;

struct SomaExtension;

impl zed::Extension for SomaExtension {
    fn new() -> Self where Self: Sized {
        Self
    }
}

zed::register_extension!(SomaExtension);
