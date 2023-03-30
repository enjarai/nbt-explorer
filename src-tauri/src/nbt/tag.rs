use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum TagError {
    InvalidType,
    InvalidUtf8(FromUtf8Error),
    IoError(io::Error),
}

impl From<io::Error> for TagError {
    fn from(err: io::Error) -> TagError {
        TagError::IoError(err)
    }
}

impl From<FromUtf8Error> for TagError {
    fn from(err: FromUtf8Error) -> TagError {
        TagError::InvalidUtf8(err)
    }
}

#[derive(Debug)]
pub enum Tag {
    ByteTag(i8),
    ShortTag(i16),
    IntTag(i32),
    LongTag(i64),
    FloatTag(f32),
    DoubleTag(f64),
    ByteArray(Vec<i8>),
    StringTag(String),
    ListTag(Vec<Tag>),
    CompoundTag(HashMap<String, Tag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tag::ByteTag(value) => Display::fmt(value, f),
            Tag::ShortTag(value) => Display::fmt(value, f),
            Tag::IntTag(value) => Display::fmt(value, f),
            Tag::LongTag(value) => Display::fmt(value, f),
            Tag::FloatTag(value) => Display::fmt(value, f),
            Tag::DoubleTag(value) => Display::fmt(value, f),
            Tag::ByteArray(value) => list_fmt(value, f),
            Tag::StringTag(value) => Display::fmt(value, f),
            Tag::ListTag(value) => list_fmt(value, f),
            Tag::CompoundTag(value) => {
                write!(f, "{{")?;
                for (i, (key, tag)) in value.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", key, tag)?;
                }
                write!(f, "}}")
            }
            Tag::IntArray(value) => list_fmt(value, f),
            Tag::LongArray(value) => list_fmt(value, f)
        }
    }
}

fn list_fmt<T: Display>(value: &Vec<T>, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[")?;
    for (i, t) in value.iter().enumerate() {
        if i != 0 {
            write!(f, ", ")?;
        }
        write!(f, "{}", t)?;
    }
    write!(f, "]")
}

impl Tag {
    pub fn count_elements(&self, counter: &mut usize) {
        match self {
            Tag::ByteTag(_) => *counter += 1,
            Tag::ShortTag(_) => *counter += 1,
            Tag::IntTag(_) => *counter += 1,
            Tag::LongTag(_) => *counter += 1,
            Tag::FloatTag(_) => *counter += 1,
            Tag::DoubleTag(_) => *counter += 1,
            Tag::ByteArray(value) => *counter += value.len(),
            Tag::StringTag(_) => *counter += 1,
            Tag::ListTag(value) => {
                for tag in value {
                    tag.count_elements(counter);
                }
            }
            Tag::CompoundTag(value) => {
                for tag in value.values() {
                    tag.count_elements(counter);
                }
            }
            Tag::IntArray(value) => *counter += value.len(),
            Tag::LongArray(value) => *counter += value.len()
        }
    }

    pub fn byte(&self) -> Option<&i8> {
        match self {
            Tag::ByteTag(b) => Some(b),
            _ => None
        }
    }

    pub fn short(&self) -> Option<&i16> {
        match self {
            Tag::ShortTag(s) => Some(s),
            _ => None
        }
    }

    pub fn int(&self) -> Option<&i32> {
        match self {
            Tag::IntTag(i) => Some(i),
            _ => None
        }
    }

    pub fn long(&self) -> Option<&i64> {
        match self {
            Tag::LongTag(l) => Some(l),
            _ => None
        }
    }

    pub fn float(&self) -> Option<&f32> {
        match self {
            Tag::FloatTag(f) => Some(f),
            _ => None
        }
    }

    pub fn double(&self) -> Option<&f64> {
        match self {
            Tag::DoubleTag(d) => Some(d),
            _ => None
        }
    }

    pub fn byte_array(&self) -> Option<&Vec<i8>> {
        match self {
            Tag::ByteArray(b) => Some(b),
            _ => None
        }
    }

    pub fn string(&self) -> Option<&String> {
        match self {
            Tag::StringTag(s) => Some(s),
            _ => None
        }
    }

    pub fn list(&self) -> Option<&Vec<Tag>> {
        match self {
            Tag::ListTag(l) => Some(l),
            _ => None
        }
    }

    pub fn compound(&self) -> Option<&HashMap<String, Tag>> {
        match self {
            Tag::CompoundTag(c) => Some(c),
            _ => None
        }
    }

    pub fn int_array(&self) -> Option<&Vec<i32>> {
        match self {
            Tag::IntArray(i) => Some(i),
            _ => None
        }
    }

    pub fn long_array(&self) -> Option<&Vec<i64>> {
        match self {
            Tag::LongArray(l) => Some(l),
            _ => None
        }
    }

    pub fn byte_mut(&mut self) -> Option<&mut i8> {
        match self {
            Tag::ByteTag(b) => Some(b),
            _ => None
        }
    }

    pub fn short_mut(&mut self) -> Option<&mut i16> {
        match self {
            Tag::ShortTag(s) => Some(s),
            _ => None
        }
    }

    pub fn int_mut(&mut self) -> Option<&mut i32> {
        match self {
            Tag::IntTag(i) => Some(i),
            _ => None
        }
    }

    pub fn long_mut(&mut self) -> Option<&mut i64> {
        match self {
            Tag::LongTag(l) => Some(l),
            _ => None
        }
    }

    pub fn float_mut(&mut self) -> Option<&mut f32> {
        match self {
            Tag::FloatTag(f) => Some(f),
            _ => None
        }
    }

    pub fn double_mut(&mut self) -> Option<&mut f64> {
        match self {
            Tag::DoubleTag(d) => Some(d),
            _ => None
        }
    }

    pub fn byte_array_mut(&mut self) -> Option<&mut Vec<i8>> {
        match self {
            Tag::ByteArray(b) => Some(b),
            _ => None
        }
    }

    pub fn string_mut(&mut self) -> Option<&mut String> {
        match self {
            Tag::StringTag(s) => Some(s),
            _ => None
        }
    }

    pub fn list_mut(&mut self) -> Option<&mut Vec<Tag>> {
        match self {
            Tag::ListTag(l) => Some(l),
            _ => None
        }
    }

    pub fn compound_mut(&mut self) -> Option<&mut HashMap<String, Tag>> {
        match self {
            Tag::CompoundTag(c) => Some(c),
            _ => None
        }
    }

    pub fn int_array_mut(&mut self) -> Option<&mut Vec<i32>> {
        match self {
            Tag::IntArray(i) => Some(i),
            _ => None
        }
    }

    pub fn long_array_mut(&mut self) -> Option<&mut Vec<i64>> {
        match self {
            Tag::LongArray(l) => Some(l),
            _ => None
        }
    }
}
