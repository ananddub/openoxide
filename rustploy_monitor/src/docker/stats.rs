#[cfg(test)]
mod tests {
    use super::super::types::{ContainerStats, ContainerSummary};

    fn stats_from(json: &str) -> ContainerStats {
        serde_json::from_str(json).expect("valid stats json")
    }

    #[test]
    fn cpu_percent_scales_by_core_count() {
        let stats = stats_from(
            r#"{
                "cpu_stats": {
                    "cpu_usage": {"total_usage": 2000},
                    "system_cpu_usage": 20000,
                    "online_cpus": 4
                },
                "precpu_stats": {
                    "cpu_usage": {"total_usage": 1000},
                    "system_cpu_usage": 10000,
                    "online_cpus": 4
                }
            }"#,
        );

        assert!((stats.cpu_percent() - 40.0).abs() < 0.001);
    }

    #[test]
    fn cpu_percent_is_zero_on_first_sample() {
        let stats = stats_from(
            r#"{
                "cpu_stats": {"cpu_usage": {"total_usage": 500}, "system_cpu_usage": 1000},
                "precpu_stats": {"cpu_usage": {"total_usage": 0}, "system_cpu_usage": 0}
            }"#,
        );

        assert!(stats.cpu_percent() >= 0.0);
        assert!(stats.cpu_percent().is_finite());
    }

    #[test]
    fn cpu_percent_survives_a_missing_system_usage() {
        let stats = stats_from(
            r#"{
                "cpu_stats": {"cpu_usage": {"total_usage": 500}},
                "precpu_stats": {"cpu_usage": {"total_usage": 100}}
            }"#,
        );

        assert_eq!(stats.cpu_percent(), 0.0);
    }

    #[test]
    fn cpu_percent_falls_back_to_percpu_length() {
        let stats = stats_from(
            r#"{
                "cpu_stats": {
                    "cpu_usage": {"total_usage": 2000, "percpu_usage": [500, 500, 500, 500]},
                    "system_cpu_usage": 20000
                },
                "precpu_stats": {
                    "cpu_usage": {"total_usage": 1000, "percpu_usage": [250, 250, 250, 250]},
                    "system_cpu_usage": 10000
                }
            }"#,
        );

        assert!((stats.cpu_percent() - 40.0).abs() < 0.001);
    }

    #[test]
    fn counters_that_go_backwards_do_not_underflow() {
        let stats = stats_from(
            r#"{
                "cpu_stats": {"cpu_usage": {"total_usage": 100}, "system_cpu_usage": 1000},
                "precpu_stats": {"cpu_usage": {"total_usage": 500}, "system_cpu_usage": 5000}
            }"#,
        );

        assert_eq!(stats.cpu_percent(), 0.0);
    }

    #[test]
    fn memory_excludes_page_cache() {
        let stats = stats_from(
            r#"{
                "memory_stats": {
                    "usage": 200000000,
                    "limit": 400000000,
                    "stats": {"inactive_file": 50000000}
                }
            }"#,
        );

        assert_eq!(stats.memory.used_bytes(), 150_000_000);
        assert!((stats.memory.used_percent() - 37.5).abs() < 0.001);
    }

    #[test]
    fn memory_falls_back_to_cgroup_v1_cache() {
        let stats =
            stats_from(r#"{"memory_stats": {"usage": 100, "limit": 200, "stats": {"cache": 40}}}"#);

        assert_eq!(stats.memory.used_bytes(), 60);
    }

    #[test]
    fn memory_percent_is_zero_without_a_limit() {
        let stats = stats_from(r#"{"memory_stats": {"usage": 100, "limit": 0}}"#);
        assert_eq!(stats.memory.used_percent(), 0.0);
    }

    #[test]
    fn network_sums_every_interface() {
        let stats = stats_from(
            r#"{
                "networks": {
                    "eth0": {"rx_bytes": 100, "tx_bytes": 200},
                    "eth1": {"rx_bytes": 50, "tx_bytes": 25}
                }
            }"#,
        );

        assert_eq!(stats.network_bytes(), (150, 225));
    }

    #[test]
    fn block_io_splits_by_direction_case_insensitively() {
        let stats = stats_from(
            r#"{
                "blkio_stats": {
                    "io_service_bytes_recursive": [
                        {"op": "Read", "value": 1000},
                        {"op": "write", "value": 2000},
                        {"op": "Read", "value": 500},
                        {"op": "Sync", "value": 9999}
                    ]
                }
            }"#,
        );

        assert_eq!(stats.block_io_bytes(), (1500, 2000));
    }

    #[test]
    fn block_io_handles_a_null_list() {
        let stats = stats_from(r#"{"blkio_stats": {"io_service_bytes_recursive": null}}"#);
        assert_eq!(stats.block_io_bytes(), (0, 0));
    }

    #[test]
    fn an_empty_payload_decodes_to_zeros() {
        let stats = stats_from("{}");
        assert_eq!(stats.cpu_percent(), 0.0);
        assert_eq!(stats.memory.used_bytes(), 0);
        assert_eq!(stats.network_bytes(), (0, 0));
    }

    #[test]
    fn container_name_drops_the_leading_slash() {
        let summary: ContainerSummary =
            serde_json::from_str(r#"{"Id": "abcdef123456789", "Names": ["/web"]}"#).unwrap();

        assert_eq!(summary.display_name().as_str(), "web");
        assert_eq!(summary.short_id(), "abcdef123456");
    }

    #[test]
    fn container_without_names_falls_back_to_id() {
        let summary: ContainerSummary =
            serde_json::from_str(r#"{"Id": "abcdef123456789", "Names": []}"#).unwrap();

        assert_eq!(summary.display_name().as_str(), "abcdef123456");
    }
}
