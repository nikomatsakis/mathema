#![cfg(test)]

#[macro_use]
mod env;

mathema_test! {
    new_twice is |env| {
        env.assert_mathema("")
           .with_args(&["new", "foo"])
           .unwrap();

        env.assert_mathema("")
           .with_args(&["new", "foo"])
           .stderr()
           .contains("Failed to create directory `foo`: File exists")
           .and()
           .fails()
           .unwrap();
    }
}

mathema_test! {
    git_initialization is |env| {
        env.assert_mathema("")
           .with_args(&["new", "foo"])
           .unwrap();

        env.assert_git("foo")
           .with_args(&["status"])
           .stdout()
           .contains("On branch master\nnothing to commit, working directory clean")
           .unwrap();
    }
}

mathema_test! {
    status_unknown_files is |env| {
        env.assert_mathema("")
           .with_args(&["new", "foo"])
           .unwrap();

        env.write_file("foo/bar.cards", "\
en hello
gr γιάσου
")
           .unwrap();

        env.assert_mathema("foo")
           .with_args(&["status"])
           .stdout()
           .contains("Unknown card files")
           .stdout()
           .contains("bar.cards")
           .unwrap();
    }
}
