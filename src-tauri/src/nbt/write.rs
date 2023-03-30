use std::io::Write;

use byteorder::{BigEndian, WriteBytesExt};

use super::tag::{Tag, TagError};

impl Tag {
    pub fn write<T: Write>(&self, buf: &mut T) -> Result<(), TagError> {
        match self {
            Tag::CompoundTag(value) => {
                for (key, value) in value {
                    value.write_type(buf)?;
                    write_string(buf, key)?;
                    value.write_element(buf)?;
                }
                Ok(())
            }
            _ => Err(TagError::InvalidType)
        }
    }

    fn write_type<T: Write>(&self, buf: &mut T) -> Result<(), TagError> {
        buf.write_u8(self.get_type())?;
        Ok(())
    }

    fn get_type(&self) -> u8 {
        match self {
            Tag::ByteTag(_) => 0x01,
            Tag::ShortTag(_) => 0x02,
            Tag::IntTag(_) => 0x03,
            Tag::LongTag(_) => 0x04,
            Tag::FloatTag(_) => 0x05,
            Tag::DoubleTag(_) => 0x06,
            Tag::ByteArray(_) => 0x07,
            Tag::StringTag(_) => 0x08,
            Tag::ListTag(_) => 0x09,
            Tag::CompoundTag(_) => 0x0A,
            Tag::IntArray(_) => 0x0B,
            Tag::LongArray(_) => 0x0C
        }
    }

    fn write_element<T: Write>(&self, buf: &mut T) -> Result<(), TagError> {
        match self {
            Tag::ByteTag(value) => buf.write_i8(*value)?,
            Tag::ShortTag(value) => buf.write_i16::<BigEndian>(*value)?,
            Tag::IntTag(value) => buf.write_i32::<BigEndian>(*value)?,
            Tag::LongTag(value) => buf.write_i64::<BigEndian>(*value)?,
            Tag::FloatTag(value) => buf.write_f32::<BigEndian>(*value)?,
            Tag::DoubleTag(value) => buf.write_f64::<BigEndian>(*value)?,
            Tag::ByteArray(value) => {
                buf.write_i32::<BigEndian>(value.len() as i32)?;
                for byte in value {
                    buf.write_i8(*byte)?;
                }
            }
            Tag::StringTag(value) => write_string(buf, value)?,
            Tag::ListTag(value) => {
                let type_byte = value.first().map(|t| t.get_type()).unwrap_or(0x00);
                buf.write_u8(type_byte)?;
                buf.write_i32::<BigEndian>(value.len() as i32)?;
                for tag in value {
                    tag.write_element(buf)?;
                }
            }
            Tag::CompoundTag(value) => {
                for (key, value) in value {
                    value.write_type(buf)?;
                    write_string(buf, key)?;
                    value.write_element(buf)?;
                }
                buf.write_u8(0x00)?;
            }
            Tag::IntArray(value) => {
                buf.write_i32::<BigEndian>(value.len() as i32)?;
                for int in value {
                    buf.write_i32::<BigEndian>(*int)?;
                }
            }
            Tag::LongArray(value) => {
                buf.write_i32::<BigEndian>(value.len() as i32)?;
                for long in value {
                    buf.write_i64::<BigEndian>(*long)?;
                }
            }
        }
        Ok(())
    }
}

fn write_string<T: Write>(buf: &mut T, value: &str) -> Result<(), TagError> {
    let value = value.as_bytes();
    buf.write_u16::<BigEndian>(value.len() as u16)?;
    buf.write(value)?;
    Ok(())
}
