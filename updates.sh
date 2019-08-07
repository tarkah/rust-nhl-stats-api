#!/bin/bash

sed -i -E 's/crate::apis/crate::sync/g' src/sync/*.rs
sed -i -E 's/crate::apis/crate::async/g' src/async/*.rs
sed -i -E 's/id: f32/id: u32/g' src/sync/*.rs src/async/*.rs
sed -i -E -e '15,16s/^    /    \/\/ /g' -e '24s/epg/\/\/ epg/g' src/models/game_content_media.rs