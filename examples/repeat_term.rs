use bevy::{log::LogPlugin, prelude::*};
use bevy_commands::{AddAppCommand, AppCommand, CommandsStdioPlugins, QueuedCommands, StdioPrompt};

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, LogPlugin::default()))
        .add_plugins(CommandsStdioPlugins)
        .insert_resource(StdioPrompt("> ".into()))
        .add_app_command::<RepeatCommand, _>(repeat_command)
        .add_systems(Startup, setup)
        .run();
}

/// Prints the provided message back to the sender <COUNT> times
#[derive(clap::Parser, AppCommand)]
#[command(name = "repeat")]
struct RepeatCommand {
    /// The message to echo back
    message: String,
    /// The amount of times to echo the message back
    #[arg(short, long, default_value_t = 1)]
    count: usize,
}

fn repeat_command(mut queue: QueuedCommands<RepeatCommand>) {
    queue.consume(|mut ctx| {
        for _ in 0..ctx.data.count {
            ctx.ok(ctx.data.message.clone());
        }
    })
}

fn setup() {
    info!("Type `repeat <message>` to repeat a message back to the console");
    info!("Or type `help` to see all registered commands");
}
