#![cfg(test)]

#[macro_use]
mod env;

mathema_test! {
    new_twice is |env| {
        env.assert_mathema()
           .with_args(&["new", "foo"])
           .unwrap();

        env.assert_mathema()
           .with_args(&["new", "foo"])
           .stderr()
           .contains("failed to create directory `foo`")
           .and()
           .fails()
           .unwrap();
    }
}
