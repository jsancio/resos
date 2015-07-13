use libprocess::UPID;
use protobuf;
use proto::{MasterInfo};
use zookeeper::{Watcher, WatchedEvent, ZkError, ZkResult, ZooKeeper};
use std::str::FromStr;
use std::time::Duration;

static ZK_SESSION_TIMEOUT: u64 = 10000;

struct MasterDetectorWatcher;

impl Watcher for MasterDetectorWatcher {
    fn handle(&self, e: &WatchedEvent) {
        debug!("{:?}", e);
    }
}

pub struct MasterDetector {
    zk: ZooKeeper,
    master: Option<UPID>
}

impl MasterDetector {
    pub fn new(connect_string: &str) -> ZkResult<MasterDetector> {
        let zk = try!(ZooKeeper::connect(connect_string,
                                         Duration::from_secs(ZK_SESSION_TIMEOUT),
                                         MasterDetectorWatcher));
        Ok(MasterDetector{zk: zk, master: None})
    }

    pub fn start(&mut self) {
        match self.get_master() {
            Ok(master_info) => self.master = Some(FromStr::from_str(master_info.get_pid()).unwrap()),
            Err(e) => error!("Failed to find leader in ZK {}", e)
        }
    }

    fn get_master(&self) -> ZkResult<MasterInfo> {
        let children = try!(self.zk.get_children("/", true));
        
        let mut contenders: Vec<_> = children.iter()
                                             .filter(|child| child.starts_with("info"))
                                             .collect();
        contenders.sort();

        debug!("contenders -> {:?}", contenders);

        match contenders.first() {
            Some(leader) => {
                let leader_path = "/".to_string() + leader;
                let (data, _acl) = try!(self.zk.get_data(&leader_path, true));
                match protobuf::parse_from_bytes(&data) {
                    Ok(master_info) => Ok(master_info),
                    Err(err) => Err(ZkError::MarshallingError)
                }
            }
            None => Err(ZkError::NoNode)
        }
    }

    pub fn master(&self) -> Option<&UPID> {
        self.master.as_ref()
    }
}