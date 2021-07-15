pub const GUILDS_PATH: &str = "guilds";
pub const DIRECTORIES: [&str; 1] = ["guilds"];
pub const VOICE_ONLY_CHANNEL: &str = "voice-only";
//pub const GENERAL_CATEGORY: &str = "general";

#[derive(Eq, Hash)]
pub enum MemberKeys {
    Name,
    Id,
    CurrXp,
    TotalXp,
    Level,
    RoleName,
    MemesSent,
    MessagesSent,
    Discriminator
}

impl PartialEq for MemberKeys {
    fn eq(&self, other: &MemberKeys) -> bool {
        return std::mem::discriminant(&self) == std::mem::discriminant(&other);
    }
}
