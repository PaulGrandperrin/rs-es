/*
 * Copyright 2015-2016 Ben Ashford
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! For metrics-based aggregations

use std::collections::{BTreeMap, HashMap};

use serde::ser;
use serde::ser::{Serialize, Serializer};
use serde_json::{from_value, to_value, Value};

// TODO - deprecated
use rustc_serialize::json::{Json, ToJson};

use ::error::EsError;
use ::json::NoOuter;
use ::units::JsonVal;

// TODO - deprecated
use super::FieldOrScript;

use super::{Aggregation, AggregationResult};
use super::common::{Agg, Script};

macro_rules! metrics_agg {
    ($b:ident) => {
        agg!($b);

        impl<'a> From<$b<'a>> for Aggregation<'a> {
            fn from(from: $b<'a>) -> Aggregation<'a> {
                Aggregation::Metrics(MetricsAggregation::$b(from))
            }
        }
    }
}

/// Min aggregation
#[derive(Debug)]
pub struct Min<'a>(Agg<'a, NoOuter>);
metrics_agg!(Min);

#[derive(Debug)]
pub struct Max<'a>(Agg<'a, NoOuter>);
metrics_agg!(Max);

/// Sum aggregation
#[derive(Debug)]
pub struct Sum<'a>(Agg<'a, NoOuter>);
metrics_agg!(Sum);

/// Avg aggregation
#[derive(Debug)]
pub struct Avg<'a>(Agg<'a, NoOuter>);
metrics_agg!(Avg);

// field_or_script_new!(Avg);
// field_or_script_to_json!(Avg);
// metrics_agg!(Avg);

// /// Stats aggregation
// #[derive(Debug)]
// pub struct Stats<'a>(FieldOrScript<'a>);

// field_or_script_new!(Stats);
// field_or_script_to_json!(Stats);
// metrics_agg!(Stats);

// /// Extended stats aggregation
// #[derive(Debug)]
// pub struct ExtendedStats<'a>(FieldOrScript<'a>);

// field_or_script_new!(ExtendedStats);
// field_or_script_to_json!(ExtendedStats);
// metrics_agg!(ExtendedStats);

// /// Value count aggregation
// #[derive(Debug)]
// pub struct ValueCount<'a>(FieldOrScript<'a>);

// field_or_script_new!(ValueCount);
// field_or_script_to_json!(ValueCount);
// metrics_agg!(ValueCount);

// /// Percentiles aggregation, see: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics-percentile-aggregation.html
// ///
// /// # Examples
// ///
// /// ```
// /// use rs_es::operations::search::aggregations::Percentiles;
// ///
// /// let p1 = Percentiles::new("field_name").with_compression(100u64);
// /// let p2 = Percentiles::new("field_name").with_percents(vec![10.0, 20.0]);
// /// ```
// #[derive(Debug)]
// pub struct Percentiles<'a> {
//     fos:         FieldOrScript<'a>,
//     percents:    Option<Vec<f64>>,
//     compression: Option<u64>
// }

// impl<'a> Percentiles<'a> {
//     pub fn new<F: Into<FieldOrScript<'a>>>(fos: F) -> Percentiles<'a> {
//         Percentiles {
//             fos:         fos.into(),
//             percents:    None,
//             compression: None
//         }
//     }

//     add_field!(with_percents, percents, Vec<f64>);
//     add_field!(with_compression, compression, u64);
// }

// impl<'a> ToJson for Percentiles<'a> {
//     fn to_json(&self) -> Json {
//         let mut d = BTreeMap::new();
//         self.fos.add_to_object(&mut d);
//         optional_add!(self, d, percents);
//         optional_add!(self, d, compression);
//         Json::Object(d)
//     }
// }

// metrics_agg!(Percentiles);

// /// Percentile Ranks aggregation
// #[derive(Debug)]
// pub struct PercentileRanks<'a> {
//     fos:    FieldOrScript<'a>,
//     values: Vec<f64>
// }

// impl<'a> PercentileRanks<'a> {
//     pub fn new<F: Into<FieldOrScript<'a>>>(fos: F, vals: Vec<f64>) -> PercentileRanks<'a> {
//         PercentileRanks {
//             fos:    fos.into(),
//             values: vals
//         }
//     }
// }

// impl<'a> ToJson for PercentileRanks<'a> {
//     fn to_json(&self) -> Json {
//         let mut d = BTreeMap::new();
//         self.fos.add_to_object(&mut d);
//         d.insert("values".to_owned(), self.values.to_json());
//         Json::Object(d)
//     }
// }

// metrics_agg!(PercentileRanks);

// /// Cardinality aggregation
// #[derive(Debug)]
// pub struct Cardinality<'a> {
//     fos:                 FieldOrScript<'a>,
//     precision_threshold: Option<u64>,
//     rehash:              Option<bool>
// }

// impl<'a> Cardinality<'a> {
//     pub fn new<F: Into<FieldOrScript<'a>>>(fos: F) -> Cardinality<'a> {
//         Cardinality {
//             fos:                 fos.into(),
//             precision_threshold: None,
//             rehash:              None
//         }
//     }

//     add_field!(with_precision_threshold, precision_threshold, u64);
//     add_field!(with_rehash, rehash, bool);
// }

// impl<'a> ToJson for Cardinality<'a> {
//     fn to_json(&self) -> Json {
//         let mut d = BTreeMap::new();
//         self.fos.add_to_object(&mut d);
//         optional_add!(self, d, precision_threshold);
//         optional_add!(self, d, rehash);
//         Json::Object(d)
//     }
// }

// metrics_agg!(Cardinality);

// /// Geo Bounds aggregation
// #[derive(Debug)]
// pub struct GeoBounds<'a> {
//     field:          &'a str,
//     wrap_longitude: Option<bool>
// }

// impl<'a> GeoBounds<'a> {
//     pub fn new(field: &'a str) -> GeoBounds {
//         GeoBounds {
//             field: field,
//             wrap_longitude: None
//         }
//     }

//     add_field!(with_wrap_longitude, wrap_longitude, bool);
// }

// impl<'a> ToJson for GeoBounds<'a> {
//     fn to_json(&self) -> Json {
//         let mut d = BTreeMap::new();
//         d.insert("field".to_owned(), self.field.to_json());
//         optional_add!(self, d, wrap_longitude);
//         Json::Object(d)
//     }
// }

// metrics_agg!(GeoBounds);

/// Scripted method aggregation
#[derive(Debug)]
pub struct ScriptedMetric<'a> {
    init_script:         Option<&'a str>,
    map_script:          &'a str,
    combine_script:      Option<&'a str>,
    reduce_script:       Option<&'a str>,
    params:              Option<Json>,
    reduce_params:       Option<Json>,
    lang:                Option<&'a str>,
    init_script_file:    Option<&'a str>,
    init_script_id:      Option<&'a str>,
    map_script_file:     Option<&'a str>,
    map_script_id:       Option<&'a str>,
    combine_script_file: Option<&'a str>,
    combine_script_id:   Option<&'a str>,
    reduce_script_file:  Option<&'a str>,
    reduce_script_id:    Option<&'a str>
}

impl<'a> ScriptedMetric<'a> {
    pub fn new(map_script: &'a str) -> ScriptedMetric<'a> {
        ScriptedMetric {
            init_script:         None,
            map_script:          map_script,
            combine_script:      None,
            reduce_script:       None,
            params:              None,
            reduce_params:       None,
            lang:                None,
            init_script_file:    None,
            init_script_id:      None,
            map_script_file:     None,
            map_script_id:       None,
            combine_script_file: None,
            combine_script_id:   None,
            reduce_script_file:  None,
            reduce_script_id:    None
        }
    }

    add_field!(with_init_script, init_script, &'a str);
    add_field!(with_combine_script, combine_script, &'a str);
    add_field!(with_reduce_script, reduce_script, &'a str);
    add_field!(with_params, params, Json);
    add_field!(with_reduce_params, reduce_params, Json);
    add_field!(with_lang, lang, &'a str);
    add_field!(with_init_script_file, init_script_file, &'a str);
    add_field!(with_init_script_id, init_script_id, &'a str);
    add_field!(with_map_script_file, map_script_file, &'a str);
    add_field!(with_map_script_id, map_script_id, &'a str);
    add_field!(with_combine_script_file, combine_script_file, &'a str);
    add_field!(with_combine_script_id, combine_script_id, &'a str);
    add_field!(with_reduce_script_file, reduce_script_file, &'a str);
    add_field!(with_reduce_script_id, reduce_script_id, &'a str);
}

// DEPRECATED
// impl<'a> ToJson for ScriptedMetric<'a> {
//     fn to_json(&self) -> Json {
//         let mut d = BTreeMap::new();
//         d.insert("map_script".to_owned(), self.map_script.to_json());
//         optional_add!(self, d, init_script);
//         optional_add!(self, d, combine_script);
//         optional_add!(self, d, reduce_script);
//         optional_add!(self, d, params);
//         optional_add!(self, d, reduce_params);
//         optional_add!(self, d, lang);
//         optional_add!(self, d, init_script_file);
//         optional_add!(self, d, init_script_id);
//         optional_add!(self, d, map_script_file);
//         optional_add!(self, d, map_script_id);
//         optional_add!(self, d, combine_script_file);
//         optional_add!(self, d, combine_script_id);
//         optional_add!(self, d, reduce_script_file);
//         optional_add!(self, d, reduce_script_id);
//         Json::Object(d)
//     }
// }

/// Individual aggregations and their options
#[derive(Debug)]
pub enum MetricsAggregation<'a> {
    Min(Min<'a>),
    Max(Max<'a>),
    Sum(Sum<'a>),
    Avg(Avg<'a>),
    // Stats(Stats<'a>),
    // ExtendedStats(ExtendedStats<'a>),
    // ValueCount(ValueCount<'a>),
    // Percentiles(Percentiles<'a>),
    // PercentileRanks(PercentileRanks<'a>),
    // Cardinality(Cardinality<'a>),
    // GeoBounds(GeoBounds<'a>),
    // ScriptedMetric(ScriptedMetric<'a>)
}

impl<'a> MetricsAggregation<'a> {
    pub fn details(&self) -> &'static str {
        use self::MetricsAggregation::*;
        match self {
            &Min(_) => "min",
            &Max(_) => "max",
            &Sum(_) => "sum",
            &Avg(_) => "avg"
        }
    }
}

impl<'a> Serialize for MetricsAggregation<'a> {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer {
        use self::MetricsAggregation::*;
        match self {
            &Min(ref min) => min.serialize(serializer),
            &Max(ref max) => max.serialize(serializer),
            &Sum(ref sum) => sum.serialize(serializer),
            &Avg(ref avg) => avg.serialize(serializer)
        }
    }
}

// results

#[derive(Debug)]
pub enum MetricsAggregationResult {
    Min(MinResult),
    Max(MaxResult),
    Sum(SumResult),
    Avg(AvgResult)
}

impl MetricsAggregationResult {
    pub fn from<'a>(ma: &MetricsAggregation<'a>, json: &Value) -> Result<Self, EsError> {
        use self::MetricsAggregation::*;
        // TODO - must be a more efficient way to do this
        let json = json.clone();
        Ok(match ma {
            &Min(_) => {
                MetricsAggregationResult::Min(try!(from_value(json)))
            },
            &Max(_) => {
                MetricsAggregationResult::Max(try!(from_value(json)))
            },
            &Sum(_) => {
                MetricsAggregationResult::Sum(try!(from_value(json)))
            },
            &Avg(_) => {
                MetricsAggregationResult::Avg(try!(from_value(json)))
            },
            // &MetricsAggregation::Stats(_) => {
            //     AggregationResult::Stats(StatsResult::from(json))
            // },
            // &MetricsAggregation::ExtendedStats(_) => {
            //     AggregationResult::ExtendedStats(ExtendedStatsResult::from(json))
            // },
            // &MetricsAggregation::ValueCount(_) => {
            //     AggregationResult::ValueCount(ValueCountResult::from(json))
            // }
            // &MetricsAggregation::Percentiles(_) => {
            //     AggregationResult::Percentiles(PercentilesResult::from(json))
            // },
            // &MetricsAggregation::PercentileRanks(_) => {
            //     AggregationResult::PercentileRanks(PercentileRanksResult::from(json))
            // },
            // &MetricsAggregation::Cardinality(_) => {
            //     AggregationResult::Cardinality(CardinalityResult::from(json))
            // },
            // &MetricsAggregation::GeoBounds(_) => {
            //     AggregationResult::GeoBounds(GeoBoundsResult::from(json))
            // },
            // &MetricsAggregation::ScriptedMetric(_) => {
            //     AggregationResult::ScriptedMetric(ScriptedMetricResult::from(json))
            // }
        })
    }
}

macro_rules! metrics_agg_as {
    ($n:ident,$t:ident,$rt:ty) => {
        agg_as!($n,Metrics,MetricsAggregationResult,$t,$rt);
    }
}

impl AggregationResult {
    metrics_agg_as!(as_min, Min, MinResult);
    metrics_agg_as!(as_max, Max, MaxResult);
    metrics_agg_as!(as_sum, Sum, SumResult);
    metrics_agg_as!(as_avg, Avg, AvgResult);
    // agg_as!(as_stats, Stats, StatsResult);
    // agg_as!(as_extended_stats, ExtendedStats, ExtendedStatsResult);
    // agg_as!(as_value_count, ValueCount, ValueCountResult);
    // agg_as!(as_percentiles, Percentiles, PercentilesResult);
    // agg_as!(as_percentile_ranks, PercentileRanks, PercentileRanksResult);
    // agg_as!(as_cardinality, Cardinality, CardinalityResult);
    // agg_as!(as_geo_bounds, GeoBounds, GeoBoundsResult);
    // agg_as!(as_scripted_metric, ScriptedMetric, ScriptedMetricResult);
}

// specific result objects

/// Min Result
#[derive(Debug, Deserialize)]
pub struct MinResult {
    pub value: JsonVal
}

#[derive(Debug, Deserialize)]
pub struct MaxResult {
    pub value: JsonVal
}

#[derive(Debug, Deserialize)]
pub struct SumResult {
    pub value: f64
}

#[derive(Debug, Deserialize)]
pub struct AvgResult {
    pub value: f64
}

#[cfg(test)]
pub mod tests {
    use serde_json;

    use super::super::Aggregations;
    use super::Min;

    #[test]
    fn test_min_aggregation() {
        let aggs:Aggregations = ("min_test", Min::field("blah")).into();

        assert_eq!("{\"min_test\":{\"min\":{\"field\":\"blah\"}}}",
                   serde_json::to_string(&aggs).unwrap());
    }
}
