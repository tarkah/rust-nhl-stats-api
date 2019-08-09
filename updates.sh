#!/bin/bash

# Rename modules
sed -i -E 's/crate::apis/crate::sync/g' src/sync/*.rs
sed -i -E 's/crate::apis/crate::async/g' src/async/*.rs

# Convert id parameters to int (to be fixed in openapi spec)
sed -i -E 's/id: f32/id: u32/g' src/sync/*.rs src/async/*.rs

# Fix duplicate crate::models on parameters referencing enums
sed -i -E 's/crate::models::crate::models::/crate::models::/g' src/sync/*.rs src/async/*.rs

# Add PartialEq, Clone to Enums
sed -i -E -e '13s/\)]/, PartialEq, Clone\)]/g' src/models/enum_standing_types.rs
sed -i -E -e '13s/\)]/, PartialEq, Clone\)]/g' src/models/enum_stat_types.rs
sed -i -E -e '13s/\)]/, PartialEq, Clone\)]/g' src/models/enum_expand_schedule.rs
sed -i -E -e '13s/\)]/, PartialEq, Clone\)]/g' src/models/enum_expand_teams.rs

# Flatten for array at base level and make object transparent
sed -i -E -e '15s/\)]/, flatten\)]/g' src/models/standing_types.rs
sed -i -E '14i\#[serde(transparent)]' src/models/standing_types.rs

# Flatten for array at base level and make object transparent
sed -i -E -e '15s/\)]/, flatten\)]/g' src/models/stat_types.rs
sed -i -E '14i\#[serde(transparent)]' src/models/stat_types.rs

# Flatten for array at base level and make object transparent
sed -i -E -e '15s/\)]/, flatten\)]/g' src/models/team_stats_stats.rs
sed -i -E '14i\#[serde(transparent)]' src/models/team_stats_stats.rs

# Manually add / correct type which needs to be enum
sed -i -E -e '15s/\)]/, flatten\)]/g' src/models/game_content_media.rs
cat <<EOF >> src/models/game_content_media.rs
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AnyOfGameMediaNhltvGameMediaAudioGameHighlightType {
    Nhltv(crate::models::GameMediaNhltv),
    Audio(crate::models::GameMediaAudio),
    Highlight(crate::models::GameHighlightType),
}
EOF
printf '\npub use self::game_content_media::AnyOfGameMediaNhltvGameMediaAudioGameHighlightType;' >> src/models/mod.rs

# Manually add / correct type which needs to be enum
sed -i -E -e '15s/\)]/, flatten\)]/g' src/models/team_stats_stats_splits.rs
cat <<EOF >> src/models/team_stats_stats_splits.rs
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AnyOfTeamStatsValuesTeamStatsRankings {
    Values(super::TeamStatsValues),
    Rankings(super::TeamStatsRankings),
}
EOF
printf '\npub use self::team_stats_stats_splits::AnyOfTeamStatsValuesTeamStatsRankings;' >> src/models/mod.rs