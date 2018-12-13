#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Scope {
    //for oauth2 bots, this puts the bot in the user's selected guild by default
    Bot,
    //allows /users/@me/connections to return linked third-party accounts - https://discordapp.com/developers/docs/resources/user#get-user-connections
    Connections,
    //enables /users/@me to return an email - https://discordapp.com/developers/docs/resources/user#get-current-user
    Email,
    //allows /users/@me without email - https://discordapp.com/developers/docs/resources/user#get-current-user
    Identity,
    //allows /users/@me/guilds to return basic information about all of a user's guilds - https://discordapp.com/developers/docs/resources/user#get-current-user-guilds
    Guilds,
    //allows /guilds/{guild.id}/members/{user.id} to be used for joining users to a guild - https://discordapp.com/developers/docs/resources/guild#add-guild-member
    JoinGuilds,
    //allows your app to join users to a group dm - https://discordapp.com/developers/docs/resources/channel#group-dm-add-recipient
    JoinGroupDM,
}


impl From<Scope> for String {
    fn from(scope: Scope) -> Self {
        match scope {
            Scope::Bot => "bot",
            Scope::Connections => "connections",
            Scope::Email => "email",
            Scope::Identity => "identify",
            Scope::Guilds => "guilds",
            Scope::JoinGuilds => "guilds.join",
            Scope::JoinGroupDM => "gdm.join",
        }.to_string()
    }
}

use std::fmt;

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Into::<String>::into(*self))
    }
}

impl fmt::Debug for Scope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Scope: {}", Into::<String>::into(*self))
    }
}
