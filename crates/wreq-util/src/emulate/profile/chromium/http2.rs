use wreq::http2::{
    HeadersDependency, HeadersPriority, Http2Options, PseudoId, PseudoOrder, SettingId,
    SettingsOrder,
};

#[derive(Clone, Copy)]
pub(super) enum Preset {
    StreamLimit,
    StreamLimitNoPush,
    NoPush,
}

impl Preset {
    pub(super) fn build(self) -> Http2Options {
        // Unset SETTINGS entries are omitted, not sent with a zero value.
        let (max_concurrent_streams, enable_push) = match self {
            Self::StreamLimit => (Some(1_000), None),
            Self::StreamLimitNoPush => (Some(1_000), Some(false)),
            Self::NoPush => (None, Some(false)),
        };

        let mut builder = Http2Options::builder()
            .initial_window_size(6_291_456)
            .initial_connection_window_size(15_728_640)
            .max_header_list_size(262_144)
            .header_table_size(65_536)
            .max_concurrent_streams(max_concurrent_streams)
            .headers_priority(HeadersPriority::new(HeadersDependency::Root, 255, true))
            .headers_pseudo_order(
                PseudoOrder::builder()
                    .extend([
                        PseudoId::Method,
                        PseudoId::Authority,
                        PseudoId::Scheme,
                        PseudoId::Path,
                    ])
                    .build(),
            )
            .settings_order(
                SettingsOrder::builder()
                    .extend([
                        SettingId::HeaderTableSize,
                        SettingId::EnablePush,
                        SettingId::MaxConcurrentStreams,
                        SettingId::InitialWindowSize,
                        SettingId::MaxFrameSize,
                        SettingId::MaxHeaderListSize,
                        SettingId::EnableConnectProtocol,
                        SettingId::NoRfc7540Priorities,
                    ])
                    .build(),
            );

        if let Some(enable_push) = enable_push {
            builder = builder.enable_push(enable_push);
        }
        builder.build()
    }
}
