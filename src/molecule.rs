use alloc::vec;
use alloc::vec::Vec;

pub fn decode_dynvec(data: &[u8]) -> Vec<Vec<u8>> {
    assert!(data.len() >= 4);
    assert!(data.len() as u32 == u32::from_le_bytes(data[0..4].try_into().unwrap()));
    let nums = (u32::from_le_bytes(data[4..8].try_into().unwrap()) / 4 - 1) as usize;
    let mut head: Vec<usize> = vec![];
    for i in 0..nums {
        head.push(u32::from_le_bytes(data[i * 4 + 4..i * 4 + 8].try_into().unwrap()) as usize);
    }
    head.push(data.len());
    let mut body: Vec<Vec<u8>> = Vec::new();
    for i in 0..nums {
        body.push(data[head[i]..head[i + 1]].to_vec());
    }
    body
}

pub fn decode_fixvec(data: &[u8]) -> Vec<Vec<u8>> {
    assert!(data.len() >= 4);
    let icnt = u32::from_le_bytes(data[0..4].try_into().unwrap()) as usize;
    let mut body: Vec<Vec<u8>> = vec![];
    if icnt > 0 {
        let size = data[4..].len() as usize / icnt;
        for i in 0..icnt {
            body.push(data[4 + i * size..4 + i * size + size].to_vec());
        }
    }
    body
}

pub fn decode_seq(data: &[u8], size: &[usize]) -> Vec<Vec<u8>> {
    let mut r: Vec<Vec<u8>> = vec![];
    let mut s = 0;
    for n in size {
        r.push(data[s..s + n].to_vec());
        s += n;
    }
    r
}

pub fn encode_dynvec(data: Vec<Vec<u8>>) -> Vec<u8> {
    let mut head: Vec<u8> = vec![];
    let mut body: Vec<u8> = vec![];
    let head_size = 4 + 4 * data.len();
    let mut body_size = 0;
    for item in data {
        let size = head_size + body_size;
        let size = size as u32;
        head.extend(&size.to_le_bytes());
        body.extend(&item);
        body_size += item.len();
    }
    let size = head_size + body_size;
    let size = size as u32;
    [size.to_le_bytes().to_vec(), head, body].concat()
}

pub fn encode_fixvec(data: Vec<Vec<u8>>) -> Vec<u8> {
    let mut r = vec![];
    r.extend(&(data.len() as u32).to_le_bytes());
    for e in data {
        r.extend(&e);
    }
    r
}

pub fn encode_seq(data: Vec<Vec<u8>>) -> Vec<u8> {
    let mut r: Vec<u8> = vec![];
    for e in data {
        r.extend(&e)
    }
    r
}

pub struct Byte {
    data: u8,
}

impl Byte {
    pub fn new(data: u8) -> Self {
        Self { data }
    }

    pub fn molecule(&self) -> Vec<u8> {
        vec![self.data]
    }

    pub fn molecule_decode(data: &[u8]) -> u8 {
        assert_eq!(data.len(), 1);
        data[0]
    }

    pub fn molecule_size() -> usize {
        1
    }
}

pub struct Byte32 {
    data: [u8; 32],
}

impl Byte32 {
    pub fn new(data: [u8; 32]) -> Self {
        Self { data }
    }

    pub fn molecule(&self) -> Vec<u8> {
        self.data.to_vec()
    }

    pub fn molecule_decode(data: &[u8]) -> [u8; 32] {
        assert_eq!(data.len(), 32);
        let mut r = [0u8; 32];
        r.copy_from_slice(&data);
        r
    }

    pub fn molecule_size() -> usize {
        32
    }
}

pub struct Bytes {
    data: Vec<u8>,
}

impl Bytes {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn molecule(&self) -> Vec<u8> {
        let mut r = vec![];
        r.extend(&(self.data.len() as u32).to_le_bytes());
        r.extend(&self.data);
        r
    }

    pub fn molecule_decode(data: &[u8]) -> Vec<u8> {
        assert!(data.len() >= 4);
        let l = u32::from_le_bytes(data[0..4].try_into().unwrap());
        assert_eq!(l as usize, data.len() - 4);
        data[4..].to_vec()
    }
}

pub struct U32 {
    data: u32,
}

impl U32 {
    pub fn new(data: u32) -> Self {
        Self { data }
    }

    pub fn molecule(&self) -> Vec<u8> {
        self.data.to_le_bytes().to_vec()
    }

    pub fn molecule_decode(data: &[u8]) -> u32 {
        assert_eq!(data.len(), 4);
        u32::from_le_bytes(data.try_into().unwrap())
    }

    pub fn molecule_size() -> usize {
        4
    }
}

pub struct U64 {
    data: u64,
}

impl U64 {
    pub fn new(data: u64) -> Self {
        Self { data }
    }

    pub fn molecule(&self) -> Vec<u8> {
        self.data.to_le_bytes().to_vec()
    }

    pub fn molecule_decode(data: &[u8]) -> u64 {
        assert_eq!(data.len(), 8);
        u64::from_le_bytes(data.try_into().unwrap())
    }

    pub fn molecule_size() -> usize {
        8
    }
}

pub struct U128 {
    data: u128,
}

impl U128 {
    pub fn new(data: u128) -> Self {
        Self { data }
    }

    pub fn molecule(&self) -> Vec<u8> {
        self.data.to_le_bytes().to_vec()
    }

    pub fn molecule_decode(data: &[u8]) -> u128 {
        assert_eq!(data.len(), 16);
        u128::from_le_bytes(data.try_into().unwrap())
    }

    pub fn molecule_size() -> usize {
        16
    }
}
