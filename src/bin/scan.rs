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
                if node.tags.contains("power", "tower")
                //    node.tags.contains("structure", "tubular")
                {
                    poles.insert(node.id, node);
                }
            },
            _ => (),
        }
    //     if obj.is_node()
    //         && obj.tags().contains("power", "tower")
    //         && obj.tags().contains_key("colour")
    //         // && obj.tags().get("colour").unwrap().contains("blue")
    //         && obj.tags().contains("structure", "tubular")
    //     {
    //         // println!("{:?} {}", obj, osm_viewer_url(&obj));
    //         poles.insert(obj.id());
    //     }
    //     // if obj.is_way() &&
    //     //    obj.tags().contains("power", "line") &&
    //     //    obj.tags().contains("voltage", "220000")
    //     // {

    //     // }
    }

    rdr.rewind()?;
    println!("scan ways...");
    let mut ways_of_interest: Vec<Way> = Vec::new();
    for obj in rdr.par_iter().map(Result::unwrap) {
        // println!("{:?}", obj);
        match obj {
            OsmObj::Way(way) => {
                if way.tags.contains("power", "line") &&
                   way.tags.contains("voltage", "220000") &&
                   way.nodes.iter().any(|n| poles.contains_key(n))
                {
                    ways_of_interest.push(way);
                }
            },
            _ => (),
        }
    }

    let pc = PrettyConfig::new();
    ron::ser::to_writer_pretty(File::create("poles.ron")?, &poles, pc.clone())?;
    ron::ser::to_writer_pretty(File::create("ways_of_interest.ron")?, &ways_of_interest, pc.clone())?;

    Ok(())
}
