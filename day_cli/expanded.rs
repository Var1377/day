#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
#[macro_use]
extern crate macros;

mod config {

    use clap::{Args, Subcommand};
    use traits::Runnable;
    pub struct ConfigArgs {
        #[clap(subcommand)]
        subcmd: Option<SubCommand>,
    }
    #[allow(
        dead_code,
        unreachable_code,
        unused_variables,
        unused_braces,
        unused_qualifications
    )]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
        clippy::almost_swapped
    )]
    #[automatically_derived]
    impl clap::FromArgMatches for ConfigArgs {
        fn from_arg_matches(
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn from_arg_matches_mut(
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            #![allow(deprecated)]
            let v = ConfigArgs {
                subcmd: {
                    if __clap_arg_matches
                        .subcommand_name()
                        .map(<SubCommand as clap::Subcommand>::has_subcommand)
                        .unwrap_or(false)
                    {
                        Some(<SubCommand as clap::FromArgMatches>::from_arg_matches_mut(
                            __clap_arg_matches,
                        )?)
                    } else {
                        None
                    }
                },
            };
            ::std::result::Result::Ok(v)
        }
        fn update_from_arg_matches(
            &mut self,
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn update_from_arg_matches_mut(
            &mut self,
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            #![allow(deprecated)]
            {
                #[allow(non_snake_case)]
                let subcmd = &mut self.subcmd;
                if let Some(subcmd) = subcmd.as_mut() {
                    <SubCommand as clap::FromArgMatches>::update_from_arg_matches_mut(
                        subcmd,
                        __clap_arg_matches,
                    )?;
                } else {
                    *subcmd = Some(<SubCommand as clap::FromArgMatches>::from_arg_matches_mut(
                        __clap_arg_matches,
                    )?);
                }
            }
            ::std::result::Result::Ok(())
        }
    }
    #[allow(
        dead_code,
        unreachable_code,
        unused_variables,
        unused_braces,
        unused_qualifications
    )]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
        clippy::almost_swapped
    )]
    #[automatically_derived]
    impl clap::Args for ConfigArgs {
        fn group_id() -> Option<clap::Id> {
            Some(clap::Id::from("ConfigArgs"))
        }
        fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
            {
                let __clap_app =
                    __clap_app.group(clap::ArgGroup::new("ConfigArgs").multiple(true).args({
                        let members: [clap::Id; 0usize] = [];
                        members
                    }));
                let __clap_app = <SubCommand as clap::Subcommand>::augment_subcommands(__clap_app);
                let __clap_app = __clap_app;
                __clap_app
            }
        }
        fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
            {
                let __clap_app =
                    __clap_app.group(clap::ArgGroup::new("ConfigArgs").multiple(true).args({
                        let members: [clap::Id; 0usize] = [];
                        members
                    }));
                let __clap_app = <SubCommand as clap::Subcommand>::augment_subcommands(__clap_app);
                let __clap_app = __clap_app
                    .subcommand_required(false)
                    .arg_required_else_help(false);
                __clap_app
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ConfigArgs {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "ConfigArgs",
                "subcmd",
                &&self.subcmd,
            )
        }
    }
    impl Runnable for ConfigArgs {
        type Args = Cli;
        fn run(&self, args: &Self::Args) -> anyhow::Result<()> {
            match &self.subcmd {
                Some(subcmd) => subcmd.run(args),
                None => {
                    {
                        ::std::io::_print(format_args!(
                            "No subcommand provided. Try `day config --help`\n"
                        ));
                    };
                    Ok(())
                }
            }
        }
    }
    #[parent(ConfigArgs)]
    pub enum SubCommand {
        Sleep(SleepArgs),
    }
    #[allow(
        dead_code,
        unreachable_code,
        unused_variables,
        unused_braces,
        unused_qualifications
    )]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
        clippy::almost_swapped
    )]
    #[automatically_derived]
    impl clap::FromArgMatches for SubCommand {
        fn from_arg_matches(
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn from_arg_matches_mut(
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            #![allow(deprecated)]
            if let Some((__clap_name, mut __clap_arg_sub_matches)) =
                __clap_arg_matches.remove_subcommand()
            {
                let __clap_arg_matches = &mut __clap_arg_sub_matches;
                if __clap_name == "sleep" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Sleep(
                        <SleepArgs as clap::FromArgMatches>::from_arg_matches_mut(
                            __clap_arg_matches,
                        )?,
                    ));
                }
                ::std::result::Result::Err(clap::Error::raw(
                    clap::error::ErrorKind::InvalidSubcommand,
                    {
                        let res = ::alloc::fmt::format(format_args!(
                            "The subcommand \'{0}\' wasn\'t recognized",
                            __clap_name
                        ));
                        res
                    },
                ))
            } else {
                ::std::result::Result::Err(clap::Error::raw(
                    clap::error::ErrorKind::MissingSubcommand,
                    "A subcommand is required but one was not provided.",
                ))
            }
        }
        fn update_from_arg_matches(
            &mut self,
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn update_from_arg_matches_mut<'b>(
            &mut self,
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            #![allow(deprecated)]
            if let Some(__clap_name) = __clap_arg_matches.subcommand_name() {
                match self {
                    Self::Sleep(ref mut __clap_arg) if "sleep" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) =
                            __clap_arg_matches.remove_subcommand().unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        clap::FromArgMatches::update_from_arg_matches_mut(
                            __clap_arg,
                            __clap_arg_matches,
                        )?
                    }
                    s => {
                        *s = <Self as clap::FromArgMatches>::from_arg_matches_mut(
                            __clap_arg_matches,
                        )?;
                    }
                }
            }
            ::std::result::Result::Ok(())
        }
    }
    #[allow(
        dead_code,
        unreachable_code,
        unused_variables,
        unused_braces,
        unused_qualifications
    )]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
        clippy::almost_swapped
    )]
    #[automatically_derived]
    impl clap::Subcommand for SubCommand {
        fn augment_subcommands<'b>(__clap_app: clap::Command) -> clap::Command {
            let __clap_app = __clap_app;
            let __clap_app = __clap_app.subcommand({
                let __clap_subcommand = clap::Command::new("sleep");
                let __clap_subcommand = __clap_subcommand;
                let __clap_subcommand =
                    { <SleepArgs as clap::Args>::augment_args(__clap_subcommand) };
                __clap_subcommand
            });
            __clap_app
        }
        fn augment_subcommands_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
            let __clap_app = __clap_app;
            let __clap_app = __clap_app.subcommand({
                let __clap_subcommand = clap::Command::new("sleep");
                let __clap_subcommand = __clap_subcommand;
                let __clap_subcommand =
                    { <SleepArgs as clap::Args>::augment_args_for_update(__clap_subcommand) };
                __clap_subcommand
            });
            __clap_app
        }
        fn has_subcommand(__clap_name: &str) -> bool {
            if "sleep" == __clap_name {
                return true;
            }
            false
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for SubCommand {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                SubCommand::Sleep(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Sleep", &__self_0)
                }
            }
        }
    }
    impl ::traits::Runnable for SubCommand {
        type Args = parent(ConfigArgs);
        fn run(&self, args: &Self::Args) -> anyhow::Result<()> {
            match self {
                SubCommand::Sleep(field) => field.run(args),
            }
        }
    }
    struct SleepArgs {}
    #[allow(
        dead_code,
        unreachable_code,
        unused_variables,
        unused_braces,
        unused_qualifications
    )]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
        clippy::almost_swapped
    )]
    #[automatically_derived]
    impl clap::FromArgMatches for SleepArgs {
        fn from_arg_matches(
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn from_arg_matches_mut(
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            #![allow(deprecated)]
            let v = SleepArgs {};
            ::std::result::Result::Ok(v)
        }
        fn update_from_arg_matches(
            &mut self,
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn update_from_arg_matches_mut(
            &mut self,
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            #![allow(deprecated)]
            ::std::result::Result::Ok(())
        }
    }
    #[allow(
        dead_code,
        unreachable_code,
        unused_variables,
        unused_braces,
        unused_qualifications
    )]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
        clippy::almost_swapped
    )]
    #[automatically_derived]
    impl clap::Args for SleepArgs {
        fn group_id() -> Option<clap::Id> {
            Some(clap::Id::from("SleepArgs"))
        }
        fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
            {
                let __clap_app =
                    __clap_app.group(clap::ArgGroup::new("SleepArgs").multiple(true).args({
                        let members: [clap::Id; 0usize] = [];
                        members
                    }));
                __clap_app
            }
        }
        fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
            {
                let __clap_app =
                    __clap_app.group(clap::ArgGroup::new("SleepArgs").multiple(true).args({
                        let members: [clap::Id; 0usize] = [];
                        members
                    }));
                __clap_app
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for SleepArgs {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "SleepArgs")
        }
    }
    impl Runnable for SleepArgs {
        type Args = ConfigArgs;
        fn run(&self, args: &Self::Args) -> anyhow::Result<()> {
            {
                ::std::io::_print(format_args!("Sleep config\n"));
            };
            Ok(())
        }
    }
    pub struct Config {
        /// time breaking / time working
        pub break_ratio: f64,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Config {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Config",
                "break_ratio",
                &&self.break_ratio,
            )
        }
    }
    impl Config {
        pub fn load() {}
        pub fn load_from_file() {}
    }
    impl Default for Config {
        fn default() -> Self {
            Self { break_ratio: 0.15 }
        }
    }
}
use clap::{Parser, Subcommand};
use config::ConfigArgs;
use traits::Runnable;
use human_panic::setup_panic;
#[clap(
    name = "day.rs",
    version = "0.1.0",
    author = "Varun Latthe (Var1337)",
    about,
    after_help = "Day.rs is a command line tool to help you maximise efficiency around an already packed day. To see what it can do, run `day config`0"
)]
struct Cli {
    #[clap(subcommand)]
    subcmd: SubCommand,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl clap::Parser for Cli {}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped
)]
#[automatically_derived]
impl clap::CommandFactory for Cli {
    fn command<'b>() -> clap::Command {
        let __clap_app = clap::Command::new("day.rs");
        <Self as clap::Args>::augment_args(__clap_app)
    }
    fn command_for_update<'b>() -> clap::Command {
        let __clap_app = clap::Command::new("day.rs");
        <Self as clap::Args>::augment_args_for_update(__clap_app)
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped
)]
#[automatically_derived]
impl clap::FromArgMatches for Cli {
    fn from_arg_matches(
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn from_arg_matches_mut(
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        #![allow(deprecated)]
        let v = Cli {
            subcmd: {
                <SubCommand as clap::FromArgMatches>::from_arg_matches_mut(__clap_arg_matches)?
            },
        };
        ::std::result::Result::Ok(v)
    }
    fn update_from_arg_matches(
        &mut self,
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn update_from_arg_matches_mut(
        &mut self,
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        #![allow(deprecated)]
        {
            #[allow(non_snake_case)]
            let subcmd = &mut self.subcmd;
            <SubCommand as clap::FromArgMatches>::update_from_arg_matches_mut(
                subcmd,
                __clap_arg_matches,
            )?;
        }
        ::std::result::Result::Ok(())
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped
)]
#[automatically_derived]
impl clap::Args for Cli {
    fn group_id() -> Option<clap::Id> {
        Some(clap::Id::from("Cli"))
    }
    fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app.group(clap::ArgGroup::new("Cli").multiple(true).args({
                let members: [clap::Id; 0usize] = [];
                members
            }));
            let __clap_app = <SubCommand as clap::Subcommand>::augment_subcommands(__clap_app);
            let __clap_app = __clap_app
                .subcommand_required(true)
                .arg_required_else_help(true);
            __clap_app.version("0.1.0").author("Varun Latthe (Var1337)").about("A comprehensive CLI time management tool").after_help("Day.rs is a command line tool to help you maximise efficiency around an already packed day. To see what it can do, run `day config`0")
        }
    }
    fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app.group(clap::ArgGroup::new("Cli").multiple(true).args({
                let members: [clap::Id; 0usize] = [];
                members
            }));
            let __clap_app = <SubCommand as clap::Subcommand>::augment_subcommands(__clap_app);
            let __clap_app = __clap_app
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand_required(false)
                .arg_required_else_help(false);
            __clap_app.version("0.1.0").author("Varun Latthe (Var1337)").about("A comprehensive CLI time management tool").after_help("Day.rs is a command line tool to help you maximise efficiency around an already packed day. To see what it can do, run `day config`0")
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Cli {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "Cli", "subcmd", &&self.subcmd)
    }
}
impl Runnable for Cli {
    type Args = ();
    fn run(&self, _args: &Self::Args) -> anyhow::Result<()> {
        match &self.subcmd {
            SubCommand::Config(config_args) => config_args.run(&self),
        }
    }
}
enum SubCommand {
    #[clap(visible_aliases = & ["cfg", "c"])]
    /// Show or change configuration values
    Config(ConfigArgs),
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped
)]
#[automatically_derived]
impl clap::FromArgMatches for SubCommand {
    fn from_arg_matches(
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn from_arg_matches_mut(
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        #![allow(deprecated)]
        if let Some((__clap_name, mut __clap_arg_sub_matches)) =
            __clap_arg_matches.remove_subcommand()
        {
            let __clap_arg_matches = &mut __clap_arg_sub_matches;
            if __clap_name == "config" && !__clap_arg_matches.contains_id("") {
                return ::std::result::Result::Ok(Self::Config(
                    <ConfigArgs as clap::FromArgMatches>::from_arg_matches_mut(__clap_arg_matches)?,
                ));
            }
            ::std::result::Result::Err(clap::Error::raw(
                clap::error::ErrorKind::InvalidSubcommand,
                {
                    let res = ::alloc::fmt::format(format_args!(
                        "The subcommand \'{0}\' wasn\'t recognized",
                        __clap_name
                    ));
                    res
                },
            ))
        } else {
            ::std::result::Result::Err(clap::Error::raw(
                clap::error::ErrorKind::MissingSubcommand,
                "A subcommand is required but one was not provided.",
            ))
        }
    }
    fn update_from_arg_matches(
        &mut self,
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn update_from_arg_matches_mut<'b>(
        &mut self,
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        #![allow(deprecated)]
        if let Some(__clap_name) = __clap_arg_matches.subcommand_name() {
            match self {
                Self::Config(ref mut __clap_arg) if "config" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) =
                        __clap_arg_matches.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    clap::FromArgMatches::update_from_arg_matches_mut(
                        __clap_arg,
                        __clap_arg_matches,
                    )?
                }
                s => {
                    *s = <Self as clap::FromArgMatches>::from_arg_matches_mut(__clap_arg_matches)?;
                }
            }
        }
        ::std::result::Result::Ok(())
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped
)]
#[automatically_derived]
impl clap::Subcommand for SubCommand {
    fn augment_subcommands<'b>(__clap_app: clap::Command) -> clap::Command {
        let __clap_app = __clap_app;
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("config");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = { <ConfigArgs as clap::Args>::augment_args(__clap_subcommand) };
            __clap_subcommand
                .about("Show or change configuration values")
                .long_about(None)
                .visible_aliases(&["cfg", "c"])
        });
        __clap_app
    }
    fn augment_subcommands_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
        let __clap_app = __clap_app;
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("config");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand =
                { <ConfigArgs as clap::Args>::augment_args_for_update(__clap_subcommand) };
            __clap_subcommand
                .about("Show or change configuration values")
                .long_about(None)
                .visible_aliases(&["cfg", "c"])
        });
        __clap_app
    }
    fn has_subcommand(__clap_name: &str) -> bool {
        if "config" == __clap_name {
            return true;
        }
        false
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for SubCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            SubCommand::Config(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Config", &__self_0)
            }
        }
    }
}
fn main() -> anyhow::Result<()> {
    {
        #[allow(unused_imports)]
        use std::panic::{self, PanicInfo};
        #[allow(unused_imports)]
        use ::human_panic::{handle_dump, print_msg, Metadata};
        match ::human_panic::PanicStyle::default() {
            ::human_panic::PanicStyle::Debug => {}
            ::human_panic::PanicStyle::Human => {
                let meta = {
                    ::human_panic::Metadata {
                        version: "0.1.0".into(),
                        name: "day".into(),
                        authors: "Varun Latthe (Var1377)".replace(":", ", ").into(),
                        homepage: "".into(),
                    }
                };
                panic::set_hook(Box::new(move |info: &PanicInfo| {
                    let file_path = handle_dump(&meta, info);
                    print_msg(file_path, &meta)
                        .expect("human-panic: printing error message to console failed");
                }));
            }
        }
    };
    let cli = Cli::parse();
    cli.run(&())
}
