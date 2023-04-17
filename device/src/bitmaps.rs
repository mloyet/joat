pub const SPADES1: &[u8] = include_bytes!("bitmaps/spades1");
pub const SPADES2: &[u8] = include_bytes!("bitmaps/spades2");
pub const SPADES3: &[u8] = include_bytes!("bitmaps/spades3");
pub const HEARTS1: &[u8] = include_bytes!("bitmaps/hearts1");
pub const HEARTS2: &[u8] = include_bytes!("bitmaps/hearts2");
pub const HEARTS3: &[u8] = include_bytes!("bitmaps/hearts3");
pub const CLUBS1: &[u8] = include_bytes!("bitmaps/clubs1");
pub const CLUBS2: &[u8] = include_bytes!("bitmaps/clubs2");
pub const CLUBS3: &[u8] = include_bytes!("bitmaps/clubs3");
pub const DIAMONDS1: &[u8] = include_bytes!("bitmaps/diamonds1");
pub const DIAMONDS2: &[u8] = include_bytes!("bitmaps/diamonds2");
pub const DIAMONDS3: &[u8] = include_bytes!("bitmaps/diamonds3");

pub const KING: &[u8] = include_bytes!("bitmaps/KING");
pub const QUEEN: &[u8] = include_bytes!("bitmaps/QUEEN");
pub const JACK: &[u8] = include_bytes!("bitmaps/JACK");

pub const SPADES: &[&[u8]] = &[&SPADES1, &SPADES2, &SPADES3];
pub const HEARTS: &[&[u8]] = &[&HEARTS1, &HEARTS2, &HEARTS3];
pub const CLUBS: &[&[u8]] = &[&CLUBS1, &CLUBS2, &CLUBS3];
pub const DIAMONDS: &[&[u8]] = &[&DIAMONDS1, &DIAMONDS2, &DIAMONDS3];
