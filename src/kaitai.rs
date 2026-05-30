use serde::Deserialize;

use crate::{Field, Packet, PacketError};

#[derive(Deserialize, Debug)]
pub struct KaitaiSpec {
    seq: Vec<KaitaiField>,
    meta: KaitaiMeta,
}

#[derive(Deserialize, Debug)]
pub struct KaitaiMeta {
    id: Option<String>,
    title: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct KaitaiField {
    id: String,
    #[serde(rename = "type")]
    ty: Option<String>,
    size: Option<String>,
}

pub fn parse_kaitai(spec: &str) -> Result<Packet, PacketError> {
    let spec: KaitaiSpec =
        serde_yaml::from_str(spec).map_err(|_| PacketError::InvalidKaitaiSpec)?;

    let fields: Vec<Field> = spec
        .seq
        .into_iter()
        .map(|kaitai_field| {
            if let Some(ty) = kaitai_field.ty {
                Ok(Field::Fixed {
                    label: kaitai_field.id,
                    bits: type_to_bits(&ty)?,
                })
            } else if let Some(expression) = kaitai_field.size {
                Ok(Field::Variable {
                    label: format!("{} ({})", kaitai_field.id, expression),
                })
            } else {
                Err(PacketError::InvalidField(kaitai_field.id))
            }
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Packet {
        title: spec.meta.title.or_else(|| spec.meta.id),
        fields: fields,
    })
}

fn type_to_bits(ty: &str) -> Result<usize, PacketError> {
    match ty {
        b if let Some(bits) = b.strip_prefix('b') => {
            let bits = bits
                .parse::<usize>()
                .map_err(|_| PacketError::InvalidBits(b.to_string()))?;

            if !(1..=64).contains(&bits) {
                return Err(PacketError::InvalidBits(b.to_string()));
            }

            Ok(bits)
        }
        u if let Some(bytes) = u.strip_prefix('u') => {
            let bytes = bytes
                .trim_end_matches("le")
                .trim_end_matches("be")
                .parse::<usize>()
                .map_err(|_| PacketError::InvalidBits(u.to_string()))?;
            Ok(bytes * 8)
        }
        other => Err(PacketError::InvalidBits(other.to_string())),
    }
}
