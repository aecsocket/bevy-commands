use bevy::{log::LogPlugin, prelude::*};
use bevy_commander::{prelude::*, readline::*};

pub enum Sender {
    Console,
}

impl CommandSender for Sender {
    fn send_all<'a>(&self, lines: impl IntoIterator<Item = &'a str>) {
        for line in lines {
            info!("{}", line);
        }
    }
}

impl ConsoleCommandSender for Sender {
    fn console() -> Self {
        Self::Console
    }
}

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, LogPlugin::default()))
        .add_plugins((
            CommanderPlugin::<Sender>::new(),
            CommanderReadlinePlugin::<Sender> {
                prompt: "> ".into(),
                ..default()
            },
        ))
        .add_command::<EchoCommand, _>(echo_command)
        .add_systems(Startup, setup)
        .run();
}

#[derive(clap::Parser, Resource)]
struct EchoCommand {
    message: String,
}

impl AppCommand for EchoCommand {
    fn name() -> &'static str {
        "echo"
    }
}

fn echo_command(ctx: CommandContext<EchoCommand, Sender>) {
    for (cmd, sender) in ctx {
        sender.send(&cmd.message);
    }
}

fn setup() {
    info!("Type `echo <message>` to echo a message back to the console");
}