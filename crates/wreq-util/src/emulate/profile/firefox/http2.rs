use wreq::http2::{
    HeadersDependency, HeadersPriority, Http2Options, Priorities, Priority, PseudoId, PseudoOrder,
    SettingId, SettingsOrder, StreamDependency, StreamId,
};

#[derive(Clone, Copy)]
pub(super) enum Preset {
    NoPush,
    PriorityTree,
    ZeroConcurrentStreams,
    AndroidWindow,
}

impl Preset {
    pub(super) fn build(self) -> Http2Options {
        let (initial_window_size, header_table_size) = if matches!(self, Self::AndroidWindow) {
            (32_768, 4_096)
        } else {
            (131_072, 65_536)
        };

        let builder = Http2Options::builder()
            .initial_window_size(initial_window_size)
            .max_frame_size(16_384)
            .initial_connection_window_size(12_517_377 + 65_535)
            .header_table_size(header_table_size)
            .headers_pseudo_order(
                PseudoOrder::builder()
                    .extend([
                        PseudoId::Method,
                        PseudoId::Path,
                        PseudoId::Authority,
                        PseudoId::Scheme,
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

        // The priority-tree preset omits SETTINGS_ENABLE_PUSH.
        if matches!(self, Self::PriorityTree) {
            return builder
                .initial_stream_id(15)
                .headers_priority(HeadersPriority::new(
                    HeadersDependency::Declared(StreamId::from(13)),
                    41,
                    false,
                ))
                .priorities(
                    Priorities::builder()
                        .extend([
                            Priority::new(
                                StreamId::from(3),
                                StreamDependency::new(StreamId::from(0), 200, false),
                            ),
                            Priority::new(
                                StreamId::from(5),
                                StreamDependency::new(StreamId::from(0), 100, false),
                            ),
                            Priority::new(
                                StreamId::from(7),
                                StreamDependency::new(StreamId::from(0), 0, false),
                            ),
                            Priority::new(
                                StreamId::from(9),
                                StreamDependency::new(StreamId::from(7), 0, false),
                            ),
                            Priority::new(
                                StreamId::from(11),
                                StreamDependency::new(StreamId::from(3), 0, false),
                            ),
                            Priority::new(
                                StreamId::from(13),
                                StreamDependency::new(StreamId::from(0), 240, false),
                            ),
                        ])
                        .build(),
                )
                .build();
        }

        let mut builder = builder
            .initial_stream_id(3)
            .enable_push(false)
            .headers_priority(HeadersPriority::new(HeadersDependency::Root, 41, false));
        if matches!(self, Self::ZeroConcurrentStreams) {
            builder = builder.max_concurrent_streams(0);
        }
        builder.build()
    }
}
