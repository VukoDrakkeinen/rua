#[global_allocator]
static GLOBAL: std::alloc::System = std::alloc::System;

mod action_builddir;
mod action_install;
mod action_search;
mod action_upgrade;
mod aur_rpc_utils;
mod cli_args;
mod git_utils;
mod pacman;
mod print_format;
mod print_package_info;
mod print_package_table;
mod reviewing;
mod rua_environment;
mod rua_files;
mod srcinfo_to_pkgbuild;
mod tar_check;
mod terminal_util;
mod wrapped;

use crate::print_package_info::info;
use crate::wrapped::shellcheck;
use cli_args::Action;
use cli_args::CliArgs;
use std::process::exit;
use structopt::StructOpt;

fn main() {
	let cli_args: CliArgs = CliArgs::from_args();
	let rua_env = rua_environment::prepare_environment(&cli_args);
	match cli_args.action {
		Action::Info { ref target } => {
			info(target, false).unwrap();
		}
		Action::Install {
			asdeps,
			offline,
			target,
		} => {
			action_install::install(&target, &rua_env, offline, asdeps);
		}
		Action::Builddir {
			offline,
			force,
			target,
		} => {
			action_builddir::action_builddir(target, &rua_env, offline, force);
		}
		Action::Search { target } => action_search::action_search(target),
		Action::Shellcheck { target } => {
			let result = shellcheck(target);
			result
				.map_err(|err| {
					eprintln!("{}", err);
					exit(1);
				})
				.ok();
		}
		Action::Tarcheck { target } => {
			tar_check::tar_check_unwrap(&target);
			eprintln!("Finished checking package: {:?}", target);
		}
		Action::Upgrade { devel, printonly } => {
			action_upgrade::upgrade(&rua_env, devel, printonly);
		}
	};
}
