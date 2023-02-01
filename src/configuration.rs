#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

// get the configuratio from a YAML file and store in a struct. For this it needs to:
// - store in a variable of the type Config(struct)
// - use external crate to make a re-readable instance of a struct (my limited understanding) that can be shared and serialized
// - builds the instance

// - add_source self explanatory
// - tries to deserialize and returns Settings struct or Errors.
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialise a config reader.
    let settings = config::Config::builder()
        // from a YAML file (name configurations.yaml)
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;

    settings.try_deserialize::<Settings>()
}
