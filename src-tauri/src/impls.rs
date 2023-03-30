use serde::{Serialize, ser::{SerializeSeq, SerializeMap}};

use crate::nbt::tag::Tag;

impl Serialize for Tag {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        match self {
            Tag::ByteTag(value) => s.serialize_i8(*value),
            Tag::ShortTag(value) => s.serialize_i16(*value),
            Tag::IntTag(value) => s.serialize_i32(*value),
            Tag::LongTag(value) => s.serialize_i64(*value),
            Tag::FloatTag(value) => s.serialize_f32(*value),
            Tag::DoubleTag(value) => s.serialize_f64(*value),
            Tag::ByteArray(value) => {
                let mut seq = s.serialize_seq(Some(value.len()))?;
                for byte in value {
                    seq.serialize_element(byte)?;
                }
                seq.end()
            },
            Tag::StringTag(value) => s.serialize_str(value),
            Tag::ListTag(value) => {
                let mut seq = s.serialize_seq(Some(value.len()))?;
                for tag in value {
                    seq.serialize_element(tag)?;
                }
                seq.end()
            },
            Tag::CompoundTag(value) => {
                let mut map = s.serialize_map(Some(value.len()))?;
                for (key, value) in value {
                    map.serialize_entry(key, value)?;
                }
                map.end()
            }
            Tag::IntArray(value) => {
                let mut seq = s.serialize_seq(Some(value.len()))?;
                for int in value {
                    seq.serialize_element(int)?;
                }
                seq.end()
            },
            Tag::LongArray(value) => {
                let mut seq = s.serialize_seq(Some(value.len()))?;
                for long in value {
                    seq.serialize_element(long)?;
                }
                seq.end()
            },
        }
    }
}