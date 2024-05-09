#[allow(unused)]
#[derive(Debug, Clone, serde::Serialize)]
pub struct Ticker {
    pub cik_str: String,
    pub ticker: String,
    pub title: String,
}

impl From<&tokio_postgres::Row> for Ticker {
    fn from(value: &tokio_postgres::Row) -> Self {
        Self {
            cik_str: value.get("cik_str"),
            ticker: value.get("ticker"),
            title: value.get("title"),
        }
    }
}

impl<'de> serde::de::Deserialize<'de> for Ticker {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            CikStr,
            Ticker,
            Title,
        }

        struct TickerVisitor;

        impl<'de> serde::de::Visitor<'de> for TickerVisitor {
            type Value = Ticker;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Ticker")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let cik_str = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let ticker = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                let title = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;
                Ok(Self::Value {
                    cik_str,
                    ticker,
                    title,
                })
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut cik_str = None;
                let mut ticker = None;
                let mut title = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::CikStr => {
                            if cik_str.is_some() {
                                return Err(serde::de::Error::duplicate_field("cik_str"));
                            }
                            cik_str = Some(format!("{:0>10}", map.next_value::<i32>()?));
                        }
                        Field::Ticker => {
                            if ticker.is_some() {
                                return Err(serde::de::Error::duplicate_field("ticker"));
                            }
                            ticker = Some(map.next_value()?);
                        }
                        Field::Title => {
                            if title.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title = Some(map.next_value()?);
                        }
                    }
                }
                let cik_str = cik_str.ok_or_else(|| serde::de::Error::missing_field("cik_str"))?;
                let ticker = ticker.ok_or_else(|| serde::de::Error::missing_field("ticker"))?;
                let title = title.ok_or_else(|| serde::de::Error::missing_field("title"))?;
                Ok(Self::Value {
                    cik_str,
                    ticker,
                    title,
                })
            }
        }

        const FIELDS: &[&str] = &["cik_str", "ticker", "title"];
        deserializer.deserialize_struct("Ticker", FIELDS, TickerVisitor)
    }
}
