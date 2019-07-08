use libc::*;
use std::ptr;

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
pub enum EcBackendId {
    NULL = 0,
    JERASURE_RS_VAND = 1,
    JERASURE_RS_CAUCHY = 2,
    FLAT_XOR_HD = 3,
    ISA_L_RS_VAND = 4,
    SHSS = 5,
    LIBERASURECODE_RS_VAND = 6,
}

#[repr(C)]
pub enum EcChecksumType {
    NONE = 1,
    CRC32 = 2,
    MD5 = 3,
}

#[repr(C)]
pub struct EcArgs {
    pub k: c_int,
    pub m: c_int,
    pub w: c_int,
    pub hd: c_int,
    pub priv_args: [u64; 5],
    pub ct: EcChecksumType,
}

pub type Desc = c_int;
pub type ErrorCode = c_uint;

pub const EBACKENDNOTSUPP: u32 = 200;
pub const EECMETHODNOTIMPL: u32 = 201;
pub const EBACKENDINITERR: u32 = 202;
pub const EBACKENDINUSE: u32 = 203;
pub const EBACKENDNOTAVAIL: u32 = 204;
pub const EBADCHKSUM: u32 = 205;
pub const EINVALIDPARAMS: u32 = 206;
pub const EBADHEADER: u32 = 207;
pub const EINSUFFFRAGS: u32 = 208;

#[link(name = "erasurecode", kind = "static")]
#[link(name = "gf_complete", kind = "static")]
#[link(name = "z", kind = "static")]
#[link(name = "Jerasure", kind = "static")]
#[link(name = "Xorcode", kind = "static")]
extern "C" {
    /// Create a liberasurecode instance and return a descriptor
    /// for use with EC operations (encode, decode, reconstruct)
    ///
    /// @param id - one of the supported backends as
    ///        defined by ec_backend_id_t
    /// @param ec_args - arguments to the EC backend
    ///        arguments common to all backends
    ///          k - number of data fragments
    ///          m - number of parity fragments
    ///          w - word size, in bits
    ///          hd - hamming distance (=m for Reed-Solomon)
    ///          ct - fragment checksum type (stored with the fragment metadata)
    ///        backend-specific arguments
    ///          null_args - arguments for the null backend
    ///          flat_xor_hd, jerasure do not require any special args
    ///
    /// @return liberasurecode instance descriptor (int > 0)
    ///
    fn liberasurecode_instance_create(id: EcBackendId, args: *const EcArgs) -> Desc;

    /// Close a liberasurecode instance
    ///
    /// @param desc - liberasurecode descriptor to close
    ///
    /// @return 0 on success, otherwise non-zero error code
    ///
    fn liberasurecode_instance_destroy(desc: Desc) -> c_int;

    /// Erasure encode a data buffer
    ///
    /// @param desc - liberasurecode descriptor/handle
    ///        from liberasurecode_instance_create()
    /// @param orig_data - data to encode
    /// @param orig_data_size - length of data to encode
    /// @param encoded_data - pointer to _output_ array (char **) of k data
    ///        fragments (char *), allocated by the callee
    /// @param encoded_parity - pointer to _output_ array (char **) of m parity
    ///        fragments (char *), allocated by the callee
    /// @param fragment_len - pointer to _output_ length of each fragment, assuming
    ///        all fragments are the same length
    ///
    /// @return 0 on success, -error code otherwise
    ///
    fn liberasurecode_encode(
        desc: Desc,
        orig_data: *const u8,
        orig_data_size: u64,
        encoded_data: *mut *mut *mut u8,
        encoded_parity: *mut *mut *mut u8,
        fragment_len: *mut u64,
    ) -> c_int;

    /// Cleanup structures allocated by librasurecode_encode
    ///
    /// The caller has no context, so cannot safely free memory
    /// allocated by liberasurecode, so it must pass the
    /// deallocation responsibility back to liberasurecode.
    ///
    /// @param desc - liberasurecode descriptor/handle
    ///        from liberasurecode_instance_create()
    /// @param encoded_data - (char **) array of k data
    ///        fragments (char *), allocated by liberasurecode_encode
    /// @param encoded_parity - (char **) array of m parity
    ///        fragments (char *), allocated by liberasurecode_encode
    ///
    /// @return 0 in success; -error otherwise
    ///
    fn liberasurecode_encode_cleanup(
        desc: Desc,
        encoded_data: *mut *mut u8,
        encoded_parity: *mut *mut u8,
    ) -> c_int;

    /// Reconstruct original data from a set of k encoded fragments
    ///
    /// @param desc - liberasurecode descriptor/handle
    ///        from liberasurecode_instance_create()
    /// @param fragments - erasure encoded fragments (> = k)
    /// @param num_fragments - number of fragments being passed in
    /// @param fragment_len - length of each fragment (assume they are the same)
    /// @param force_metadata_checks - force fragment metadata checks (default: 0)
    /// @param out_data - _output_ pointer to decoded data
    /// @param out_data_len - _output_ length of decoded output
    ///          (both output data pointers are allocated by liberasurecode,
    ///           caller invokes liberasurecode_decode_clean() after it has
    ///           read decoded data in 'out_data')
    ///
    /// @return 0 on success, -error code otherwise
    ///
    fn liberasurecode_decode(
        desc: Desc,
        fragments: *const *const u8,
        num_fragments: c_int,
        fragment_len: u64,
        force_metadata_checks: c_int,
        out_data: *mut *mut u8,
        out_data_len: *mut u64,
    ) -> c_int;

    /// Cleanup structures allocated by librasurecode_decode
    ///
    /// The caller has no context, so cannot safely free memory
    /// allocated by liberasurecode, so it must pass the
    /// deallocation responsibility back to liberasurecode.
    ///
    /// @param desc - liberasurecode descriptor/handle
    ///        from liberasurecode_instance_create()
    /// @param data - (char *) buffer of data decoded by librasurecode_decode
    ///
    /// @return 0 on success; -error otherwise
    ///
    fn liberasurecode_decode_cleanup(desc: Desc, data: *mut u8) -> c_int;

    /// Reconstruct a missing fragment from a subset of available fragments
    ///
    /// @param desc - liberasurecode descriptor/handle
    ///        from liberasurecode_instance_create()
    /// @param available_fragments - erasure encoded fragments
    /// @param num_fragments - number of fragments being passed in
    /// @param fragment_len - size in bytes of the fragments
    /// @param destination_idx - missing idx to reconstruct
    /// @param out_fragment - output of reconstruct
    ///
    /// @return 0 on success, -error code otherwise
    ///
    fn liberasurecode_reconstruct_fragment(
        desc: Desc,
        available_fragments: *const *const u8,
        num_fragments: c_int,
        fragment_len: u64,
        destination_idx: c_int,
        out_fragment: *mut u8,
    ) -> Desc;
}

pub fn instance_create(id: EcBackendId, args: &EcArgs) -> Result<Desc, ErrorCode> {
    match unsafe { liberasurecode_instance_create(id, args) } {
        desc if desc > 0 => Ok(desc),
        code => Err(-code as ErrorCode),
    }
}

pub fn instance_destroy(desc: Desc) -> Result<(), ErrorCode> {
    match unsafe { liberasurecode_instance_destroy(desc) } {
        0 => Ok(()),
        code => Err(code as ErrorCode),
    }
}

pub fn encode(
    desc: Desc,
    orig_data: &[u8],
) -> Result<(*mut *mut u8, *mut *mut u8, u64), ErrorCode> {
    let mut encoded_data = ptr::null_mut();
    let mut encoded_parity = ptr::null_mut();
    let mut fragment_len = 0;
    let result = unsafe {
        liberasurecode_encode(
            desc,
            orig_data.as_ptr(),
            orig_data.len() as u64,
            &mut encoded_data,
            &mut encoded_parity,
            &mut fragment_len,
        )
    };
    match result {
        0 => Ok((encoded_data, encoded_parity, fragment_len)),
        _ => Err(-result as ErrorCode),
    }
}

pub fn encode_cleanup(
    desc: Desc,
    encoded_data: *mut *mut u8,
    encoded_parity: *mut *mut u8,
) -> Result<(), ErrorCode> {
    match unsafe { liberasurecode_encode_cleanup(desc, encoded_data, encoded_parity) } {
        0 => Ok(()),
        code => Err(-code as ErrorCode),
    }
}

pub fn decode(
    desc: Desc,
    fragments: &[&[u8]],
    force_metadata_checks: bool,
) -> Result<(*mut u8, u64), ErrorCode> {
    assert!(!fragments.is_empty());

    let mut out_data = ptr::null_mut();
    let mut out_data_len = 0;
    let result = unsafe {
        liberasurecode_decode(
            desc,
            fragments
                .iter()
                .map(|x| x.as_ptr())
                .collect::<Vec<_>>()
                .as_ptr(),
            fragments.len() as c_int,
            fragments[0].len() as u64,
            if force_metadata_checks { 1 } else { 0 },
            &mut out_data,
            &mut out_data_len,
        )
    };
    match result {
        0 => Ok((out_data, out_data_len)),
        _ => Err(-result as ErrorCode),
    }
}

pub fn decode_cleanup(desc: Desc, data: *mut u8) -> Result<(), ErrorCode> {
    match unsafe { liberasurecode_decode_cleanup(desc, data) } {
        0 => Ok(()),
        code => Err(-code as ErrorCode),
    }
}

pub fn reconstruct_fragment(
    desc: Desc,
    available_fragments: &[&[u8]],
    destination_idx: usize,
) -> Result<Vec<u8>, ErrorCode> {
    assert!(!available_fragments.is_empty());
    let mut buf = vec![0; available_fragments[0].len()];
    let result = unsafe {
        liberasurecode_reconstruct_fragment(
            desc,
            available_fragments
                .iter()
                .map(|x| x.as_ptr())
                .collect::<Vec<_>>()
                .as_ptr(),
            available_fragments.len() as c_int,
            available_fragments[0].len() as u64,
            destination_idx as c_int,
            buf.as_mut_ptr(),
        )
    };
    match result {
        0 => Ok(buf),
        _ => Err(-result as ErrorCode),
    }
}
