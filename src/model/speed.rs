use std::{ops::AddAssign, str::FromStr};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize, Serialize, Ord, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub enum Speed {
    UltraBullet,
    Bullet,
    Blitz,
    Rapid,
    Classical,
    Correspondence,
}

impl Speed {
    pub const ALL: [Speed; 6] = [
        Speed::UltraBullet,
        Speed::Bullet,
        Speed::Blitz,
        Speed::Rapid,
        Speed::Classical,
        Speed::Correspondence,
    ];
}

impl FromStr for Speed {
    type Err = InvalidSpeed;

    fn from_str(s: &str) -> Result<Speed, InvalidSpeed> {
        Ok(match s {
            "ultraBullet" => Speed::UltraBullet,
            "bullet" => Speed::Bullet,
            "blitz" => Speed::Blitz,
            "rapid" => Speed::Rapid,
            "classical" => Speed::Classical,
            "correspondence" => Speed::Correspondence,
            _ => return Err(InvalidSpeed),
        })
    }
}

#[derive(Error, Debug)]
#[error("invalid speed")]
pub struct InvalidSpeed;

#[derive(Debug, Default)]
pub struct BySpeed<T> {
    pub ultra_bullet: T,
    pub bullet: T,
    pub blitz: T,
    pub rapid: T,
    pub classical: T,
    pub correspondence: T,
}

impl<T> BySpeed<T> {
    pub fn by_speed(&self, speed: Speed) -> &T {
        match speed {
            Speed::UltraBullet => &self.ultra_bullet,
            Speed::Bullet => &self.bullet,
            Speed::Blitz => &self.blitz,
            Speed::Rapid => &self.rapid,
            Speed::Classical => &self.classical,
            Speed::Correspondence => &self.correspondence,
        }
    }

    pub fn by_speed_mut(&mut self, speed: Speed) -> &mut T {
        match speed {
            Speed::UltraBullet => &mut self.ultra_bullet,
            Speed::Bullet => &mut self.bullet,
            Speed::Blitz => &mut self.blitz,
            Speed::Rapid => &mut self.rapid,
            Speed::Classical => &mut self.classical,
            Speed::Correspondence => &mut self.correspondence,
        }
    }

    pub fn as_ref(&self) -> BySpeed<&T> {
        BySpeed {
            ultra_bullet: &self.ultra_bullet,
            bullet: &self.bullet,
            blitz: &self.blitz,
            rapid: &self.rapid,
            classical: &self.classical,
            correspondence: &self.correspondence,
        }
    }

    pub fn try_map<U, E, F>(self, mut f: F) -> Result<BySpeed<U>, E>
    where
        F: FnMut(Speed, T) -> Result<U, E>,
    {
        Ok(BySpeed {
            ultra_bullet: f(Speed::UltraBullet, self.ultra_bullet)?,
            bullet: f(Speed::Bullet, self.bullet)?,
            blitz: f(Speed::Blitz, self.blitz)?,
            rapid: f(Speed::Rapid, self.rapid)?,
            classical: f(Speed::Classical, self.classical)?,
            correspondence: f(Speed::Correspondence, self.correspondence)?,
        })
    }
}

impl<T: AddAssign> AddAssign for BySpeed<T> {
    fn add_assign(&mut self, rhs: BySpeed<T>) {
        self.ultra_bullet += rhs.ultra_bullet;
        self.bullet += rhs.bullet;
        self.blitz += rhs.blitz;
        self.rapid += rhs.rapid;
        self.classical += rhs.classical;
        self.correspondence += rhs.correspondence;
    }
}
