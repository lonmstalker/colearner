use teloxide::macros::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rules="lowercase", description="Поддерживаемые команды")]
pub enum Command {
    #[command(description = "Помощь")]
    Help,
    #[command(description = "Старт")]
    Start
}

