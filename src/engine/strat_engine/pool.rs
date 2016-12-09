// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use std::vec::Vec;

use uuid::Uuid;
use devicemapper::Device;

use engine::EngineResult;
use engine::Pool;
use engine::Filesystem;
use engine::Dev;
use engine::Cache;

use super::blockdev::BlockDev;
use super::consts::*;

#[derive(Debug, Clone)]
pub struct StratFilesystem {
    pub name: String,
    pub thin_id: u32,
}

#[derive(Debug)]
pub struct StratPool {
    pub name: String,
    pub pool_uuid: Uuid,
    pub cache_devs: BTreeMap<Uuid, BlockDev>,
    pub block_devs: BTreeMap<Uuid, BlockDev>,
    pub filesystems: BTreeMap<String, Box<StratFilesystem>>,
    pub raid_level: u16,
}

impl StratPool {
    pub fn new(name: &str,
               devices: BTreeSet<Device>,
               raid_level: u16,
               force: bool)
               -> EngineResult<StratPool> {
        let pool_uuid = Uuid::new_v4();
        let bds = try!(BlockDev::initialize(&pool_uuid, devices, MIN_MDA_SIZE, force));

        Ok(StratPool {
            name: name.to_owned(),
            pool_uuid: pool_uuid,
            cache_devs: BTreeMap::new(),
            block_devs: bds,
            filesystems: BTreeMap::new(),
            raid_level: raid_level,
        })
    }
}

impl Pool for StratPool {
    fn create_filesystem(&mut self,
                         _filesystem_name: &str,
                         _mount_point: &str,
                         _quota_size: Option<u64>)
                         -> EngineResult<()> {
        Ok(())
    }

    fn create_snapshot(&mut self, _snapshot_name: &str, _source: &str) -> EngineResult<()> {
        unimplemented!()
    }

    fn add_blockdevs(&mut self, paths: &[&Path], force: bool) -> EngineResult<Vec<PathBuf>> {
        let mut devices = BTreeSet::new();
        for path in paths {
            let dev = try!(Device::from_str(&path.to_string_lossy()));
            devices.insert(dev);
        }

        let mut bds = try!(BlockDev::initialize(&self.pool_uuid, devices, MIN_MDA_SIZE, force));
        let bdev_paths = bds.iter().map(|p| p.1.devnode.clone()).collect();
        self.block_devs.append(&mut bds);
        Ok(bdev_paths)
    }

    fn add_cachedevs(&mut self, paths: &[&Path], force: bool) -> EngineResult<Vec<PathBuf>> {
        let mut devices = BTreeSet::new();
        for path in paths {
            let dev = try!(Device::from_str(&path.to_string_lossy()));
            devices.insert(dev);
        }

        let mut bds = try!(BlockDev::initialize(&self.pool_uuid, devices, MIN_MDA_SIZE, force));
        let bdev_paths = bds.iter().map(|p| p.1.devnode.clone()).collect();
        self.cache_devs.append(&mut bds);
        Ok(bdev_paths)
    }

    fn destroy_filesystem(&mut self, _filesystem: &str) -> EngineResult<()> {
        unimplemented!()
    }

    fn filesystems(&mut self) -> BTreeMap<&Uuid, &mut Filesystem> {
        unimplemented!()
    }

    fn remove_blockdev(&mut self, _path: &Path) -> EngineResult<()> {
        unimplemented!()
    }

    fn remove_cachedev(&mut self, _path: &Path) -> EngineResult<()> {
        unimplemented!()
    }

    fn blockdevs(&mut self) -> Vec<&mut Dev> {
        unimplemented!()
    }

    fn cachedevs(&mut self) -> Vec<&mut Cache> {
        unimplemented!()
    }

    fn get_filesystem(&mut self, _id: &Uuid) -> EngineResult<&mut Filesystem> {
        unimplemented!()
    }

    fn get_filesystem_id(&self, _name: &str) -> EngineResult<Uuid> {
        unimplemented!()
    }

    fn get_filesystem_by_name(&mut self, _name: &str) -> EngineResult<&mut Filesystem> {
        unimplemented!()
    }
}