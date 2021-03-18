use lazy_static::lazy_static;

lazy_static! {
    pub static ref DEFAULT_SYSTEM_PACKAGE_CONFIGURATION: app::SystemPackage =
        app::SystemPackage::new("github.com/ein-lang/os", "main");
}

pub static PACKAGE_INITIALIZATION_CONFIGURATION: app::PackageInitializationConfiguration =
    app::PackageInitializationConfiguration {
        application_main_file_content: indoc::indoc!(
            "
            main : System -> Number
            main sys =
              let
                _ = fdWrite sys stdout \"Hello, world!\\n\"
              in
                0
            "
        ),
        library_main_file_content: indoc::indoc!(
            "
            export { foo }

            foo : Number -> Number
            foo x = x
            "
        ),
        library_main_basename: "Foo",
    };
