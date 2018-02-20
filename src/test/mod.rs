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

mathema_test! {
    add_file is |env| {
        env.assert_mathema("")
           .with_args(&["new", "foo"])
           .unwrap();

        env.write_file("foo/bar.cards", "\
en hello
gr γιάσου
")
           .unwrap();

        env.assert_mathema("foo")
           .with_args(&["add", "bar.cards"])
           .stdout()
           .contains("`bar.cards` added to database.")
           .and()
           .stdout()
           .contains("1 new card found.")
           .unwrap();

        env.assert_git("foo")
           .with_args(&["status"])
           .stdout().contains("nothing to commit, working directory clean")
           .unwrap();

        env.assert_git("foo")
           .with_args(&["show", "--stat", "HEAD"])
           .stdout().contains(" bar.cards        | 3 +++")
           .unwrap();
    }
}

mathema_test! {
    add_transliterates is |env| {
        env.assert_mathema("")
           .with_args(&["new", "foo"])
           .unwrap();

        env.write_file("foo/bar.cards", "\
en hello
gr gi;asoy
")
           .unwrap();

        env.assert_mathema("foo")
           .with_args(&["add", "bar.cards"])
           .stdout()
           .contains("`bar.cards` added to database.")
           .and()
           .stdout()
           .contains("1 new card found.")
           .unwrap();

        let string = env.read_file("foo/bar.cards").unwrap();
        assert!(string.contains("gr γιάσου"), "file does not contain γιάσου:\n{}", string);
    }
}

mathema_test! {
    add_added_file is |env| {
        env.assert_mathema("")
           .with_args(&["new", "foo"])
           .unwrap();

        env.write_file("foo/bar.cards", "\
en hello
gr γιάσου
")
           .unwrap();

        env.assert_mathema("foo")
           .with_args(&["add", "bar.cards"])
           .stdout()
           .contains("`bar.cards` added to database.")
           .unwrap();

        env.append_file("foo/bar.cards", "
en water
gr νερό
")
           .unwrap();

        env.assert_mathema("foo")
           .with_args(&["add", "bar.cards"])
           .stdout()
           .contains("`bar.cards` already found in database.")
           .and()
           .stdout()
           .contains("1 new card found.")
           .unwrap();

        env.assert_git("foo")
           .with_args(&["status"])
           .stdout().contains("nothing to commit, working directory clean")
           .unwrap();

        env.assert_git("foo")
           .with_args(&["show", "--stat", "HEAD"])
           .stdout().contains(" bar.cards | 4 ++++")
           .unwrap();
    }
}
