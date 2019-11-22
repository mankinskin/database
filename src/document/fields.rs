use std::collections::HashMap;

pub struct ArrayValue(pub google_firestore::ArrayValue);

impl std::convert::From<google_firestore::ArrayValue> for ArrayValue {
    fn from(other: google_firestore::ArrayValue) -> Self {
        ArrayValue(other)
    }
}

impl std::convert::From<Vec<FieldValue>> for ArrayValue {
    fn from(vals: Vec<FieldValue>) -> Self {
        ArrayValue(google_firestore::ArrayValue{
            values: Some(vals.iter()
                             .map(|v| v.clone().into())
                             .collect()),
        })
    }
}

impl From<Option<HashMap<String, google_firestore::Value>>> for MapValue {
    fn from(other: Option<HashMap<String, google_firestore::Value>>) -> Self {
        if let Some(map) = other {
            MapValue(google_firestore::MapValue {
                fields: Some(map),
                ..google_firestore::MapValue::default()
            })
        } else {
            MapValue(google_firestore::MapValue::default())
        }
    }
}
pub struct MapValue(pub google_firestore::MapValue);

impl std::convert::From<google_firestore::MapValue> for MapValue {
    fn from(other: google_firestore::MapValue) -> Self {
        MapValue(other)
    }
}
pub struct GeoPointValue(pub google_firestore::LatLng);

impl std::cmp::PartialEq for GeoPointValue {
    fn eq(&self, other: &Self) -> bool {
        self.0.latitude == other.0.latitude &&
        self.0.longitude == other.0.longitude
    }
}
impl std::convert::From<google_firestore::LatLng> for GeoPointValue {
    fn from(other: google_firestore::LatLng) -> Self {
        GeoPointValue(other)
    }
}
impl std::convert::From<(f64, f64)> for GeoPointValue {
    fn from((lat, lng): (f64, f64)) -> Self {
        GeoPointValue(google_firestore::LatLng {
            latitude: Some(lat),
            longitude: Some(lng),
        })
    }
}
#[derive(PartialEq, Clone)]
pub struct FieldValue(pub google_firestore::Value);

impl FieldValue {

    pub fn timestamp_value(v: &str) -> FieldValue {
        FieldValue(google_firestore::Value {
            timestamp_value: Some(v.to_string()),
            ..Default::default()
        })
    }
    pub fn reference_value(v: &str) -> FieldValue {
        FieldValue(google_firestore::Value {
            reference_value: Some(v.to_string()),
            ..Default::default()
        })
    }
    pub fn map_value(v: google_firestore::MapValue) -> FieldValue {
        FieldValue(google_firestore::Value {
            map_value: Some(v),
            ..Default::default()
        })
    }
    pub fn null_value() -> FieldValue {
        FieldValue(google_firestore::Value {
            null_value: None,
            ..Default::default()
        })
    }
}

impl From<google_firestore::Value> for FieldValue {
    fn from(v: google_firestore::Value) -> Self {
        FieldValue(v)
    }
}
impl From<FieldValue> for google_firestore::Value {
    fn from(v: FieldValue) -> Self {
        v.0
    }
}
use std::convert::{TryFrom};

impl From<()> for FieldValue {
    fn from(v: ()) -> FieldValue {
        FieldValue(google_firestore::Value {
            null_value: Some("NULL".to_string()),
            ..Default::default()
        })
    }
}
impl From<bool> for FieldValue {
    fn from(v: bool) -> FieldValue {
        FieldValue(google_firestore::Value {
            boolean_value: Some(v),
            ..Default::default()
        })
    }
}
impl TryFrom<FieldValue> for bool {
    type Error = &'static str;
    fn try_from(v: FieldValue) -> Result<Self, Self::Error> {
        match v {
            FieldValue(google_firestore::Value {
                boolean_value: Some(v),
                ..
            }) => Ok(v),
            _ => Err("Failed to read FieldValue as bool.")
        }
    }
}
impl From<f64> for FieldValue {
    fn from(v: f64) -> Self {
        FieldValue(google_firestore::Value {
            double_value: Some(v),
            ..Default::default()
        })
    }
}
impl TryFrom<FieldValue> for f64 {
    type Error = &'static str;
    fn try_from(v: FieldValue) -> Result<Self, Self::Error> {
        match v {
            FieldValue(google_firestore::Value {
                double_value: Some(v),
                ..
            }) => Ok(v),
            _ => Err("Failed to read FieldValue as double(f64).")
        }
    }
}
impl From<i64> for FieldValue {
    fn from(v: i64) -> Self {
        FieldValue(google_firestore::Value {
            integer_value: Some(v.to_string()),
            ..Default::default()
        })
    }
}
impl From<u32> for FieldValue {
    fn from(v: u32) -> Self {
        Self::from(v as i64)
    }
}
impl From<i32> for FieldValue {
    fn from(v: i32) -> Self {
        Self::from(v as i64)
    }
}
impl TryFrom<FieldValue> for i64 {
    type Error = &'static str;
    fn try_from(v: FieldValue) -> Result<Self, Self::Error> {
        match v {
            FieldValue(google_firestore::Value {
                integer_value: Some(v),
                ..
            }) => v.parse::<i64>().map_err(|_| "Failed to read FieldValue as long int(i64)."),
            _ => Err("Failed to read FieldValue as long int(i64).")
        }
    }
}
impl TryFrom<FieldValue> for u32 {
    type Error = &'static str;
    fn try_from(v: FieldValue) -> Result<Self, Self::Error> {
        i64::try_from(v).map(|i| i as Self)
    }
}

impl From<String> for FieldValue {
    fn from(v: String) -> Self {
        FieldValue(google_firestore::Value {
            string_value: Some(v),
            ..Default::default()
        })
    }
}
impl TryFrom<FieldValue> for String {
    type Error = &'static str;
    fn try_from(v: FieldValue) -> Result<Self, Self::Error> {
        match v {
            FieldValue(google_firestore::Value {
                string_value: Some(v),
                ..
            }) => Ok(v),
            _ => Err("Failed to read FieldValue as String.")
        }
    }
}
impl From<(f64, f64)> for FieldValue {
    fn from((lat, lng): (f64, f64)) -> Self {
        Self::from(google_firestore::LatLng {
            latitude: Some(lat),
            longitude: Some(lng),
        })
    }
}
impl From<&str> for FieldValue {
    fn from(v: &str) -> Self {
        Self::from(v.to_string())
    }
}
impl From<google_firestore::LatLng> for FieldValue {
    fn from(v: google_firestore::LatLng) -> Self {
        FieldValue(google_firestore::Value {
            geo_point_value: Some(v),
            ..Default::default()
        })
    }
}
impl std::convert::TryFrom<FieldValue> for (f64, f64) {
    type Error = &'static str;
    fn try_from(v: FieldValue) -> Result<Self, Self::Error> {
        match v {
            FieldValue(google_firestore::Value {
                geo_point_value: Some(v),
                ..
            }) => Ok((v.latitude.unwrap_or(0.0),
                      v.longitude.unwrap_or(0.0))),
            _ => Err("Failed to read FieldValue as (f64, f64).")
        }
    }
}
impl std::convert::TryFrom<FieldValue> for google_firestore::LatLng {
    type Error = &'static str;
    fn try_from(v: FieldValue) -> Result<Self, Self::Error> {
        match v {
            FieldValue(google_firestore::Value {
                geo_point_value: Some(v),
                ..
            }) => Ok(v),
            _ => Err("Failed to read FieldValue as LatLng.")
        }
    }
}
impl From<&[u8]> for FieldValue {
    fn from(v: &[u8]) -> Self {
        FieldValue(google_firestore::Value {
            bytes_value: Some(std::str::from_utf8(v)
                                .unwrap_or("").to_string()),
            ..Default::default()
        })
    }
}

impl<T: Into<google_firestore::Value> + Clone> From<Vec<T>> for FieldValue {
    fn from(v: Vec<T>) -> Self {
        FieldValue(google_firestore::Value {
            array_value: Some(google_firestore::ArrayValue {
                    values: Some(v.iter()
                                  .map(|val| (*val).clone().into())
                                  .collect::<Vec<google_firestore::Value>>()),
                }),
            ..Default::default()
        })
    }
}
//impl<T: From<google_firestore::Value> + Clone> std::convert::TryFrom<FieldValue> for Vec<T> {
//    type Error = &'static str;
//    fn try_from(v: FieldValue) -> Result<Self, Self::Error> {
//        match v {
//            FieldValue(google_firestore::Value {
//                array_value:
//                    Some(google_firestore::ArrayValue {
//                        values: Some(arr),
//                    }),
//                ..
//            }) => Ok(arr.iter()
//                      .map(|val| (*val).clone().into())
//                      .collect()),
//            _ => Err("Failed to read FieldValue as String.")
//        }
//    }
//}

impl<T: Into<google_firestore::Value> + Clone> From<HashMap<String, T>> for FieldValue {
    fn from(v: HashMap<String, T>) -> Self {
        FieldValue(google_firestore::Value {
            map_value: Some(google_firestore::MapValue {
                fields: Some(v.iter()
                              .map(|(key, val)|
                                   ((*key).clone(), (*val).clone().into()))
                              .collect::<HashMap<String, google_firestore::Value>>()),
        }),
            ..Default::default()
        })
    }
}
impl Default for FieldValue {
    fn default() -> Self {
        FieldValue(Default::default())
    }
}

impl std::fmt::Debug for FieldValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let v = self.0.clone();
        let s = if let Some(bytes) = v.bytes_value {
            bytes
        } else if let Some(timestamp) = v.timestamp_value {
            timestamp
        } else if let Some(geo_point) = v.geo_point_value {
            format!("{:?}", geo_point)
        } else if let Some(reference) = v.reference_value {
            format!("Reference: {}", reference)
        } else if let Some(double) = v.double_value {
            format!("{}", double)
        } else if let Some(map) = v.map_value {
            format!("{:?}", map)
        } else if let Some(string) = v.string_value {
            string
        } else if let Some(boolean) = v.boolean_value {
            format!("{}", boolean)
        } else if let Some(array) = v.array_value {
            format!("{:?}", array)
        } else if let Some(integer) = v.integer_value {
            format!("{}", integer)
        } else if let Some(null) = v.null_value {
            format!("{}", null)
        } else {
            "None".to_string()
        };
        write!(f, "{}", s)
    }
}
