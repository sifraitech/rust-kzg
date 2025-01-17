pub mod c_bindings;
pub mod eip_7594;

pub const FIELD_ELEMENTS_PER_BLOB: usize = 4096;
pub const BYTES_PER_G1: usize = 48;
pub const BYTES_PER_G2: usize = 96;
pub const BYTES_PER_BLOB: usize = BYTES_PER_FIELD_ELEMENT * FIELD_ELEMENTS_PER_BLOB;
pub const BYTES_PER_FIELD_ELEMENT: usize = 32;
pub const BYTES_PER_PROOF: usize = 48;
pub const BYTES_PER_COMMITMENT: usize = 48;
pub const FIELD_ELEMENTS_PER_EXT_BLOB: usize = 2 * FIELD_ELEMENTS_PER_BLOB;
pub const FIELD_ELEMENTS_PER_CELL: usize = 64;
pub const BYTES_PER_CELL: usize = FIELD_ELEMENTS_PER_CELL * BYTES_PER_FIELD_ELEMENT;
pub const CELLS_PER_EXT_BLOB: usize = FIELD_ELEMENTS_PER_EXT_BLOB / FIELD_ELEMENTS_PER_CELL;
pub const RANDOM_CHALLENGE_KZG_CELL_BATCH_DOMAIN: [u8; 16] = *b"RCKZGCBATCH__V1_";
pub const TRUSTED_SETUP_NUM_G1_POINTS: usize = 4096;
pub const TRUSTED_SETUP_NUM_G2_POINTS: usize = 65;
