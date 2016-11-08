use cbor::{CborBytes};
use std::collections::btree_map::BTreeMap;
use time::{now, Timespec};
use std::ffi::OsStr;
#[cfg(feature = "filesystem")] use std::io::{Cursor, Write};
#[cfg(feature = "filesystem")] use libc::{ENOENT, EIO};
#[cfg(feature = "filesystem")] use fuse::*;


#[cfg(feature = "filesystem")]
const TTL: Timespec = Timespec { sec: 1, nsec: 0 }; // 1 second

#[derive(PartialEq, Clone, Debug, RustcDecodable, RustcEncodable)]
pub enum AttachmentType {
    File,
    Symlink,
    Directory,
}

impl AttachmentType {
    #[cfg(feature = "filesystem")]
    pub fn to_fuse(&self) -> FileType {
        match *self {
            AttachmentType::File => FileType::RegularFile,
            AttachmentType::Symlink => FileType::Symlink,
            AttachmentType::Directory => FileType::Directory,
        }
    }
}

#[derive(PartialEq, Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Attachment {
    pub children: BTreeMap<String, u64>,
    pub parent: u64,
    pub kind: AttachmentType,
    pub mtime: u64,
    pub ctime: u64,
    pub uid: u32,
    pub gid: u32,
    pub perm: u16,
    pub content: CborBytes,
}

impl Attachment {
    pub fn new() -> Attachment {
        let ts = now().to_timespec().sec as u64;
        Attachment {
            children: BTreeMap::new(),
            parent: 1,
            kind: AttachmentType::Directory,
            mtime: ts,
            ctime: ts,
            uid: 0,
            gid: 0,
            perm: 0o777,
            content: CborBytes(Vec::new()),
        }
    }

    #[cfg(feature = "filesystem")]
    pub fn to_attr(&self, id: u64) -> FileAttr {
        FileAttr {
            ino: id,
            size: self.content.len() as u64,
            blocks: 1,
            atime: Timespec { sec: self.mtime as i64, nsec: 0 },
            mtime: Timespec { sec: self.mtime as i64, nsec: 0 },
            ctime: Timespec { sec: self.ctime as i64, nsec: 0 },
            crtime: Timespec { sec: self.ctime as i64, nsec: 0 },
            kind: self.kind.to_fuse(),
            perm: self.perm,
            nlink: 1,
            uid: self.uid,
            gid: self.gid,
            rdev: 0,
            flags: 0,
        }
    }
}
#[derive(PartialEq, Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Attachments {
    pub nodes: BTreeMap<u64, Attachment>,
    pub root: u64,
    pub max: u64,
}

impl Attachments {
    pub fn new() -> Attachments {
        let mut nodes = BTreeMap::new();
        nodes.insert(1, Attachment::new());
        Attachments {
            nodes: nodes,
            root: 1,
            max: 1,
        }
    }

    fn create(&mut self, parent: u64, name: &OsStr, mode: u32, kind: AttachmentType, _flags: u32) -> Option<(u64, &mut Attachment)> {
        if let Some(mut n) = self.nodes.remove(&parent) {
            if n.kind != AttachmentType::Directory {
                return None
            }
            let mut child = Attachment::new();
            child.parent = parent;
            child.kind = kind;
            child.perm = mode as u16;
            let id = self.max;
            self.max += 1;
            self.nodes.insert(id, child);
            n.children.insert(name.to_str().unwrap_or("__ERROR__").to_owned(), id);
            self.nodes.insert(parent, n);
            Some((id, self.nodes.get_mut(&id).unwrap()))
        } else {
            None
        }
    }
}

#[cfg(feature = "filesystem")]
impl Filesystem for Attachments {
     fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
         if let Some(n) = self.nodes.get(&parent) {
             if n.kind != AttachmentType::Directory {
                 reply.error(ENOENT);
                 return
             }
             if let Some((cid, c)) = n.children.get(name.to_str().unwrap_or("__ERROR__"))
                                     .and_then(|cid| self.nodes.get(cid).map(|c| (cid, c))) {
                 reply.entry(&TTL, &c.to_attr(*cid), 0);
             } else {
                 reply.error(ENOENT);
             }
         } else {
             reply.error(ENOENT);
         }
     }

     fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
         if let Some(n) = self.nodes.get(&ino) {
             reply.attr(&TTL, &n.to_attr(ino));
         } else {
             reply.error(ENOENT);
         }
     }

     fn setattr(&mut self, _req: &Request, ino: u64, _mode: Option<u32>, uid: Option<u32>, gid: Option<u32>, size: Option<u64>, _atime: Option<Timespec>, mtime: Option<Timespec>, _fh: Option<u64>, _crtime: Option<Timespec>, _chgtime: Option<Timespec>, _bkuptime: Option<Timespec>, flags: Option<u32>, reply: ReplyAttr) {
         if let Some(n) = self.nodes.get_mut(&ino) {
             if let Some(uid_v) = uid {
                 n.uid = uid_v;
             }
             if let Some(gid_v) = gid {
                 n.gid = gid_v;
             }
             if let Some(Timespec { sec: mtime_v, nsec: _ }) = mtime {
                 n.mtime = mtime_v as u64;
             }
             if let Some(size_v) = size {
                 let CborBytes(ref mut cont_vec) = n.content;
                 cont_vec.resize(size_v as usize, b'\0');
             }
             reply.attr(&TTL, &n.to_attr(ino));
         } else {
             reply.error(ENOENT);
         }
     }

     fn open(&mut self, _req: &Request, ino: u64, flags: u32, reply: ReplyOpen) {
         if let Some(n) = self.nodes.get_mut(&ino) {
             reply.opened(0, 0);
         } else {
             reply.error(ENOENT);
         }
     }

     fn read(&mut self, _req: &Request, ino: u64, _fh: u64, offset: u64, _size: u32, reply: ReplyData) {
         if let Some(n) = self.nodes.get(&ino) {
             reply.data(&n.content[offset as usize..]);
         } else {
             reply.error(ENOENT);
         }
     }

     fn readdir(&mut self, _req: &Request, ino: u64, _fh: u64, offset: u64, mut reply: ReplyDirectory) {
         if let Some(n) = self.nodes.get(&ino) {
             if n.kind != AttachmentType::Directory {
                 reply.error(ENOENT);
                 return
             }
             if offset == 0 {
                 reply.add(ino, 0, FileType::Directory, ".");
                 reply.add(n.parent, 1, FileType::Directory, "..");
                 let mut idx = 2;
                 for (name, id) in &n.children {
                     if let Some(c) = self.nodes.get(&id) {
                         reply.add(*id, idx, c.kind.to_fuse(), name);
                         idx += 1;
                     }
                 }
             }
             reply.ok();
         } else {
             reply.error(ENOENT);
         }
     }

     fn readlink(&mut self, _req: &Request, ino: u64, reply: ReplyData) {
         if let Some(n) = self.nodes.get(&ino) {
             if n.kind != AttachmentType::Symlink {
                 reply.error(ENOENT);
                 return
             }
             reply.data(&n.content[..]);
         } else {
             reply.error(ENOENT);
         }
     }

     fn fsync(&mut self, _req: &Request, _ino: u64, _fh: u64, _datasync: bool, reply: ReplyEmpty) {
         reply.ok();
     }

     fn fsyncdir(&mut self, _req: &Request, _ino: u64, _fh: u64, _datasync: bool, reply: ReplyEmpty) {
         reply.ok();
     }

     fn mknod(&mut self, _req: &Request, parent: u64, name: &OsStr, mode: u32, _rdev: u32, reply: ReplyEntry) {
         // TODO: check mode for FIFO, devices etc.
         if let Some((cid, c)) = self.create(parent, name, mode, AttachmentType::File, 0) {
             reply.entry(&TTL, &c.to_attr(cid), 0);
         } else {
             reply.error(ENOENT);
         }
     }

     fn mkdir(&mut self, _req: &Request, parent: u64, name: &OsStr, mode: u32, reply: ReplyEntry) {
        if let Some((cid, c)) = self.create(parent, name, mode, AttachmentType::Directory, 0) {
            reply.entry(&TTL, &c.to_attr(cid), 0);
        } else {
            reply.error(ENOENT);
        }
     }

     fn create(&mut self, _req: &Request, parent: u64, name: &OsStr, mode: u32, flags: u32, reply: ReplyCreate) {
         if let Some((cid, c)) = self.create(parent, name, mode, AttachmentType::File, flags) {
             reply.created(&TTL, &c.to_attr(cid), 0, 0, flags);
         } else {
             reply.error(ENOENT);
         }
     }

     fn write(&mut self, _req: &Request, ino: u64, _fh: u64, offset: u64, data: &[u8], flags: u32, reply: ReplyWrite) {
         if let Some(mut n) = self.nodes.remove(&ino) {
             let CborBytes(cont_vec) = n.content;
             let mut cursor = Cursor::new(cont_vec);
             cursor.set_position(offset);
             if let Ok(()) = cursor.write_all(data) {
                reply.written(data.len() as u32);
             } else {
                reply.error(EIO);
             }
             n.content = CborBytes(cursor.into_inner());
             self.nodes.insert(ino, n);
         } else {
             reply.error(ENOENT);
         }
     }

     fn access(&mut self, _req: &Request, _ino: u64, _mask: u32, reply: ReplyEmpty) {
         // TODO
         reply.ok();
     }

     fn flush(&mut self, _req: &Request, _ino: u64, _fh: u64, _lock_owner: u64, reply: ReplyEmpty) {
         reply.ok();
     }

}
