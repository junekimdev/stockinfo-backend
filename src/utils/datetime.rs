// use super::error::Error;
// use std::ops::Add;
// use std::time::{SystemTime, UNIX_EPOCH};

// pub fn get_timestamp(t: SystemTime) -> crate::utils::Result<u64> {
//     let epoch = t.duration_since(UNIX_EPOCH)?;
//     Ok(epoch.as_secs())
// }

// pub fn parse_time_from(micros: u64) -> SystemTime {
//     let t = std::time::Duration::from_micros(micros);
//     UNIX_EPOCH.add(t)
// }

// pub fn parse_date_from(str: &str) -> crate::utils::Result<time::Date> {
//     let format1 = time::macros::format_description!("[year][month][day]");
//     let format2 = time::macros::format_description!("[year]-[month]-[day]");
//     match time::Date::parse(str, &format1) {
//         Ok(d1) => Ok(d1),
//         Err(e1) => match time::Date::parse(str, &format2) {
//             Ok(d2) => Ok(d2),
//             Err(e2) => Err(Error::General(format!(
//                 "{} | {}",
//                 &e1.to_string(),
//                 &e2.to_string()
//             ))),
//         },
//     }
// }

pub fn get_sunday_of_week(date: &time::Date) -> crate::utils::Result<time::Date> {
    Ok(time::Date::from_iso_week_date(
        date.year(),
        date.sunday_based_week(),
        time::Weekday::Sunday,
    )?)
}

// //==================== Time Serialize/Deserialize ====================

// /// Serialize SystemTime into micro-seconds in u128
// pub fn time_serialize<S>(t: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: serde::Serializer,
// {
//     let n = t.duration_since(UNIX_EPOCH).unwrap().as_micros();
//     serializer.serialize_u128(n)
// }

// /// Serialize Optional SystemTime into micro-seconds in u128 or null
// pub fn time_opt_serialize<S>(t: &Option<SystemTime>, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: serde::Serializer,
// {
//     if let Some(tt) = t {
//         let n = tt.duration_since(UNIX_EPOCH).unwrap().as_micros();
//         serializer.serialize_u128(n)
//     } else {
//         serializer.serialize_none()
//     }
// }

// struct TimeVisitor;

// impl<'de> serde::de::Visitor<'de> for TimeVisitor {
//     type Value = SystemTime;

//     fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(formatter, "micro seconds in u64")
//     }

//     fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
//     where
//         E: serde::de::Error,
//     {
//         let t = std::time::Duration::from_micros(v);
//         Ok(UNIX_EPOCH.add(t))
//     }
// }

// /// Deserialize SystemTime from u64 (micro-seconds)
// pub fn time_deserialize<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
// where
//     D: serde::Deserializer<'de>,
// {
//     deserializer.deserialize_u64(TimeVisitor)
// }

//==================== Date Serialize/Deserialize ====================

/// Serialize time::Date into YYYYMMDD format
pub fn date_serialize<S>(t: &time::Date, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let format = time::format_description::parse("[year]-[month]-[day]").unwrap();
    let s = t.format(&format).unwrap();
    serializer.serialize_str(&s)
}

/// Serialize Optional time::Date into YYYYMMDD format
pub fn date_opt_serialize<S>(t: &Option<time::Date>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if let Some(tt) = t {
        let format = time::format_description::parse("[year]-[month]-[day]").unwrap();
        let s = tt.format(&format).unwrap();
        serializer.serialize_str(&s)
    } else {
        serializer.serialize_none()
    }
}

struct DateVisitor;

impl<'de> serde::de::Visitor<'de> for DateVisitor {
    type Value = time::Date;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "string in date format: YYYYMMDD | YYYY-MM-DD")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let format1 = time::macros::format_description!("[year][month][day]");
        let format2 = time::macros::format_description!("[year]-[month]-[day]");
        match time::Date::parse(v, &format1) {
            Ok(d1) => Ok(d1),
            Err(e1) => match time::Date::parse(v, &format2) {
                Ok(d2) => Ok(d2),
                Err(e2) => Err(E::custom(format!(
                    "{} | {}",
                    &e1.to_string(),
                    &e2.to_string()
                ))),
            },
        }
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(&v)
    }
}

/// Deserialize time::Date from string
pub fn date_deserialize<'de, D>(deserializer: D) -> Result<time::Date, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_string(DateVisitor)
}

struct DateOptVisitor;

impl<'de> serde::de::Visitor<'de> for DateOptVisitor {
    type Value = Option<time::Date>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "optional string in date format: YYYYMMDD | YYYY-MM-DD"
        )
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // delegate deserializing to date_deserialize() and wrap the result with Some()
        date_deserialize(deserializer).map(Some)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(None)
    }
}

/// Deserialize Optional time::Date from string
pub fn date_opt_deserialize<'de, D>(deserializer: D) -> Result<Option<time::Date>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_option(DateOptVisitor)
}

//==================== KRX Datetime Serialize/Deserialize ====================

struct KrxDatetimeVisitor;

impl<'de> serde::de::Visitor<'de> for KrxDatetimeVisitor {
    type Value = time::OffsetDateTime;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "string in datetime format: [year].[month].[day] [AM|PM] [hour]:[minute]:[second]"
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let format_date = time::macros::format_description!("[year].[month].[day]");
        let format_time = time::macros::format_description!("[hour]:[minute]:[second]");
        let mut dt = v.split_ascii_whitespace();
        let date = dt
            .next()
            .map(|s| time::Date::parse(s, &format_date).unwrap())
            .unwrap();
        let is_pm = dt.next().map(|s| s == "PM").unwrap();
        let mut time = dt
            .next()
            .map(|s| time::Time::parse(s, &format_time).unwrap())
            .unwrap();

        // Apply AM/PM
        if is_pm && time.hour() != 12 {
            time += time::Duration::hours(12);
        } else if !is_pm && time.hour() == 12 {
            time -= time::Duration::hours(12);
        }

        let tz = time::macros::offset!(+9:00:00); // Korean Standard timezone
        Ok(time::OffsetDateTime::new_in_offset(date, time, tz))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(&v)
    }
}

/// Deserialize time::OffsetDateTime from KRX datetime string
pub fn krx_datetime_deserialize<'de, D>(deserializer: D) -> Result<time::OffsetDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_string(KrxDatetimeVisitor)
}

//==================== Tests ====================
#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, serde::Deserialize, PartialEq)]
    struct KrxDatetime {
        #[serde(deserialize_with = "krx_datetime_deserialize")]
        time: time::OffsetDateTime,
    }

    #[test]
    fn test_deserialize_krx_datetime() {
        {
            let json = "{\"time\": \"2024.01.01 AM 12:00:00\"}";
            let result = serde_json::from_str::<KrxDatetime>(json).unwrap();
            assert_eq!(result.time.hour(), 0);
        }
        {
            let json = "{\"time\": \"2024.01.01 AM 01:00:00\"}";
            let result = serde_json::from_str::<KrxDatetime>(json).unwrap();
            assert_eq!(result.time.hour(), 1);
        }
        {
            let json = "{\"time\": \"2024.01.01 PM 12:00:00\"}";
            let result = serde_json::from_str::<KrxDatetime>(json).unwrap();
            assert_eq!(result.time.hour(), 12);
        }
        {
            let json = "{\"time\": \"2024.01.01 PM 01:00:00\"}";
            let result = serde_json::from_str::<KrxDatetime>(json).unwrap();
            assert_eq!(result.time.hour(), 13);
        }
    }
}
