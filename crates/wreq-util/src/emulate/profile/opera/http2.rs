use wreq::http2::{
    HeadersDependency, HeadersPriority, Http2Options, PseudoId, PseudoOrder, SettingId,
    SettingsOrder,
};

pub(super) fn no_push() -> Http2Options {
    Http2Options::builder()
        .initial_window_size(6291456)
        .initial_connection_window_size(15728640)
        .max_header_list_size(262144)
        .header_table_size(65536)
        .enable_push(false)
        .headers_priority(headers_priority())
        .headers_pseudo_order(pseudo_order())
        .settings_order(settings_order())
        .build()
}

fn headers_priority() -> HeadersPriority {
    HeadersPriority::new(HeadersDependency::Root, 255, true)
}

fn pseudo_order() -> PseudoOrder {
    PseudoOrder::builder()
        .extend([
            PseudoId::Method,
            PseudoId::Authority,
            PseudoId::Scheme,
            PseudoId::Path,
        ])
        .build()
}

fn settings_order() -> SettingsOrder {
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
        .build()
}
