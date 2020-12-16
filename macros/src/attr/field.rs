use super::parse_assign_str;
use std::convert::TryFrom;
use syn::{Attribute, Ident, Result};

#[derive(Default)]
pub struct FieldAttr {
    pub type_override: Option<String>,
    pub rename: Option<String>,
    pub inline: bool,
    pub skip: bool,
    pub flatten: bool
}

#[cfg(feature = "serde-compat")]
#[derive(Default)]
pub struct SerdeFieldAttr(FieldAttr);

impl FieldAttr {
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        let mut result = Self::default();
        attrs
            .iter()
            .filter(|a| a.path.is_ident("ts"))
            .map(FieldAttr::try_from)
            .collect::<Result<Vec<FieldAttr>>>()?
            .into_iter()
            .for_each(|a| result.merge(a));

        #[cfg(feature = "serde-compat")]
            {
                attrs
                    .iter()
                    .filter(|a| a.path.is_ident("serde"))
                    .flat_map(|attr| match SerdeFieldAttr::try_from(attr) {
                        Ok(attr) => Some(attr),
                        Err(_) => {
                            use quote::ToTokens;
                            crate::utils::print_warning(
                                "failed to parse serde attribute",
                                format!("{}", attr.to_token_stream()),
                                "ts-rs failed to parse this attribute. It will be ignored.",
                            )
                                .unwrap();
                            None
                        }
                    })
                    .for_each(|a| result.merge(a.0));
            }

        Ok(result)
        
    }

    fn merge(&mut self, FieldAttr { type_override, rename, inline, skip, flatten }: FieldAttr) {
        self.rename = self.rename.take().or(rename);
        self.type_override = self.type_override.take().or(type_override);
        self.inline = self.inline || inline;
        self.skip = self.skip || skip;
        self.flatten = self.flatten | flatten;
    }

}

impl_parse! {
    FieldAttr(input, out) {
        "type" => out.type_override = Some(parse_assign_str(input)?),
        "rename" => out.rename = Some(parse_assign_str(input)?),
        "inline" => out.inline = true,
        "skip" => out.skip = true,
        "flatten" => out.flatten = true,
    }
}

#[cfg(feature = "serde-compat")]
impl_parse! {
    SerdeFieldAttr(input, out) {
        "rename" => out.0.rename = Some(parse_assign_str(input)?),
        "skip_serializing" => out.0.skip = true,
        "skip_deserializing" => out.0.skip = true,
        "flatten" => out.0.flatten = true,
    }
}