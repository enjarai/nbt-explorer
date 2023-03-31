use serde::{Serialize, ser::SerializeMap, Deserialize, de::Visitor};

use crate::nbt::tag::Tag;

impl Serialize for Tag {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        match self {
            Tag::ByteTag(value) => serialize_simple(s, "byte", value),
            Tag::ShortTag(value) => serialize_simple(s, "short", value),
            Tag::IntTag(value) => serialize_simple(s, "int", value),
            Tag::LongTag(value) => serialize_simple(s, "long", value),
            Tag::FloatTag(value) => serialize_simple(s, "float", value),
            Tag::DoubleTag(value) => serialize_simple(s, "double", value),
            Tag::ByteArray(value) => serialize_simple(s, "byte_array", value),
            Tag::StringTag(value) => serialize_simple(s, "string", value),
            Tag::ListTag(value) => serialize_simple(s, "list", value),
            Tag::CompoundTag(value) => serialize_simple(s, "compound", value),
            Tag::IntArray(value) => serialize_simple(s, "int_array", value),
            Tag::LongArray(value) => serialize_simple(s, "long_array", value),
        }
    }
}

fn serialize_simple<T: Serialize, S: serde::Serializer>(s: S, tag_type: &str, value: &T) -> Result<S::Ok, S::Error> {
    let mut map = s.serialize_map(Some(1))?;
    map.serialize_entry("type", tag_type)?;
    map.serialize_entry("value", value)?;
    map.end()
}

struct TagVisitor;

impl<'a> Deserialize<'a> for Tag {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a> {
        d.deserialize_map(TagVisitor)
    }
}

impl<'a> Visitor<'a> for TagVisitor {
    type Value = Tag;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a tag")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'a> {
        let mut tag_type = None;
        let mut value = None;
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "type" => tag_type = Some(map.next_value()?),
                "value" => value = Some(map.next_value()?),
                _ => return Err(serde::de::Error::unknown_field(key.as_str(), &["type", "value"])),
            }
        }
        let tag_type = tag_type.ok_or_else(|| serde::de::Error::missing_field("type"))?;
        let value = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
        match tag_type.as_str() {
            "byte" => Ok(Tag::ByteTag(value)),
            "short" => Ok(Tag::ShortTag(value)),
            "int" => Ok(Tag::IntTag(value)),
            "long" => Ok(Tag::LongTag(value)),
            "float" => Ok(Tag::FloatTag(value)),
            "double" => Ok(Tag::DoubleTag(value)),
            "byte_array" => Ok(Tag::ByteArray(value)),
            "string" => Ok(Tag::StringTag(value)),
            "list" => Ok(Tag::ListTag(value)),
            "compound" => Ok(Tag::CompoundTag(value)),
            "int_array" => Ok(Tag::IntArray(value)),
            "long_array" => Ok(Tag::LongArray(value)),
            _ => Err(serde::de::Error::unknown_variant(tag_type.as_str(), &["byte", "short", "int", "long", "float", "double", "byte_array", "string", "list", "compound", "int_array", "long_array"])),
        }
    }
}
