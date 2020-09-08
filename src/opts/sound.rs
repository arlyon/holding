use rspotify::blocking::client::Spotify;
use rspotify::blocking::oauth2::SpotifyClientCredentials;
use rspotify::blocking::oauth2::SpotifyOAuth;
use rspotify::blocking::util::get_token;
use std::fmt::Display;

use anyhow::{anyhow, Context, Result};
use clap::Clap;
use strum_macros::Display;

/// Play a number of ambient sounds through spotify.
#[derive(Clap, Eq, PartialEq)]
pub enum Sound {
    Ambient(Ambient),
    Atmosphere(Atmosphere),
    Combat(Combat),
    Mood(Mood),
    Stop,
    Auto,
}

impl Display for Sound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sound::Ambient(a) => a.fmt(f),
            Sound::Atmosphere(a) => a.fmt(f),
            Sound::Combat(a) => a.fmt(f),
            Sound::Mood(a) => a.fmt(f),
            Sound::Auto => write!(f, "auto"),
            Sound::Stop => write!(f, "stop"),
        }
    }
}

#[derive(Clap, Display, Eq, PartialEq)]
pub enum Ambient {
    #[strum(serialize = "7cgECSzxFYwjHugNdbur1O")]
    Cavern,
    #[strum(serialize = "5ayvxbK8CveLIj4llcibs2")]
    Forest,
    #[strum(serialize = "4y88W8yD8M32PJ4ZNJVzAp")]
    MountainPass,
    #[strum(serialize = "47JbzbE2fpng1VU0VeufGU")]
    Mystical,
    #[strum(serialize = "0czhzWKJ1qoC9iHH5yN93a")]
    Ocean,
    #[strum(serialize = "3lQ1VrIoMDHJmw52N3uAEc")]
    Storm,
}

#[derive(Clap, Display, Eq, PartialEq)]
pub enum Atmosphere {
    #[strum(serialize = "2t5TWAPs6HYuJ3xbpjHYpx")]
    TheCapital,
    #[strum(serialize = "0IyMP3izyM2jbYgJLydB00")]
    TheCathedral,
    #[strum(serialize = "4yguXksZpqOW10hpuDyB5A")]
    TheDesert,
    #[strum(serialize = "64UCYVCIPtZiOP2zEodORk")]
    TheDungeon,
    #[strum(serialize = "4jPscCOA5zrheXibHnmlU1")]
    TheFey,
    #[strum(serialize = "6QzZjlzHxNUo9N6E19RKpJ")]
    TheManor,
    #[strum(serialize = "0gZQWj0PjC6t2bgmroHaaW")]
    TheRoad,
    #[strum(serialize = "73YmiE2tLNG5VbNF7oGmSn")]
    TheSaloon,
    #[strum(serialize = "2xA9EIpuBH5DbmGHszQtvk")]
    TheSwamp,
    #[strum(serialize = "2StSwZk9mV2DNO3aucMZYx")]
    TheTavern,
    #[strum(serialize = "5GgU8cLccECwAvjDCGhYjj")]
    TheTown,
    #[strum(serialize = "5Qhtamj9NCxluijLnQ4edN")]
    TheUnderdark,
    #[strum(serialize = "5r2AkNQOITXRqVWqYj40QG")]
    TheWild,
}

#[derive(Clap, Display, Eq, PartialEq)]
pub enum Combat {
    #[strum(serialize = "0Q6hJZYIEu3LwbyBBHjjHo")]
    Boss,
    #[strum(serialize = "5g9ZZ9Ogml8NsjOlv8N31t")]
    Duel,
    #[strum(serialize = "4Anyq806DQpd7pRZbSADUr")]
    Epic,
    #[strum(serialize = "1SbeUQZbRHyUEIr6wsoD4q")]
    Horrifying,
    #[strum(serialize = "0bWUBjlr7O4troJKyyMVbD")]
    Standard,
    #[strum(serialize = "6T0UOAmlbWb29y2fIETtL2")]
    Tough,
}

#[derive(Clap, Display, Eq, PartialEq)]
pub enum Mood {
    #[strum(serialize = "6nSstCQcmzcEUSx8gBrcek")]
    Creepy,
    #[strum(serialize = "71AETM4dyul7BDNYE9zVBv")]
    Denouement,
    #[strum(serialize = "6KbY8nK4vdGO0zaSuoXEFr")]
    Joyful,
    #[strum(serialize = "28ICiQDK37yaahRZD7aX3J")]
    Mysterious,
    #[strum(serialize = "71yNeiFbb8bDhgLIzu9eae")]
    Ominous,
    #[strum(serialize = "3O4DGo9DS5kzUUJo6EQYdp")]
    Pleasant,
    #[strum(serialize = "3VepfFpcPxHIL7WyKYFdGI")]
    Ridiculous,
    #[strum(serialize = "3LNrO4Jvwtzk2QD1gR8ccZ")]
    Serious,
    #[strum(serialize = "5N5w6WFXigWqZMLzVo6rdh")]
    Sombre,
    #[strum(serialize = "4DYALPIektzP4vVdZFlHNe")]
    Tense,
    #[strum(serialize = "1ALzSDT8MfYQ7Xams9Nx16")]
    Triumphant,
}

impl Sound {
    pub fn run(&self) -> Result<()> {
        let playlist = if *self != Sound::Auto {
            self.to_string()
        } else {
            todo!()
        };

        let mut oauth = SpotifyOAuth::default()
            .scope("user-modify-playback-state")
            .build();

        let token_info = get_token(&mut oauth).ok_or_else(|| anyhow!("Could not get token."))?;

        let client_credential = SpotifyClientCredentials::default()
            .token_info(token_info)
            .build();

        let spotify = Spotify::default()
            .client_credentials_manager(client_credential)
            .build();

        if let Sound::Stop = self {
            spotify
                .pause_playback(None)
                .context("could not stop sound")?;
        } else {
            let playlist = spotify
                .playlist(&playlist, None, None)
                .map_err(|e| anyhow!(e))
                .context("could not load playlist")?;

            spotify
                .start_playback(None, Some(playlist.uri), None, None, None)
                .map_err(|e| anyhow!(e))
                .context("could not play track")?;

            println!("Playing {}", playlist.name);
        }

        Ok(())
    }
}
