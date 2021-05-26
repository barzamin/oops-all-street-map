use std::{collections::{BTreeMap, HashMap}, fs::File, io::Write};

use anyhow::Result;
use maud::{DOCTYPE, PreEscaped, Render, html};
use osmpbfreader::{Node, NodeId, Way};
use ron::ser::PrettyConfig;
use serde_json::json;

// fn osm_viewer_url(obj: &OsmObj) -> String {
//     match obj.id() {
//         OsmId::Node(id) => format!("https://www.openstreetmap.org/node/{}", id.0),
//         OsmId::Way(id) => format!("https://www.openstreetmap.org/way/{}", id.0),
//         OsmId::Relation(id) => format!("https://www.openstreetmap.org/relation/{}", id.0),
//     }
// }

fn gmaps_url(node: &Node) -> String {
    format!("https://www.google.com/maps/place/{},{}", node.lat(), node.lon())
}

fn main() -> Result<()> {
    let poles: BTreeMap<NodeId, Node> = ron::de::from_reader(File::open("poles.ron")?)?;
    let ways_of_interest: Vec<Way> = ron::de::from_reader(File::open("ways_of_interest.ron")?)?;

    let markup = html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                title {"ways of interest"}
                link rel="stylesheet" href="https://unpkg.com/leaflet@1.7.1/dist/leaflet.css" integrity="sha512-xodZBNTC5n17Xt2atTPuE1HxjVMSvLVW9ocqUKLsCC5CXdbqCmblAshOMAS6/keqq/sMZMZ19scR4PsZChSR7A==" crossorigin="";

                style {r#"
                    #globalmap {
                        height: 70vh;
                    }
                "#}
            }
            body {
                div#globalmap {}
                div#latlon {}

                h1 {"ways of interest"}

                @for way in &ways_of_interest {
                    h2 id=(format!("way{}", way.id.0)) {
                        "way "
                        code { (way.id.0) }

                        " " small {
                            @if let Some(name) = way.tags.get("name") {
                                (name)
                            } @else { "*" }
                        }
                    }

                    div {
                        a href=(format!("https://www.openstreetmap.org/way/{}", way.id.0)) { "osm waydetails" }
                    }

                    details {
                        summary { "raw" }
                        pre {
                            (ron::ser::to_string_pretty(way, PrettyConfig::new())?)
                        }
                    }

                    details {
                        summary { "attached nodes" }
                        ul {
                            @for node in &way.nodes {
                                @if let Some(pole) = poles.get(&node) {
                                    li {
                                        pre {
                                            (ron::ser::to_string_pretty(&pole, PrettyConfig::new())?)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            
                script src="https://unpkg.com/leaflet@1.7.1/dist/leaflet.js" integrity="sha512-XQoYMqMTK8LvdxXYG3nZ448hOEQiglfqkJs1NOQV44cWnUrBc8PkAOcXy20w0vlaXaVUearIOBhiXZ5V3ynxwA==" crossorigin="" {}
                
                script { "window.ways = new Map(" (PreEscaped(serde_json::to_string(
                    &ways_of_interest.iter().map(|w| {
                        let poles = w.nodes.iter().filter_map(|n| poles.get(n)).collect::<Vec<&Node>>();
                        (w.id.0, json!({"way": &w, "poles": poles}))
                    }).collect::<Vec<(i64, serde_json::Value)>>()
                )?)) ");" }
                script { (PreEscaped(std::fs::read_to_string("woof.js")?)) }
            }
        }
    };

    File::create("viz.html")?.write_all(markup.render().into_string().as_bytes())?;

    Ok(())
}
