use std::path::PathBuf;
use partial_config::{config, Build, Run, ConfigBuilder};
use partial_functional::prelude::*;

config!{
    #[derive(Debug)]
    pub struct Config {
        output_file: { Last<PathBuf>, Option<PathBuf> },
        verbose: { Sum<u32>, u32 },
        debug: { Any, bool },
    }
}

impl ConfigBuilder for Config<Build> {
    type Target = Config<Run>;

    fn build(self) -> Self::Target {
        Self::Target {
            output_file: self.output_file.into(),
            verbose: self.verbose.into(),
            debug: self.debug.into(),
        }
    }
}

impl ConfigBuilder for Config<Run> {
    type Target = Config<Build>;

    fn build(self) -> Self::Target {
        Self::Target {
            output_file: self.output_file.into(),
            verbose: self.verbose.into(),
            debug: self.debug.into(),
        }
    }
}

#[test]
fn combinging_three_config_types() {
    let config = Config::<Build>::default();
    let env_config = Config::<Build> {
        verbose: 1.into(),
        debug: true.into(),
        ..Default::default()
    };
    let arg_config = Config::<Build> {
        verbose: 2.into(),
        debug: false.into(),
        output_file: Some(PathBuf::from("/usr/bin/test")).into(),
    };

    let config = partial_functional::combine!(config, env_config, arg_config);

    assert_eq!(&Last::from(PathBuf::from("/usr/bin/test")), config.output_file.get());
    assert_eq!(&Sum(3), config.verbose.get());
    assert_eq!(&Any(true), config.debug.get());
}

#[test]
fn initial_config_should_be_build_values() {
    let config = Config::<Build>::default();

    assert_eq!(&Last::empty(), config.output_file.get());
    assert_eq!(&Sum::<u32>::empty(), config.verbose.get());
    assert_eq!(&Any::empty(), config.debug.get());
}

#[test]
fn default_config_build() {
    let config = Config::<Build>::default();

    let config = config.build();

    assert_eq!(&None, config.output_file.value());
    assert_eq!(0, config.verbose.get());
    assert_eq!(false, config.debug.get());
}

#[test]
fn build_and_then_build_again_should_be_same_as_initial() {
    let config = Config::<Build>::default();

    let config = config.build();
    let config = config.build();

    assert_eq!(&Last::empty(), config.output_file.get());
    assert_eq!(&Sum::<u32>::empty(), config.verbose.get());
    assert_eq!(&Any::empty(), config.debug.get());
}
