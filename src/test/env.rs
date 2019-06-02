//! My test fixture for actually running the execute.
//!
//! Run like:
//!
//! ```
//! mathema_test! {
//!     test_name is |env| {
//!         ...
//!     }
//! }
//! ```

extern crate assert_cli;
extern crate tempdir;

use self::assert_cli::Assert;
use self::tempdir::TempDir;
use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*};
use std::path::Path;

macro_rules! mathema_test {
    ($test_name:ident is $closure:expr) => {
        #[test]
        fn $test_name() {
            env::mathema_test("foo", "foo", 22, $closure)
        }
    };
}

crate struct TestEnv {
    temp_dir: TempDir,
}

crate fn mathema_test(
    test_name: &str,
    file: &str,
    line: usize,
    mut body: impl FnMut(&mut TestEnv),
) {
    let temp_dir = TempDir::new(&format!("mathema-test-{}-{}-{}", test_name, file, line)).unwrap();
    body(&mut TestEnv { temp_dir })
}

impl TestEnv {
    /// Returns an `Assert` that is configured to run `mathema`.
    crate fn assert_mathema(&mut self, in_dir: &str) -> Assert {
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        Assert::command(&[
            "cargo",
            "run",
            "--manifest-path",
            manifest_dir.to_str().unwrap(),
            "--",
        ])
        .current_dir(self.temp_dir.path().join(in_dir).to_owned())
    }

    crate fn assert_git(&mut self, in_dir: &str) -> Assert {
        Assert::command(&["git"]).current_dir(self.temp_dir.path().join(in_dir).to_owned())
    }

    crate fn write_file(&mut self, name: &str, contents: &str) -> io::Result<()> {
        let path = self.temp_dir.path().join(name);
        let mut file = File::create(path)?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }

    crate fn append_file(&mut self, name: &str, contents: &str) -> io::Result<()> {
        let path = self.temp_dir.path().join(name);
        let mut file = OpenOptions::new().append(true).open(path)?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }

    crate fn read_file(&self, name: &str) -> io::Result<String> {
        let path = self.temp_dir.path().join(name);
        let mut buf = String::new();
        let mut file = File::open(path)?;
        file.read_to_string(&mut buf)?;
        Ok(buf)
    }
}
