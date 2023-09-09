use logger::{error, info, warn};
use crate::slashcmds;

use serenity::{
    builder::CreateApplicationCommand, client::Context, framework::standard::CommandResult,
    model::application::command::Command, model::application::command::CommandOptionType,
    model::application::command::CommandType,
    model::application::interaction::application_command::ApplicationCommandInteraction,
};


pub struct CommandManager {
    commands_registered: bool,
    commands: Vec<CreateApplicationCommand>,
}

impl CommandManager {
    pub fn new() -> Self {
        CommandManager {
            commands_registered: false,
            commands: CommandManager::build_commands(),
        }
    }

    pub async fn on_command(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction,
    ) -> CommandResult {
        let command_name = command.data.name.to_lowercase();

        match command_name.as_str() {
            "about" => slashcmds::about::about(ctx, command).await,
            "listening" => slashcmds::spotify::listening::listening(ctx, command).await,
            "track" => slashcmds::spotify::track::track(ctx, command).await,
            "invite" => slashcmds::invite::invite(ctx, command).await,
            "cover" => slashcmds::spotify::cover::cover(ctx, command).await,

            e => {
                warn!("Unknown application command received: {}", e);
                Ok(())
            }
        }
    }


    //
    // pub async fn register_commands_guild(&mut self, ctx: &Context, guild: &Guild) {
    //     match guild
    //         .set_application_commands(&ctx.http, |setter| {
    //             setter.set_application_commands(self.commands.clone())
    //         })
    //         .await
    //     {
    //         Err(e) => error!(
    //             "Unable to set application commands for guild '{}': {}",
    //             guild.id, e
    //         ),
    //         Ok(commands) => info!(
    //             "Registered {} commands in guild: {}",
    //             commands.len(),
    //             guild.id
    //         ),
    //     }
    // }

    pub async fn register_commands_global(&mut self, ctx: &Context) {
        if self.commands_registered || cfg!(debug_assertions){
            return;
        }
        self.commands_registered = true;


        // let commands = Command::get_global_application_commands(&ctx.http).await.unwrap();
        //
        // for i in commands.iter() {
        //     match Command::delete_global_application_command(&ctx.http, i.id).await {
        //         Ok(result) => {println!("success: {:?}", result)}
        //         Err(e) => {println!("fail: {}", e)}
        //     }
        // }


        match Command::set_global_application_commands(&ctx.http, |setter| {
            setter.set_application_commands(self.commands.clone())
        })
        .await
        {
            Ok(cmds) => info!("Registered {} application commands", cmds.len()),
            Err(e) => error!("Unable to set application commands: {}", e),
        }
    }

    pub fn build_commands() -> Vec<CreateApplicationCommand> {
        let mut cmds = Vec::new();
        let mut cmd = CreateApplicationCommand::default();

        cmd.kind(CommandType::ChatInput)
            .name("about")
            .description("Information of this Bot");
        cmds.push(cmd);

        cmd = CreateApplicationCommand::default();
        cmd.kind(CommandType::ChatInput)
            .name("invite")
            .description("Grab my invite link to invite me to your server");
        cmds.push(cmd);

        cmd = CreateApplicationCommand::default();
        cmd.kind(CommandType::ChatInput)
            .name("listening")
            .description("show the Track info that the user is listening to")
            .create_option(|o| {
                o.name("user")
                    .kind(CommandOptionType::User)
                    .description("a user")
                    .required(false)
            });
        cmds.push(cmd);

        cmd = CreateApplicationCommand::default();
        cmd.kind(CommandType::ChatInput)
            .name("track")
            .description("show the Track url that the user is listening to")
            .create_option(|o| {
                o.name("user")
                    .kind(CommandOptionType::User)
                    .description("a user")
                    .required(false)
            });
        cmds.push(cmd);

        cmd = CreateApplicationCommand::default();
        cmd.kind(CommandType::ChatInput)
            .name("cover")
            .description("show the Track cover that the user is listening to")
            .create_option(|o| {
                o.name("user")
                    .kind(CommandOptionType::User)
                    .description("a user")
                    .required(false)
            });
        cmds.push(cmd);

        cmds
    }
}