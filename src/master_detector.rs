use libprocess::UPID;
use protobuf;
use proto::{MasterInfo};
use zookeeper::{Acl, CreateMode, Watcher, WatchedEvent, ZkError, ZooKeeper};
use std::str::FromStr;
use std::time::Duration;

static MASTER_DETECTOR_ZK_SESSION_TIMEOUT: u32 = 10000;

struct MasterDetectorWatcher;

impl Watcher for MasterDetectorWatcher {
    fn handle(&self, e: &WatchedEvent) {
        debug!("{:?}", e)
    }
}

pub struct MasterDetector {
    zk: ZooKeeper,
    master: Option<UPID>
}

impl MasterDetector {
    pub fn new(connect_string: &str, ) -> Result<MasterDetector, ZkError> {
        let zk = try!(ZooKeeper::connect(connect_string, Duration::from_secs(60), MasterDetectorWatcher));
        Ok(MasterDetector{zk: zk, master: None})
    }

    pub fn start(&mut self) {
        let children = self.zk.get_children("/", true).unwrap();
        
        let mut contenders: Vec<_> = children.iter()
                                             .filter(|child| child.starts_with("info"))
                                             .collect();
        contenders.sort();
        
        debug!("contenders -> {:?}", contenders);

        if let Some(leader) = contenders.first() {
            let leader_path = "/".to_string() + leader;
            let data = match self.zk.get_data(&leader_path, true) {
                Ok(data) => {
                    let master_info: MasterInfo = protobuf::parse_from_bytes(&data.0).unwrap();
                    debug!("data in master is {:?}", master_info);
                    self.master = Some(FromStr::from_str(master_info.get_pid()).unwrap());
                },
                Err(e) => panic!("Can't get leader data: {}", e)
            };
        } else {
            panic!("No leader in group!")
        };
    }

    pub fn master(&self) -> Option<&UPID> {
        self.master.as_ref()
    }
}