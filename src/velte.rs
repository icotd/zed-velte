use zed_extension_api as zed;

struct VelteExtension;

impl zed::Extension for VelteExtension {
    fn new() -> Self {
        Self
    }
}

zed::register_extension!(VelteExtension);
