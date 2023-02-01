// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use log::warn;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::str;
use std::{
    collections::{btree_map::Entry, BTreeMap},
    convert::TryInto,
};
use tokio::fs::{self, read_dir, DirBuilder, File, OpenOptions};
use tokio::io::{self, AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

const DELETE_FILE_INTERVAL: u64 = 8;
const INDEX_NAME: &str = "index";

#[derive(Debug, Clone, Copy)]
pub enum LogType {
    Skip = 0,
    Propose = 1,
    QuorumVotes = 2,
    FinalizeBlock = 3,
}

impl From<u8> for LogType {
    fn from(s: u8) -> LogType {
        match s {
            1 => LogType::Propose,
            2 => LogType::QuorumVotes,
            3 => LogType::FinalizeBlock,
            _ => LogType::Skip,
        }
    }
}

pub struct Wal {
    height_fs: BTreeMap<u64, File>,
    dir: String,
    current_height: u64,
    ifile: File,
}

impl Wal {
    async fn delete_old_file(dir: &str, current_height: u64) -> io::Result<()> {
        let mut read_dir = read_dir(dir).await?;
        while let Some(entry) = read_dir.next_entry().await? {
            let fpath = entry.path();
            if let Some(fname) = fpath.file_name() {
                let strs: Vec<&str> = fname.to_str().unwrap().split('.').collect();
                if !strs.is_empty() {
                    let num = strs[0].parse::<u64>().unwrap_or(current_height);
                    if num + DELETE_FILE_INTERVAL < current_height {
                        fs::remove_file(fpath).await?;
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn create(dir: &str) -> io::Result<Wal> {
        let fss = read_dir(dir).await;
        if fss.is_err() {
            DirBuilder::new().recursive(true).create(dir).await?;
        }

        let file_path = dir.to_string() + "/" + INDEX_NAME;
        let mut ifs = OpenOptions::new()
            .read(true)
            .create(true)
            .write(true)
            .open(file_path)
            .await?;
        ifs.rewind().await?;

        let mut string_buf: String = String::new();
        let res_fsize = ifs.read_to_string(&mut string_buf).await?;
        let num_str = string_buf.trim();
        let cur_height: u64;
        let last_file_path: String;
        if res_fsize == 0 {
            last_file_path = dir.to_string() + "/1.log";
            cur_height = 1;
        } else {
            let hi_res = num_str.parse::<u64>();
            if let Ok(hi) = hi_res {
                cur_height = hi;
                last_file_path = dir.to_string() + "/" + cur_height.to_string().as_str() + ".log"
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "index file data wrong",
                ));
            }
        }

        Self::delete_old_file(dir, cur_height).await?;

        let fs = OpenOptions::new()
            .read(true)
            .create(true)
            .write(true)
            .open(last_file_path)
            .await?;

        let mut tmp = BTreeMap::new();
        tmp.insert(cur_height, fs);

        Ok(Wal {
            height_fs: tmp,
            dir: dir.to_string(),
            current_height: cur_height,
            ifile: ifs,
        })
    }

    fn get_file_path(dir: &str, height: u64) -> String {
        let mut name = height.to_string();
        name += ".log";
        let pathname = dir.to_string() + "/";
        pathname + &*name
    }

    async fn set_index_file(&mut self, height: u64) -> io::Result<u64> {
        self.current_height = height;
        self.ifile.rewind().await?;
        let hstr = height.to_string();
        let content = hstr.as_bytes();
        let len = content.len() as u64;
        self.ifile.set_len(len).await?;
        self.ifile.write_all(content).await?;
        self.ifile.sync_data().await?;
        Ok(len)
    }

    pub async fn set_height(&mut self, height: u64) -> io::Result<u64> {
        let len = self.set_index_file(height).await?;
        if let Entry::Vacant(entry) = self.height_fs.entry(height) {
            let filename = Wal::get_file_path(&self.dir, height);
            let fs = OpenOptions::new()
                .create(true)
                .read(true)
                .append(true)
                .open(filename)
                .await?;
            entry.insert(fs);
        }

        if height > DELETE_FILE_INTERVAL {
            let newer_file = self.height_fs.split_off(&(height - DELETE_FILE_INTERVAL));
            for &i in self.height_fs.keys() {
                let delfilename = Wal::get_file_path(&self.dir, i);
                let _ = fs::remove_file(delfilename).await;
            }
            self.height_fs = newer_file;
        }
        Ok(len)
    }

    pub async fn save(&mut self, height: u64, log_type: LogType, msg: &[u8]) -> io::Result<u64> {
        let mtype = log_type as u8;
        let mlen = msg.len() as u32;
        if mlen == 0 {
            return Ok(0);
        }

        if height > self.current_height && height < self.current_height + DELETE_FILE_INTERVAL {
            if let Entry::Vacant(entry) = self.height_fs.entry(height) {
                let filename = Wal::get_file_path(&self.dir, height);
                let fs = OpenOptions::new()
                    .read(true)
                    .create(true)
                    .write(true)
                    .open(filename)
                    .await?;
                entry.insert(fs);
            }
        }

        let mut hlen = 0;
        if let Some(fs) = self.height_fs.get_mut(&height) {
            let len_bytes: [u8; 4] = mlen.to_le_bytes();
            let type_bytes: [u8; 1] = [mtype];
            let check_sum = calculate_hash(&msg);
            fs.seek(io::SeekFrom::End(0)).await?;
            fs.write_all(&len_bytes[..]).await?;
            fs.write_all(&type_bytes[..]).await?;
            fs.write_all(&check_sum.to_le_bytes()).await?;
            hlen = fs.write(msg).await?;
            fs.flush().await?;
        } else {
            warn!(
                "wal not save height {} current height {} ",
                height, self.current_height
            );
        }
        let _ = self.set_height(height).await;
        Ok(hlen as u64)
    }

    pub fn get_cur_height(&self) -> u64 {
        self.current_height
    }

    pub async fn load(&mut self) -> Vec<(u8, Vec<u8>)> {
        let mut vec_buf: Vec<u8> = Vec::new();
        let mut vec_out: Vec<(u8, Vec<u8>)> = Vec::new();
        let cur_height = self.current_height;
        if self.height_fs.is_empty() || cur_height == 0 {
            return vec_out;
        }

        for (height, fs) in &mut self.height_fs {
            if *height < self.current_height {
                continue;
            }
            fs.rewind().await.unwrap();
            let res_fsize = fs.read_to_end(&mut vec_buf).await;
            if res_fsize.is_err() {
                return vec_out;
            }
            let fsize = res_fsize.unwrap();
            let mut index = 0;

            loop {
                if index + 13 > fsize {
                    break;
                }
                let bytes = match vec_buf[index..index + 4].try_into() {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        warn!("wal file may be corrupted in len_bytes: {}", e);
                        return Vec::new();
                    }
                };
                let tmp = u32::from_le_bytes(bytes);
                let bodylen = tmp as usize;
                let mtype = vec_buf[index + 4];

                let bytes = match vec_buf[index + 5..index + 13].try_into() {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        warn!("wal file may be corrupted in check_sum: {}", e);
                        return Vec::new();
                    }
                };
                let saved_crc = u64::from_le_bytes(bytes);
                index += 13;
                if index + bodylen > fsize {
                    break;
                }

                let msg = &vec_buf[index..index + bodylen];
                let check_sum = calculate_hash(&msg);

                if check_sum != saved_crc {
                    warn!(
                        "wal crc checked error saved {} check {}",
                        saved_crc, check_sum
                    );
                    break;
                }
                vec_out.push((mtype, msg.to_vec()));
                index += bodylen;
            }
        }
        vec_out
    }

    pub async fn clear_file(&mut self) -> io::Result<()> {
        self.height_fs.clear();
        let mut read_dir = read_dir(self.dir.clone()).await?;
        while let Some(entry) = read_dir.next_entry().await? {
            let fpath = entry.path();
            let fname = fpath.file_name().and_then(|f| f.to_str());
            if let Some(fname) = fname {
                if !fname.contains(INDEX_NAME) {
                    let _ = fs::remove_file(fpath).await;
                }
            }
        }
        Ok(())
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
