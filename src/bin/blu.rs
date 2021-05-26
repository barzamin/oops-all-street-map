use std::{collections::{BTreeMap, BTreeSet}, fs::File};

use osmpbfreader::{Node, NodeId, OsmId, OsmObj, OsmPbfReader, Way};
use anyhow::Result;
use ron::ser::PrettyConfig;

fn main() -> Result<()> {
    let mut rdr = OsmPbfReader::new(File::open("/mnt/torrents/power-infra.osm.pbf")?);
    let mut poles: BTreeMap<NodeId, Node> = BTreeMap::new();
    println!("find poles...");
    for obj in rdr.par_iter().map(Result::unwrap) {
        match obj {
            OsmObj::Node(node) => {
                if node.tags.contains("power", "tower") {
                    poles.insert(node.id, node);
                }
            },
            _ => (),
        }
    }

    rdr.rewind()?;
    println!("scan ways...");
    let mut ways_of_interest: Vec<Way> = Vec::new();
    for obj in rdr.par_iter().map(Result::unwrap) {
        match obj {
            OsmObj::Way(way) => {
                if way.tags.contains("power", "line") &&
                   way.tags.contains("voltage", "220000")
                {
                    for n in &way.nodes {
                        if let Some(pole) = poles.get(n) {
                            if pole.tags.contains_key("colour") {
                                println!("{:?}", pole);
                            }
                        }
                    }
                }
            },
            _ => (),
        }
    }

    Ok(())
}
