use clap::{Parser, Subcommand};
use inflections::case;
use snmp::SyncSession;
use std::time::Duration;
use strum::IntoEnumIterator;
use w6otx::w6otx_snmp;

const DEFAULT_HOST: &str = "apc-rpdu:161";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value=DEFAULT_HOST, required=false)]
    host: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    PowerOn { outlet: w6otx_snmp::Outlet },
    PowerOff { outlet: w6otx_snmp::Outlet },
    Bounce { outlet: w6otx_snmp::Outlet },
    Status { outlet: w6otx_snmp::Outlet },
}

fn main() {
    let args = Args::parse();
    let host = args.host;
    let community = b"private";
    let timeout = Duration::from_secs(5);
    let mut session = SyncSession::new(host, community, Some(timeout), 0)
        .expect("Failed to create SNMP session.");

    match &args.command {
        Some(Commands::PowerOn { outlet }) => {
            let command = w6otx_snmp::OutletControlCommand::ImmediateOn;
            w6otx_snmp::control_outlet(&mut session, *outlet, command)
                .expect("Failed to control outlet.");
        }
        Some(Commands::PowerOff { outlet }) => {
            let command = w6otx_snmp::OutletControlCommand::ImmediateOff;
            w6otx_snmp::control_outlet(&mut session, *outlet, command)
                .expect("Failed to control outlet.");
        }
        Some(Commands::Bounce { outlet }) => {
            let command = w6otx_snmp::OutletControlCommand::ImmediateReboot;
            w6otx_snmp::control_outlet(&mut session, *outlet, command)
                .expect("Failed to control outlet.");
        }
        Some(Commands::Status { outlet }) => {
            let status = w6otx_snmp::get_outlet_status(&mut session, *outlet)
                .expect("Failed to get outlet status.");
            let kebab_cased_outlet = case::to_kebab_case(outlet.into());
            println!("{kebab_cased_outlet} {status:?}");
        }
        None => {
            for outlet in w6otx_snmp::Outlet::iter() {
                let status = w6otx_snmp::get_outlet_status(&mut session, outlet)
                    .expect("Failed to get outlet status.");
                let kebab_cased_outlet = case::to_kebab_case(outlet.into());
                println!("{kebab_cased_outlet} {status:?}");
            }
        }
    }
}
