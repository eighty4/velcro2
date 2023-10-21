use clap::{Parser, Subcommand};

use crate::dev::start_dev_mode;
use crate::init::init_project;
use crate::new::create_new_resource;
use crate::strap::velcro_strap;

mod dev;
mod elasticsearch;
mod init;
mod model;
mod new;
mod strap;
mod util;

#[derive(Parser)]
#[command(author, version, about)]
struct VelcroCli {
    #[command(subcommand)]
    command: VelcroCommand,
}

#[derive(Subcommand)]
enum VelcroCommand {
    Init(InitCommandArgs),
    Dev(DevCommandArgs),
    New(NewCommandArgs),
    Strap(StrapCommandArgs),
}

#[derive(Parser, Debug)]
struct InitCommandArgs {}

#[derive(Parser, Debug)]
struct DevCommandArgs {}

#[derive(Parser, Debug)]
struct NewCommandArgs {}

#[allow(dead_code)]
#[derive(Debug)]
enum NewResource {
    Mapping(String),
    Template,
}

#[derive(Parser, Debug)]
struct StrapCommandArgs {}

#[tokio::main]
async fn main() {
    println!("Velcro2 - a cli dx wip for Elasticsearch");
    match VelcroCli::parse().command {
        VelcroCommand::Init(_) => init_project(),
        VelcroCommand::Dev(_) => start_dev_mode(),
        VelcroCommand::New(_) => create_new_resource(),
        VelcroCommand::Strap(_) => velcro_strap(),
    }
}
