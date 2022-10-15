// serialize.rs -- demonstrate generating a DASH MPD in XML programatically from Rust
//
// Run with `cargo run --example serialize`

use std::time::Duration;
use chrono::prelude::*;
use env_logger::Env;
use serde::ser::Serialize;
use quick_xml::writer::Writer;
use quick_xml::se::Serializer;
use dash_mpd::{MPD, BaseURL, Representation, AdaptationSet, Period, ProgramInformation, Copyright};
use dash_mpd::{Title};


fn main () {
    env_logger::Builder::from_env(Env::default().default_filter_or("info,reqwest=warn")).init();

    let mut buffer = Vec::new();
    let writer = Writer::new_with_indent(&mut buffer, b' ', 2);
    let mut ser = Serializer::with_root(writer, Some("MPD"));

    let pi = ProgramInformation {
        Title: Some(Title { content: Some("My serialization example".into()) }),
        Copyright: Some(Copyright { content: Some("MIT Licenced".into()) }),
        lang: Some("eng".to_string()),
        moreInformationURL: Some("https://github.com/emarsden/dash-mpd-rs".into()),
        ..Default::default()
    };
    let rep1 = Representation {
        id: Some("1".to_string()),
        mimeType: Some("video/mp4".to_string()),
        codecs: Some("avc1.640028".to_string()),
        width: Some(1920),
        height: Some(800),
        bandwidth: Some(1980081),
        BaseURL: Some(BaseURL { base: "https://example.net/foobles/".to_string(), ..Default::default() }),
        ..Default::default()
    };
    let rep2 = Representation {
        id: Some("2".to_string()),
        mimeType: Some("video/mp4".to_string()),
        codecs: Some("hev1.1.6.L120.90".to_string()),
        width: Some(800),
        height: Some(600),
        bandwidth: Some(180081),
        BaseURL: Some(BaseURL { base: "https://example.net/foobles/".to_string(), ..Default::default() }),
        ..Default::default()
    };
    let adapt = AdaptationSet {
        id: Some(1),
        contentType: Some("video".to_string()),
        lang: Some("eng".to_string()),
        mimeType: Some("video/mp4".to_string()),
        codecs: Some("avc1.640028".to_string()),
        minWidth: Some(1920),
        minHeight: Some(800),
        frameRate: Some("15/2".to_string()),
        bitstreamSwitching: Some(true),
        representations: Some(vec!(rep1, rep2)),
        ..Default::default()
    };
    let period = Period {
        id: Some("1".to_string()),
        duration: Some(Duration::new(42, 0)),
        adaptations: Some(vec!(adapt)),
        ..Default::default()
    };
    let mpd = MPD {
        mpdtype: Some("static".to_string()),
        xmlns: Some("urn:mpeg:dash:schema:mpd:2011".to_string()),
        periods: vec!(period),
        ProgramInformation: Some(pi),
        publishTime: Some(Utc::now()),
        ..Default::default()
    };

    mpd.serialize(&mut ser)
        .expect("serializing MPD struct");
    let xml = String::from_utf8(buffer.clone()).unwrap();
    println!("{}", xml);
    // check round-trippability
    if let Err(e) = dash_mpd::parse(&xml) {
        eprintln!("Can't deserialize ou serialized XML: {:?}", e);
    }
}

