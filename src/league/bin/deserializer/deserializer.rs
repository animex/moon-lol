use std::collections::HashMap;

use bevy::math::Mat4;
use serde::{
    de::{self, Visitor},
    Deserialize,
};

use crate::league::{
    BinDeserializerError, BinDeserializerResult, BinType, EnumReader, HashMapReader, LeagueLoader,
    MapReader, SeqReader,
};

pub struct BinDeserializer<'de> {
    pub parser: BinParser<'de>,
    pub value_type: BinType,
}

impl<'de> BinDeserializer<'de> {
    pub fn from_bytes(input: &'de [u8], value_type: BinType) -> Self {
        BinDeserializer {
            parser: BinParser::from_bytes(input),
            value_type,
        }
    }

    pub fn from_parser(parser: BinParser<'de>, value_type: BinType) -> Self {
        BinDeserializer { parser, value_type }
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut BinDeserializer<'de> {
    type Error = BinDeserializerError;

    fn deserialize_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        struct_fields: &'static [&'static str],
        visitor: V,
    ) -> BinDeserializerResult<V::Value> {
        let field_count = match self.value_type {
            BinType::Entry => {
                u16::from_le_bytes(self.parser.read_bytes(2)?.try_into().unwrap()) as usize
            }
            BinType::Struct | BinType::Embed => {
                let _hash = u32::from_le_bytes(self.parser.read_bytes(4)?.try_into().unwrap());

                let _fields_len = self.parser.read_bytes(4)?;

                u16::from_le_bytes(self.parser.read_bytes(2)?.try_into().unwrap()) as usize
            }
            _ => panic!("解析结构体的数据类型不正确"),
        };

        println!(
            "🚀 获取结构体: {} 个字段，类型为: {:?}",
            field_count, self.value_type
        );

        let mut data_map: HashMap<u32, (BinType, &'de [u8])> = HashMap::with_capacity(field_count);

        let fields_block_all = self.parser.input;

        let mut temp_parser = BinParser::from_bytes(fields_block_all);

        for i in 0..field_count {
            let hash = u32::from_le_bytes(temp_parser.read_bytes(4)?.try_into().unwrap());
            println!("获取映射信息: 第 {} 个 hash 为 {:x}", i, hash);

            let vtype = temp_parser.read_bintype()?;

            let value_start_offset = fields_block_all.len() - temp_parser.input.len();

            let before_len = temp_parser.input.len();

            temp_parser.skip_value(vtype)?;

            println!(
                "尝试跳过类型: {:?}，总计: {}，剩余：{}",
                vtype,
                before_len - temp_parser.input.len(),
                temp_parser.input.len()
            );

            let value_end_offset = fields_block_all.len() - temp_parser.input.len();

            let value_slice = &fields_block_all[value_start_offset..value_end_offset];

            data_map.insert(hash, (vtype, value_slice));
        }

        self.parser.input = temp_parser.input;

        visitor.visit_map(MapReader {
            data_map,
            struct_fields: struct_fields.iter(),
            next_value: None,
        })
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let len = u16::from_le_bytes(self.parser.read_bytes(2)?.try_into().unwrap()) as usize;

        let s = std::str::from_utf8(self.parser.read_bytes(len)?)
            .map_err(|e| BinDeserializerError::Message(e.to_string()))?;
        println!("😫 解析字符串：{}", s);

        visitor.visit_string(s.to_owned())
    }

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> BinDeserializerResult<V::Value> {
        println!("💎 尝试反序列化 any: {:?}", self.value_type);

        match self.value_type {
            BinType::None => {
                // 根据 skip_value 的逻辑，None 类型后面似乎跟着 6 个字节
                self.parser.read_bytes(6)?;
                visitor.visit_unit()
            }
            BinType::Bool => visitor.visit_bool(self.parser.read_bytes(1)?[0] != 0),
            BinType::S8 => visitor.visit_i8(i8::from_le_bytes(
                self.parser.read_bytes(1)?.try_into().unwrap(),
            )),
            BinType::U8 => visitor.visit_u8(u8::from_le_bytes(
                self.parser.read_bytes(1)?.try_into().unwrap(),
            )),
            BinType::S16 => visitor.visit_i16(i16::from_le_bytes(
                self.parser.read_bytes(2)?.try_into().unwrap(),
            )),
            BinType::U16 => visitor.visit_u16(u16::from_le_bytes(
                self.parser.read_bytes(2)?.try_into().unwrap(),
            )),
            BinType::S32 => visitor.visit_i32(i32::from_le_bytes(
                self.parser.read_bytes(4)?.try_into().unwrap(),
            )),
            BinType::U32 => visitor.visit_u32(u32::from_le_bytes(
                self.parser.read_bytes(4)?.try_into().unwrap(),
            )),
            BinType::S64 => visitor.visit_i64(i64::from_le_bytes(
                self.parser.read_bytes(8)?.try_into().unwrap(),
            )),
            BinType::U64 => visitor.visit_u64(u64::from_le_bytes(
                self.parser.read_bytes(8)?.try_into().unwrap(),
            )),
            BinType::Float => visitor.visit_f32(f32::from_le_bytes(
                self.parser.read_bytes(4)?.try_into().unwrap(),
            )),
            BinType::Vec2 => visitor.visit_seq(SeqReader {
                de: &mut BinDeserializer::from_bytes(self.parser.input, BinType::Float),
                count: 2,
            }),
            BinType::Vec3 => visitor.visit_seq(SeqReader {
                de: &mut BinDeserializer::from_bytes(self.parser.input, BinType::Float),
                count: 3,
            }),
            BinType::Vec4 => visitor.visit_seq(SeqReader {
                de: &mut BinDeserializer::from_bytes(self.parser.input, BinType::Float),
                count: 4,
            }),
            BinType::Matrix => visitor.visit_seq(SeqReader {
                de: &mut BinDeserializer::from_bytes(self.parser.input, BinType::Float),
                count: 16,
            }),
            BinType::Color => todo!(),
            BinType::String => self.deserialize_string(visitor),
            BinType::Hash => {
                // Hash 和 Link 通常是 u32 或 u64 的包装，这里假设为 u32
                visitor.visit_u32(u32::from_le_bytes(
                    self.parser.read_bytes(4)?.try_into().unwrap(),
                ))
            }
            BinType::Path => {
                // Path 通常是 u64
                visitor.visit_u64(u64::from_le_bytes(
                    self.parser.read_bytes(8)?.try_into().unwrap(),
                ))
            }
            BinType::List | BinType::List2 => {
                let value_bin_type = self.parser.read_bintype()?;
                let _padding = self.parser.read_bytes(4)?;
                let count =
                    u32::from_le_bytes(self.parser.read_bytes(4)?.try_into().unwrap()) as usize;
                println!("📕 获取线性信息: {:?} 共 {} 个", value_bin_type, count);

                visitor.visit_seq(SeqReader {
                    de: &mut BinDeserializer::from_bytes(self.parser.input, value_bin_type),
                    count,
                })
            }
            BinType::Struct => todo!(),
            BinType::Embed => todo!(),
            BinType::Link => {
                // Hash 和 Link 通常是 u32 或 u64 的包装，这里假设为 u32
                visitor.visit_u32(u32::from_le_bytes(
                    self.parser.read_bytes(4)?.try_into().unwrap(),
                ))
            }
            BinType::Option => todo!(),
            BinType::Map => {
                let ktype = self.parser.read_bintype()?;
                let vtype = self.parser.read_bintype()?;

                let _padding = self.parser.read_bytes(4)?;
                let count = u32::from_le_bytes(self.parser.read_bytes(4)?.try_into().unwrap());
                println!("🐕 获取 Map 信息: {:?} 共 {} 个", (ktype, vtype), count);

                visitor.visit_map(HashMapReader {
                    de: &mut BinDeserializer::from_bytes(self.parser.input, ktype),
                    ktype,
                    vtype,
                    count,
                })
            }
            BinType::Flag => {
                // Flag 类似于 Bool
                visitor.visit_bool(self.parser.read_bytes(1)?[0] != 0)
            }
            BinType::Entry => todo!(),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> BinDeserializerResult<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("🎁 正在反序列化 Option，判定为 Some(...)");
        visitor.visit_some(self)
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> BinDeserializerResult<V::Value> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> BinDeserializerResult<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("👻 开始反序列化 Enum 长度: {}", self.parser.input.len());
        let class_hash = u32::from_le_bytes(self.parser.input[0..4].try_into().unwrap());
        println!("👻 准备反序列化 Enum，偷看到的类型哈希为: {:x}", class_hash);

        let (variant_index, _variant_name) = variants
            .iter()
            .enumerate()
            .find(|(_i, name)| {
                if name.starts_with("Unk") {
                    println!("👻 跳过 Unk 类型: {}", name);
                    return u32::from_str_radix(&name[3..], 16).unwrap() == class_hash;
                }
                println!(
                    "👻 class hash: {:x}  variant name : {} hash: {:x}",
                    class_hash,
                    name,
                    LeagueLoader::hash_bin(name)
                );

                LeagueLoader::hash_bin(name) == class_hash
            })
            .ok_or_else(|| {
                BinDeserializerError::Message(format!("未知的 Enum 变体哈希: 0x{:x}", class_hash))
            })?;

        println!(
            "🐕 获取变体索引: {}，变体名称: {}",
            variant_index, variants[variant_index]
        );

        visitor.visit_enum(EnumReader {
            de: self,
            variant_index: variant_index as u32,
        })
    }

    serde::forward_to_deserialize_any! {

      bool i8 i16 i32 f32 u16 i64 u8 u32 u64 f64 char str bytes

      byte_buf unit unit_struct tuple

      tuple_struct map identifier ignored_any seq

    }
}

pub struct BinParser<'de> {
    input: &'de [u8],
}

impl<'de> BinParser<'de> {
    pub fn from_bytes(input: &'de [u8]) -> Self {
        BinParser { input }
    }

    pub fn read_bytes(&mut self, len: usize) -> BinDeserializerResult<&'de [u8]> {
        if self.input.len() < len {
            return Err(BinDeserializerError::Eof);
        }

        let (slice, rest) = self.input.split_at(len);

        self.input = rest;

        Ok(slice)
    }

    pub fn read_bintype(&mut self) -> BinDeserializerResult<BinType> {
        BinType::try_from(u8::from_le_bytes(self.read_bytes(1)?.try_into().unwrap()))
    }

    pub fn skip_value(&mut self, vtype: BinType) -> BinDeserializerResult<()> {
        use std::mem::size_of;

        match vtype {
            BinType::None => {
                self.read_bytes(6)?;
            }
            BinType::Bool | BinType::S8 | BinType::U8 | BinType::Flag => {
                self.read_bytes(1)?;
            }
            BinType::S16 | BinType::U16 => {
                self.read_bytes(2)?;
            }
            BinType::S32 | BinType::U32 | BinType::Float | BinType::Hash | BinType::Link => {
                self.read_bytes(4)?;
            }
            BinType::S64 | BinType::U64 | BinType::Path => {
                self.read_bytes(8)?;
            }
            BinType::Vec2 => {
                self.read_bytes(size_of::<f32>() * 2)?;
            }
            BinType::Vec3 => {
                self.read_bytes(size_of::<f32>() * 3)?;
            }
            BinType::Vec4 => {
                self.read_bytes(size_of::<f32>() * 4)?;
            }
            BinType::Color => {
                self.read_bytes(4)?;
            }
            BinType::Matrix => {
                self.read_bytes(size_of::<f32>() * 16)?;
            }
            BinType::String => {
                let len = u16::from_le_bytes(self.read_bytes(2)?.try_into().unwrap());

                self.read_bytes(len as usize)?;
            }
            BinType::List | BinType::List2 => {
                let el_vtype = self.read_bintype()?;

                self.read_bytes(4)?;

                let count = u32::from_le_bytes(self.read_bytes(4)?.try_into().unwrap());

                for _ in 0..count {
                    self.skip_value(el_vtype)?;
                }
            }
            BinType::Struct | BinType::Embed => {
                let class_hash = u32::from_le_bytes(self.read_bytes(4)?.try_into().unwrap());

                if class_hash != 0 {
                    let fields_total_len =
                        u32::from_le_bytes(self.read_bytes(4)?.try_into().unwrap());

                    let _field_count = self.read_bytes(2)?;

                    self.read_bytes((fields_total_len - 2) as usize)?;
                }
            }
            BinType::Option => {
                let el_vtype = self.read_bintype()?;
                let count = u8::from_le_bytes(self.read_bytes(1)?.try_into().unwrap());
                if count == 1 {
                    self.skip_value(el_vtype)?;
                }
            }
            BinType::Map => {
                let ktype = self.read_bintype()?;
                let vtype = self.read_bintype()?;

                self.read_bytes(4)?;

                let count = u32::from_le_bytes(self.read_bytes(4)?.try_into().unwrap());

                for _ in 0..count {
                    self.skip_value(ktype)?;
                    self.skip_value(vtype)?;
                }
            }
            BinType::Entry => todo!(),
        }

        Ok(())
    }
}
