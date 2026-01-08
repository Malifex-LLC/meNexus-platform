// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use serde::{Deserialize, Serialize};

/// Stream status
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum StreamStatus {
    Live,
    Offline,
    Starting,
    Ending,
}

/// Stream quality option
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct QualityOption {
    pub label: String,      // e.g., "1080p60", "720p", "480p", "Auto"
    pub bitrate: u32,       // kbps
    pub is_source: bool,    // Original quality
}

/// The streamer/host information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Streamer {
    pub id: String,
    pub handle: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub is_verified: bool,
    pub follower_count: u32,
    pub is_synapse_host: bool,
}

/// Stream metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StreamInfo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub tags: Vec<String>,
    pub started_at: Option<String>,
    pub thumbnail_url: Option<String>,
}

/// Live stream statistics
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StreamStats {
    pub viewer_count: u32,
    pub peak_viewers: u32,
    pub duration_seconds: u64,
    pub bitrate_kbps: u32,
    pub fps: u32,
    pub resolution: String,
    pub latency_ms: u32,
}

/// Complete livestream data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Livestream {
    pub info: StreamInfo,
    pub streamer: Streamer,
    pub stats: StreamStats,
    pub status: StreamStatus,
    pub quality_options: Vec<QualityOption>,
}

// ============================================================================
// Mock Data Generators
// ============================================================================

impl Streamer {
    pub fn mock_host() -> Self {
        Streamer {
            id: "streamer-1".to_string(),
            handle: "neo".to_string(),
            display_name: "Neo Anderson".to_string(),
            avatar_url: None,
            is_verified: true,
            follower_count: 12847,
            is_synapse_host: true,
        }
    }
}

impl StreamInfo {
    pub fn mock_live() -> Self {
        StreamInfo {
            id: "stream-001".to_string(),
            title: "🔴 Building the Future: P2P Infrastructure Deep Dive".to_string(),
            description: "Join me as we explore the architecture behind decentralized social networks. Today we're implementing CRDT-based document synchronization and discussing the challenges of eventually consistent systems. Bring your questions!".to_string(),
            category: "Software Development".to_string(),
            tags: vec![
                "Rust".to_string(),
                "P2P".to_string(),
                "Decentralized".to_string(),
                "CRDTs".to_string(),
                "Live Coding".to_string(),
            ],
            started_at: Some("2 hours ago".to_string()),
            thumbnail_url: None,
        }
    }

    pub fn mock_offline() -> Self {
        StreamInfo {
            id: "stream-000".to_string(),
            title: "Stream Offline".to_string(),
            description: "The host is not currently streaming. Check back later or enable notifications to know when they go live.".to_string(),
            category: "".to_string(),
            tags: vec![],
            started_at: None,
            thumbnail_url: None,
        }
    }
}

impl StreamStats {
    pub fn mock_live() -> Self {
        StreamStats {
            viewer_count: 342,
            peak_viewers: 489,
            duration_seconds: 7245, // ~2 hours
            bitrate_kbps: 6000,
            fps: 60,
            resolution: "1920x1080".to_string(),
            latency_ms: 1200,
        }
    }

    pub fn mock_offline() -> Self {
        StreamStats {
            viewer_count: 0,
            peak_viewers: 0,
            duration_seconds: 0,
            bitrate_kbps: 0,
            fps: 0,
            resolution: "".to_string(),
            latency_ms: 0,
        }
    }
}

impl QualityOption {
    pub fn mock_options() -> Vec<Self> {
        vec![
            QualityOption {
                label: "Source (1080p60)".to_string(),
                bitrate: 6000,
                is_source: true,
            },
            QualityOption {
                label: "1080p".to_string(),
                bitrate: 4500,
                is_source: false,
            },
            QualityOption {
                label: "720p60".to_string(),
                bitrate: 3500,
                is_source: false,
            },
            QualityOption {
                label: "720p".to_string(),
                bitrate: 2500,
                is_source: false,
            },
            QualityOption {
                label: "480p".to_string(),
                bitrate: 1500,
                is_source: false,
            },
            QualityOption {
                label: "360p".to_string(),
                bitrate: 800,
                is_source: false,
            },
            QualityOption {
                label: "Auto".to_string(),
                bitrate: 0,
                is_source: false,
            },
        ]
    }
}

impl Livestream {
    pub fn mock_live() -> Self {
        Livestream {
            info: StreamInfo::mock_live(),
            streamer: Streamer::mock_host(),
            stats: StreamStats::mock_live(),
            status: StreamStatus::Live,
            quality_options: QualityOption::mock_options(),
        }
    }

    pub fn mock_offline() -> Self {
        Livestream {
            info: StreamInfo::mock_offline(),
            streamer: Streamer::mock_host(),
            stats: StreamStats::mock_offline(),
            status: StreamStatus::Offline,
            quality_options: vec![],
        }
    }
}

