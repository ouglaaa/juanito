extern crate chrono;
extern crate time;

use std::fs::File;
use std::io::{Error, Write};
use std::io::prelude::*;

use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json;
use serenity::{
    Client,
    model::prelude::*,
    prelude::*,
};

mod src;

mod data {
    pub const TIME_SINCE_MINE: i64 = 10;
    //60 * 20; // 20min
    pub const DIVISOR_MINE: f64 = 250.0;
    pub const DIVISOR_MINER: f64 = 300.0;
    pub const DIVISOR_ENGAGE: f64 = 200.0;
}

struct Handler {
    prefix: &'static str,
}

trait LoggingChannelDiscovery {
    fn get_logging_channel(&self, ctx: &Context, guildId: u64) -> Option<GuildChannel>;
    fn post_comes_from_raid_helper(&self, userId: UserId) -> bool;
}

impl LoggingChannelDiscovery for Handler {
    fn get_logging_channel(&self, ctx: &Context, guildId: u64) -> Option<GuildChannel> {
        ctx.http
            .get_channels(guildId)
            .unwrap()
            .into_iter()
            .find(|chan| chan.name == "logging" && chan.kind == ChannelType::Text)
    }


    fn post_comes_from_raid_helper(&self, userId: UserId) -> bool {
        userId.0 == 579155972115660803
    }
}

impl EventHandler for Handler {
    // User leaves the guild
    fn guild_member_removal(&self, _ctx: Context, _guild: GuildId, _user: serenity::model::user::User, _member_data_if_available: Option<Member>) {
        if let log = self.get_logging_channel(&_ctx, _guild.0).unwrap() {
            if let member_data = _member_data_if_available.unwrap() {
                log.send_message(&_ctx.http,
                                 |m|
                                     m.content(format!(":arrow_right::door: <@{}> ({}) has left the server.\r\n[{}]", _user.name, member_data.nick.unwrap(), Local::now())));
            }
        }
    }

    fn guild_member_addition(&self, _ctx: Context, _guild_id: GuildId, _new_member: Member) {
        if let log = self.get_logging_channel(&_ctx, _guild_id.0).unwrap() {
                log.send_message(&_ctx.http,
                                 |m|
                                     m.content(format!("* <@{}> has join the server.\r\n[{}]", _new_member.nick.unwrap() , Local::now())));
            }
    }


    /// Dispatched when a reaction is detached from a message.
    /// raid help id 579155972115660803
    /// Provides the reaction's data.
    fn reaction_add(&self, _ctx: Context, _added_reaction: Reaction) {
        if let userId = _added_reaction.user_id.0 {
            if let messageReactedResult = _ctx.http.get_message(_added_reaction.channel_id.0, _added_reaction.message_id.0) {
                match messageReactedResult {
                    Ok(Message) => {
                        if self.post_comes_from_raid_helper(Message.author.id) {
                            if let guildId = _added_reaction.guild_id {
                                if let log = self.get_logging_channel(&_ctx, guildId.unwrap().0) {
                                    let chan = log.unwrap();
                                    chan.send_message(&_ctx.http,
                                                      |m| {
                                                          m.content(format!("<<#{}>> <@{}> [{}] Added [{}]", _added_reaction.channel_id.0, userId, _added_reaction.emoji, Local::now()))
                                                      });
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    /// Dispatched when a reaction is detached from a message.
    ///
    /// Provides the reaction's data.
    fn reaction_remove(&self, _ctx: Context, _removed_reaction: Reaction) {
        if let userId = _removed_reaction.user_id.0 {
            if let messageReactedResult = _ctx.http.get_message(_removed_reaction.channel_id.0, _removed_reaction.message_id.0) {
                match messageReactedResult {
                    Ok(Message) => {
                        if self.post_comes_from_raid_helper(Message.author.id) {
                            if let guildId = _removed_reaction.guild_id {
                                if let log = self.get_logging_channel(&_ctx, guildId.unwrap().0) {
                                    let chan = log.unwrap();
                                    chan.send_message(&_ctx.http,
                                                      |m| {
                                                          m.content(format!("<<#{}>> <@{}> [{}] Removed [{}]",  _removed_reaction.channel_id.0, userId, _removed_reaction.emoji, Local::now(),))
                                                      });
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}


fn main() {
    let config: src::config::Config = src::config::Config::new();

    let handler: Handler = Handler { prefix: config.prefix() };
    let mut client = Client::new(config.token(), handler).expect("Could not creat the client");
    if let Err(why) = client.start() {
        println!("Error on starting the client: {}", why);
        std::process::exit(1);
    }
}