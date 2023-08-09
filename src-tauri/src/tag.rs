use rusqlite::ToSql;
use rusqlite::types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct HslColor {
    pub hue: u16,
    pub saturation: u8,
    pub lightness: u8,
}

// I actually don't care about the space efficiency of color, I just want
// color to be storable in a single column
impl ToSql for HslColor {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let num: u32 =
            ((self.hue as u32) << 16) +
            ((self.saturation as u32) << 8) +
            (self.lightness as u32);
        Ok(ToSqlOutput::from(num))
    }
}

impl FromSql for HslColor {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let num = u32::column_result(value)?;
        Ok(HslColor {
            hue: ((num & 0xFFFF0000) >> 16) as u16,
            saturation: ((num & 0xFF00) >> 8) as u8,
            lightness: (num & 0xFF) as u8,
        })
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TagData {
    pub name: String,
    pub color: HslColor,
    pub active: bool,
}

pub type TagId = i64;
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Tag {
    pub id: TagId,
    pub data: TagData,
}
