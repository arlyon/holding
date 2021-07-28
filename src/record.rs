use std::fmt::Display;

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use holding_kronos::{calendar::Calendar, datetime::DateTime, datetime::RawDateTime};

use crate::character::{CharacterReference, LocationReference};

#[derive(Serialize, Deserialize, Clone)]
pub struct RawRecord {
    pub id: Uuid,
    pub note: String,
    pub date: RawDateTime,
    pub chars: Vec<CharacterReference>,
    pub locs: Vec<LocationReference>,
}

impl RawRecord {
    pub fn into_record(self, cal: &Calendar) -> Record {
        Record {
            id: self.id,
            note: self.note,
            date: self.date.into_datetime(cal),
        }
    }

    pub fn new(
        date: RawDateTime,
        note: String,
        chars: Vec<CharacterReference>,
        locs: Vec<LocationReference>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            note,
            date,
            chars,
            locs,
        }
    }

    pub fn pretty(&self) -> String {
        let mut out = String::new();
        let mut curr = 0;

        let chars = self.chars.iter().map(|r| (r.start, r.end, 33));
        let locs = self.locs.iter().map(|r| (r.start, r.end, 31));

        for (start, end, color) in chars
            .chain(locs)
            .sorted_by(|(s1, _, _), (s2, _, _)| s1.cmp(s2))
        {
            out.push_str(&self.note[curr..start]);
            out.push_str(&format!("\x1b[0;{}m", color));
            out.push_str(&self.note[start..end]);
            out.push_str("\x1b[0m");
            curr = end;
        }

        out.push_str(&self.note[curr..]);
        out
    }
}

impl Display for RawRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.note)
    }
}

impl From<&Record<'_>> for RawRecord {
    fn from(val: &Record<'_>) -> Self {
        RawRecord {
            id: val.id,
            note: val.note.clone(),
            date: val.date.into(),
            chars: vec![],
            locs: vec![],
        }
    }
}

#[derive(Clone)]
pub struct Record<'a> {
    pub id: Uuid,
    pub note: String,
    pub date: DateTime<'a>,
}

impl<'a> Record<'a> {
    pub fn new(date: DateTime<'a>, note: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            note,
            date,
        }
    }
}
