pub static PACKAGE_INITIALIZATION_CONFIGURATION: app::PackageInitializationConfiguration =
    app::PackageInitializationConfiguration {
        command_main_file_content: "main : System -> Number\nmain system = 0\n",
        library_main_file_content: "export { foo }\n\nfoo : Number -> Number\nfoo x = x\n",
        library_main_basename: "Foo",
    };
