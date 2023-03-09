extern crate directories;
use directories::{BaseDirs, UserDirs, ProjectDirs};

struct State {
    has_configured: bool
}

impl State {

    fn load_configure(&mut self) {
        if let Some(proj_dirs) = ProjectDirs::from("com", "epiphany",  "broca") {
            proj_dirs.config_dir();
            // Lin: /home/alice/.config/barapp
            // Win: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\config
            // Mac: /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App
        }
    }
}
