//! Usage aggregates — mirrors src/shared/usage-aggregates.ts

use std::collections::HashMap;

/// Running totals for latency statistics.
#[derive(Debug, Clone)]
pub struct LatencyTotals {
    pub count: u64,
    pub sum: f64,
    pub min: f64,
    pub max: f64,
    pub p95_max: f64,
}

impl LatencyTotals {
    pub fn new() -> Self {
        Self {
            count: 0,
            sum: 0.0,
            min: f64::INFINITY,
            max: 0.0,
            p95_max: 0.0,
        }
    }
}

impl Default for LatencyTotals {
    fn default() -> Self {
        Self::new()
    }
}

/// Per-day latency accumulator (internal).
#[derive(Debug, Clone)]
pub struct DailyLatencyEntry {
    pub date: String,
    pub count: u64,
    pub sum: f64,
    pub min: f64,
    pub max: f64,
    pub p95_max: f64,
}

/// Latency statistics for a single measurement.
#[derive(Debug, Clone)]
pub struct LatencyLike {
    pub count: u64,
    pub avg_ms: f64,
    pub min_ms: f64,
    pub max_ms: f64,
    pub p95_ms: f64,
}

/// Latency statistics with date attached.
#[derive(Debug, Clone)]
pub struct DailyLatencyInput {
    pub date: String,
    pub count: u64,
    pub avg_ms: f64,
    pub min_ms: f64,
    pub max_ms: f64,
    pub p95_ms: f64,
}

/// Computed latency summary.
#[derive(Debug, Clone)]
pub struct LatencySummary {
    pub count: u64,
    pub avg_ms: f64,
    pub min_ms: f64,
    pub max_ms: f64,
    pub p95_ms: f64,
}

/// Computed daily latency summary.
#[derive(Debug, Clone)]
pub struct DailyLatencySummary {
    pub date: String,
    pub count: u64,
    pub avg_ms: f64,
    pub min_ms: f64,
    pub max_ms: f64,
    pub p95_ms: f64,
}

/// Merge a latency measurement into running totals.
pub fn merge_usage_latency(totals: &mut LatencyTotals, latency: Option<&LatencyLike>) {
    let Some(latency) = latency else { return };
    if latency.count == 0 {
        return;
    }
    totals.count += latency.count;
    totals.sum += latency.avg_ms * latency.count as f64;
    totals.min = totals.min.min(latency.min_ms);
    totals.max = totals.max.max(latency.max_ms);
    totals.p95_max = totals.p95_max.max(latency.p95_ms);
}

/// Merge daily latency entries into an accumulator map.
pub fn merge_usage_daily_latency(
    daily_latency_map: &mut HashMap<String, DailyLatencyEntry>,
    daily_latency: Option<&[DailyLatencyInput]>,
) {
    let Some(daily) = daily_latency else { return };
    for day in daily {
        let existing = daily_latency_map
            .entry(day.date.clone())
            .or_insert_with(|| DailyLatencyEntry {
                date: day.date.clone(),
                count: 0,
                sum: 0.0,
                min: f64::INFINITY,
                max: 0.0,
                p95_max: 0.0,
            });
        existing.count += day.count;
        existing.sum += day.avg_ms * day.count as f64;
        existing.min = existing.min.min(day.min_ms);
        existing.max = existing.max.max(day.max_ms);
        existing.p95_max = existing.p95_max.max(day.p95_ms);
    }
}

/// Build the final aggregate tail from accumulated maps.
///
/// Generic parameters mirror the TS version's generics:
/// - `TTotals` = channel totals with a `total_cost` field
/// - `TDaily` = per-day entry with a `date` field
/// - `TModelDaily` = per-model-per-day entry with `date` and `cost` fields
pub struct UsageAggregateTail<TTotals, TDaily, TModelDaily> {
    pub by_channel: Vec<ChannelTotals<TTotals>>,
    pub latency: Option<LatencySummary>,
    pub daily_latency: Vec<DailyLatencySummary>,
    pub model_daily: Vec<TModelDaily>,
    pub daily: Vec<TDaily>,
}

/// A channel with its totals, sorted by cost descending.
#[derive(Debug, Clone)]
pub struct ChannelTotals<T> {
    pub channel: String,
    pub totals: T,
}

/// Trait for types that have a `total_cost` field.
pub trait HasTotalCost {
    fn total_cost(&self) -> f64;
}

/// Trait for types that have a `date` field.
pub trait HasDate {
    fn date(&self) -> &str;
}

/// Trait for model daily entries that have `date` and `cost`.
pub trait HasDateAndCost {
    fn date(&self) -> &str;
    fn cost(&self) -> f64;
}

/// Build the tail aggregate from the accumulated maps.
pub fn build_usage_aggregate_tail<TTotals, TDaily, TModelDaily>(
    by_channel_map: HashMap<String, TTotals>,
    latency_totals: &LatencyTotals,
    daily_latency_map: HashMap<String, DailyLatencyEntry>,
    model_daily_map: HashMap<String, TModelDaily>,
    daily_map: HashMap<String, TDaily>,
) -> UsageAggregateTail<TTotals, TDaily, TModelDaily>
where
    TTotals: HasTotalCost,
    TDaily: HasDate,
    TModelDaily: HasDateAndCost,
{
    // byChannel: sorted by totalCost descending
    let mut by_channel: Vec<ChannelTotals<TTotals>> = by_channel_map
        .into_iter()
        .map(|(channel, totals)| ChannelTotals { channel, totals })
        .collect();
    by_channel.sort_by(|a, b| {
        b.totals
            .total_cost()
            .partial_cmp(&a.totals.total_cost())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // latency
    let latency = if latency_totals.count > 0 {
        Some(LatencySummary {
            count: latency_totals.count,
            avg_ms: latency_totals.sum / latency_totals.count as f64,
            min_ms: if latency_totals.min == f64::INFINITY {
                0.0
            } else {
                latency_totals.min
            },
            max_ms: latency_totals.max,
            p95_ms: latency_totals.p95_max,
        })
    } else {
        None
    };

    // daily latency: sorted by date ascending
    let mut daily_latency: Vec<DailyLatencySummary> = daily_latency_map
        .into_values()
        .map(|entry| DailyLatencySummary {
            date: entry.date,
            count: entry.count,
            avg_ms: if entry.count > 0 {
                entry.sum / entry.count as f64
            } else {
                0.0
            },
            min_ms: if entry.min == f64::INFINITY {
                0.0
            } else {
                entry.min
            },
            max_ms: entry.max,
            p95_ms: entry.p95_max,
        })
        .collect();
    daily_latency.sort_by(|a, b| a.date.cmp(&b.date));

    // model daily: sorted by date asc, then cost desc
    let mut model_daily: Vec<TModelDaily> = model_daily_map.into_values().collect();
    model_daily.sort_by(|a, b| {
        a.date()
            .cmp(b.date())
            .then_with(|| {
                b.cost()
                    .partial_cmp(&a.cost())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    });

    // daily: sorted by date ascending
    let mut daily: Vec<TDaily> = daily_map.into_values().collect();
    daily.sort_by(|a, b| a.date().cmp(b.date()));

    UsageAggregateTail {
        by_channel,
        latency,
        daily_latency,
        model_daily,
        daily,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_latency_basic() {
        let mut totals = LatencyTotals::new();
        let lat = LatencyLike {
            count: 10,
            avg_ms: 50.0,
            min_ms: 10.0,
            max_ms: 100.0,
            p95_ms: 90.0,
        };
        merge_usage_latency(&mut totals, Some(&lat));
        assert_eq!(totals.count, 10);
        assert!((totals.sum - 500.0).abs() < f64::EPSILON);
        assert!((totals.min - 10.0).abs() < f64::EPSILON);
        assert!((totals.max - 100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn merge_latency_none_is_noop() {
        let mut totals = LatencyTotals::new();
        merge_usage_latency(&mut totals, None);
        assert_eq!(totals.count, 0);
    }

    #[test]
    fn merge_latency_zero_count_is_noop() {
        let mut totals = LatencyTotals::new();
        let lat = LatencyLike {
            count: 0,
            avg_ms: 50.0,
            min_ms: 10.0,
            max_ms: 100.0,
            p95_ms: 90.0,
        };
        merge_usage_latency(&mut totals, Some(&lat));
        assert_eq!(totals.count, 0);
    }

    #[test]
    fn merge_daily_latency() {
        let mut map = HashMap::new();
        let input = vec![
            DailyLatencyInput {
                date: "2024-01-01".to_string(),
                count: 5,
                avg_ms: 20.0,
                min_ms: 10.0,
                max_ms: 30.0,
                p95_ms: 28.0,
            },
            DailyLatencyInput {
                date: "2024-01-01".to_string(),
                count: 3,
                avg_ms: 40.0,
                min_ms: 15.0,
                max_ms: 50.0,
                p95_ms: 45.0,
            },
        ];
        merge_usage_daily_latency(&mut map, Some(&input));
        let entry = &map["2024-01-01"];
        assert_eq!(entry.count, 8);
        assert!((entry.sum - (20.0 * 5.0 + 40.0 * 3.0)).abs() < f64::EPSILON);
        assert!((entry.min - 10.0).abs() < f64::EPSILON);
        assert!((entry.max - 50.0).abs() < f64::EPSILON);
        assert!((entry.p95_max - 45.0).abs() < f64::EPSILON);
    }
}
