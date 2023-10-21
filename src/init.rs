pub fn init_project() {
    // std::fs::File::create("velcro.toml").expect("create velcro.toml");
    std::fs::create_dir("../indices").expect("create ./mappings dir");
    std::fs::create_dir("templates").expect("create ./templates dir");
}
