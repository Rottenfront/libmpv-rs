use std::{
    collections::HashMap,
    ffi::{CStr, CString},
    ptr::{null, null_mut},
};

use crate::{raw::*, safe::util::make_c_string};

use super::util::{make_rust_string, make_rust_string_const};

#[derive(Debug, Clone, Copy)]
pub enum MpvFormat {
    String,
    OsdString,
    Flag,
    Int64,
    Float64,
}

impl MpvFormat {
    pub(crate) fn from_mpv_format(format: mpv_format) -> Option<Self> {
        match format {
            mpv_format_MPV_FORMAT_STRING => Some(Self::String),
            mpv_format_MPV_FORMAT_OSD_STRING => Some(Self::OsdString),
            mpv_format_MPV_FORMAT_FLAG => Some(Self::Flag),
            mpv_format_MPV_FORMAT_INT64 => Some(Self::Int64),
            mpv_format_MPV_FORMAT_DOUBLE => Some(Self::Float64),
            _ => None,
        }
    }

    pub(crate) fn to_mpv_format(self) -> mpv_format {
        match self {
            Self::String => mpv_format_MPV_FORMAT_STRING,
            Self::OsdString => mpv_format_MPV_FORMAT_OSD_STRING,
            Self::Flag => mpv_format_MPV_FORMAT_FLAG,
            Self::Int64 => mpv_format_MPV_FORMAT_INT64,
            Self::Float64 => mpv_format_MPV_FORMAT_DOUBLE,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    /// It returns the raw property string, like using ${=property} in input.conf (see input.rst).
    ///
    /// Warning: although the encoding is usually UTF-8, this is not always the case. File tags
    ///          often store strings in some legacy codepage, and even filenames don't necessarily
    ///          have to be in UTF-8 (at least on Linux). If you pass the strings to code that
    ///          requires valid UTF-8, you have to sanitize it in some way. On Windows, filenames
    ///          are always UTF-8, and libmpv converts between UTF-8 and UTF-16 when using win32
    ///          API functions. See the "Encoding of filenames" section for details.
    String(String),
    /// It returns the OSD property string, like using ${property} in input.conf (see input.rst).
    /// In many cases, this is the same as the raw string, but in other cases it's formatted for
    /// display on OSD. It's intended to be human readable. Do not attempt to parse these strings.
    ///
    /// Only valid when doing read access. The rest works like MpvNode::String.
    OsdString(String),
    /// Flag node. In origin is had either 0 or 1 value, and there is no reason to keep it
    /// as integer type.
    Flag(bool),
    Int64(i64),
    Float64(f64),
    Array(Vec<Node>),
    ByteArray(Vec<u8>),
    Map(HashMap<String, Node>),
    Node(Box<Node>),
}

impl Node {
    pub(crate) fn from_mpv_node(node: mpv_node) -> Option<Node> {
        let format = node.format;
        match format {
            mpv_format_MPV_FORMAT_STRING => {
                let data = unsafe { node.u.string };
                let Some(data) = make_rust_string(data) else {
                    return None;
                };
                Some(Node::String(data))
            }
            mpv_format_MPV_FORMAT_OSD_STRING => {
                let data = unsafe { node.u.string };
                let Some(data) = make_rust_string(data) else {
                    return None;
                };
                Some(Node::OsdString(data))
            }
            mpv_format_MPV_FORMAT_FLAG => {
                let data = unsafe { node.u.flag };
                let data = data != 0;
                Some(Node::Flag(data))
            }
            mpv_format_MPV_FORMAT_INT64 => {
                let data = unsafe { node.u.int64 };
                Some(Node::Int64(data))
            }
            mpv_format_MPV_FORMAT_DOUBLE => {
                let data = unsafe { node.u.double_ };
                Some(Node::Float64(data))
            }
            mpv_format_MPV_FORMAT_BYTE_ARRAY => {
                let data = unsafe { node.u.ba };
                if data == null_mut() {
                    return None;
                }
                let mpv_byte_array { data, size } = unsafe { *data };
                let data = unsafe { Vec::from_raw_parts(data as *mut u8, size, size) };
                Some(Node::ByteArray(data))
            }
            mpv_format_MPV_FORMAT_NODE_ARRAY => {
                let data = unsafe { node.u.list };
                if data == null_mut() {
                    return None;
                }
                let mut arr = vec![];
                let mpv_node_list { num, values, keys } = unsafe { *data };
                if values != null_mut() {
                    for i in 0..num as usize {
                        let ptr = unsafe { values.add(i) };
                        if let Some(node) = Node::from_mpv_node(unsafe { *ptr }) {
                            arr.push(node);
                        }
                    }
                }

                Some(Node::Array(arr))
            }
            mpv_format_MPV_FORMAT_NODE_MAP => {
                let data = unsafe { node.u.list };
                if data == null_mut() {
                    return None;
                }
                let mut map = HashMap::new();
                let mpv_node_list { num, values, keys } = unsafe { *data };
                if values != null_mut() {
                    for i in 0..num as usize {
                        let ptr = unsafe { values.add(i) };
                        let Some(node) = Node::from_mpv_node(unsafe { *ptr }) else {
                            continue;
                        };
                        let key_ptr = unsafe { keys.add(i) };
                        if let Some(key) = make_rust_string(unsafe { *key_ptr }) {
                            map.insert(key, node);
                        }
                    }
                }

                Some(Node::Map(map))
            }
            _ => None,
        }
    }

    pub(crate) fn to_mpv_node(self) -> Option<mpv_node> {
        use crate::raw::mpv_node__bindgen_ty_1 as node_union;
        let mut node = mpv_node {
            u: node_union { flag: 0 },
            format: mpv_format_MPV_FORMAT_FLAG,
        };
        match self {
            Node::String(s) => {
                let Some(data) = make_c_string(s) else {
                    return None;
                };
                node.u = node_union { string: data };
                node.format = mpv_format_MPV_FORMAT_STRING;
            }
            Node::OsdString(s) => {
                let Some(data) = make_c_string(s) else {
                    return None;
                };
                node.u = node_union { string: data };
                node.format = mpv_format_MPV_FORMAT_OSD_STRING;
            }
            Node::Flag(flag) => {
                node.u = node_union {
                    flag: if flag { 1 } else { 0 },
                };
            }
            Node::Int64(int) => {
                node.u = node_union { int64: int };
                node.format = mpv_format_MPV_FORMAT_INT64;
            }
            Node::Float64(float) => {
                node.u = node_union { double_: float };
                node.format = mpv_format_MPV_FORMAT_DOUBLE;
            }
            Node::Array(vec) => {
                let mut data = vec![];
                for n in vec {
                    if let Some(n) = n.to_mpv_node() {
                        data.push(n);
                    }
                }
                let size = data.len();
                let ptr = data.leak().as_mut_ptr();
                let list = Box::into_raw(Box::new(mpv_node_list {
                    num: size as _,
                    values: ptr,
                    keys: null_mut(),
                }));
                node.format = mpv_format_MPV_FORMAT_NODE_ARRAY;
                node.u = node_union { list };
            }
            Node::ByteArray(vec) => {
                let size = vec.len();
                let ptr = vec.leak().as_mut_ptr();
                let ba = Box::into_raw(Box::new(mpv_byte_array {
                    data: ptr as *mut _,
                    size,
                }));
                node.format = mpv_format_MPV_FORMAT_BYTE_ARRAY;
                node.u = node_union { ba };
            }
            Node::Map(map) => {
                let mut data = vec![];
                let mut keys = vec![];
                for (key, n) in map {
                    if let Some(n) = n.to_mpv_node() {
                        data.push(n);
                        keys.push(make_c_string(key).unwrap());
                    }
                }
                let size = data.len();
                let values = data.leak().as_mut_ptr();
                let keys = keys.leak().as_mut_ptr();
                let list = Box::into_raw(Box::new(mpv_node_list {
                    num: size as _,
                    values,
                    keys,
                }));
                node.format = mpv_format_MPV_FORMAT_NODE_MAP;
                node.u = node_union { list };
            }
            Node::Node(_) => unreachable!(),
        }
        Some(node)
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Property {
    /// Name of the property.
    pub name: String,
    /// Data of the property.
    pub data: Option<Node>,
}

impl Property {
    pub(crate) fn from_mpv_property(property: mpv_event_property) -> Option<Self> {
        let Some(name) = make_rust_string_const(property.name) else {
            return None;
        };
        let mut res = Self { name, data: None };
        if property.data == null_mut() {
            return Some(res);
        }
        let node = mpv_node {
            format: property.format,
            u: unsafe { *(property.data as *mut mpv_node__bindgen_ty_1) },
        };
        res.data = Node::from_mpv_node(node);
        Some(res)
    }
}
