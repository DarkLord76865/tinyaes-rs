//! A module containing the core of the AES algorithm.





// DISABLED LINTS

#![allow(clippy::needless_range_loop)]  // better readability
#![allow(clippy::mut_range_bound)]  // we are aware that we aren't mutating the range bound, that was never the intention





// IMPORTS

use core::ops::{
    Index,
    IndexMut,
    Range,
    RangeFrom
};





// ENUMS

/// The AES key used to encrypt and decrypt data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AESKey {
    AES128([u8; 16]),
    AES192([u8; 24]),
    AES256([u8; 32]),
}

/// The round keys used in the AES algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RoundKeys {
    AES128([[u8; 4]; 44]),
    AES192([[u8; 4]; 52]),
    AES256([[u8; 4]; 60]),
}

impl RoundKeys {
    fn len(&self) -> usize {
        match self {
            RoundKeys::AES128(round_keys) => round_keys.len(),
            RoundKeys::AES192(round_keys) => round_keys.len(),
            RoundKeys::AES256(round_keys) => round_keys.len(),
        }
    }
}
impl Index<usize> for RoundKeys {
    type Output = [u8; 4];

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            RoundKeys::AES128(round_keys) => &round_keys[index],
            RoundKeys::AES192(round_keys) => &round_keys[index],
            RoundKeys::AES256(round_keys) => &round_keys[index],
        }
    }
}
impl Index<Range<usize>> for RoundKeys {
    type Output = [[u8; 4]];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        match self {
            RoundKeys::AES128(round_keys) => &round_keys[index],
            RoundKeys::AES192(round_keys) => &round_keys[index],
            RoundKeys::AES256(round_keys) => &round_keys[index],
        }
    }
}
impl Index<RangeFrom<usize>> for RoundKeys {
    type Output = [[u8; 4]];

    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        match self {
            RoundKeys::AES128(round_keys) => &round_keys[index],
            RoundKeys::AES192(round_keys) => &round_keys[index],
            RoundKeys::AES256(round_keys) => &round_keys[index],
        }
    }
}
impl IndexMut<usize> for RoundKeys {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            RoundKeys::AES128(round_keys) => &mut round_keys[index],
            RoundKeys::AES192(round_keys) => &mut round_keys[index],
            RoundKeys::AES256(round_keys) => &mut round_keys[index],
        }
    }
}
impl IndexMut<Range<usize>> for RoundKeys {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        match self {
            RoundKeys::AES128(round_keys) => &mut round_keys[index],
            RoundKeys::AES192(round_keys) => &mut round_keys[index],
            RoundKeys::AES256(round_keys) => &mut round_keys[index],
        }
    }
}
impl IndexMut<RangeFrom<usize>> for RoundKeys {
    fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut Self::Output {
        match self {
            RoundKeys::AES128(round_keys) => &mut round_keys[index],
            RoundKeys::AES192(round_keys) => &mut round_keys[index],
            RoundKeys::AES256(round_keys) => &mut round_keys[index],
        }
    }
}





// STRUCTS

/// The AES core algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AESCore {
    /// The AES key used to encrypt and decrypt data.
    key: AESKey,
    /// The round keys used in the AES algorithm.
    round_keys: RoundKeys,
}

/// Public functions for encrypting and decrypting data.
impl AESCore {
    pub fn new(key: AESKey) -> AESCore {
        //! Creates a new AES instance with the given key.

        Self {
            key,
            round_keys: Self::key_expansion(&key),
        }
    }

    pub fn key(&self) -> AESKey {
        //! Returns the key used by this AES instance.

        self.key
    }

    pub fn set_key(&mut self, key: AESKey) {
        //! Changes the key used by this AES instance.

        self.key = key;
        self.round_keys = Self::key_expansion(&key);
    }

    pub fn encrypt(&self, block: &[u8; 16]) -> [u8; 16] {
        //! Encrypts the given block of data.

        // convert block to state
        let mut state: [[u8; 4]; 4] = [[0; 4]; 4];
        for r in 0..4 {
            for c in 0..4 {
                state[r][c] = block[r + c * 4];
            }
        }

        // encryption starts here
        Self::add_round_key(&mut state, &self.round_keys[0..4]);
        for round in 1..(match self.key {
            AESKey::AES128(_) => 10,
            AESKey::AES192(_) => 12,
            AESKey::AES256(_) => 14,
        }) {
            Self::sub_bytes(&mut state);
            Self::shift_rows(&mut state);
            Self::mix_columns(&mut state);
            Self::add_round_key(&mut state, &self.round_keys[round * 4..(round + 1) * 4]);
        }
        Self::sub_bytes(&mut state);
        Self::shift_rows(&mut state);
        Self::add_round_key(&mut state, &self.round_keys[(self.round_keys.len() - 4)..]);
        // encryption ends here

        // convert state to output block
        let mut out_block: [u8; 16] = [0; 16];
        for r in 0..4 {
            for c in 0..4 {
                out_block[r + c * 4] = state[r][c];
            }
        }
        out_block
    }

    pub fn decrypt(&self, block: &[u8; 16]) -> [u8; 16] {
        //! Decrypts the given block of data.

        // convert block to state
        let mut state: [[u8; 4]; 4] = [[0; 4]; 4];
        for r in 0..4 {
            for c in 0..4 {
                state[r][c] = block[r + c * 4];
            }
        }

        // decryption starts here
        Self::add_round_key(&mut state, &self.round_keys[(self.round_keys.len() - 4)..]);
        for round in (1..(match self.key {
            AESKey::AES128(_) => 10,
            AESKey::AES192(_) => 12,
            AESKey::AES256(_) => 14,
        })).rev() {
            Self::inv_shift_rows(&mut state);
            Self::inv_sub_bytes(&mut state);
            Self::add_round_key(&mut state, &self.round_keys[round * 4..(round + 1) * 4]);
            Self::inv_mix_columns(&mut state);
        }
        Self::inv_shift_rows(&mut state);
        Self::inv_sub_bytes(&mut state);
        Self::add_round_key(&mut state, &self.round_keys[0..4]);
        // decryption ends here

        // convert state to output block
        let mut out_block: [u8; 16] = [0; 16];
        for r in 0..4 {
            for c in 0..4 {
                out_block[r + c * 4] = state[r][c];
            }
        }
        out_block
    }
}

/// Functions for encrypting and decrypting used in the AES algorithm.
impl AESCore {
    fn add_round_key(state: &mut [[u8; 4]; 4], round_keys: &[[u8; 4]]) {
        //! Adds the given round key to the state.

        for r in 0..4 {
            for c in 0..4 {
                state[r][c] ^= round_keys[c][r];
            }
        }
    }

    fn mix_columns(state: &mut [[u8; 4]; 4]) {
        //! Mixes the columns of the state.

        let mut temp_column: [u8; 4] = [0; 4];
        for c in 0..4 {
            temp_column[0] =
                (if (state[0][c] >> 7) == 1 {(state[0][c] << 1) ^ 0x1b} else {state[0][c] << 1}) ^
                ((if (state[1][c] >> 7) == 1 {(state[1][c] << 1) ^ 0x1b} else {state[1][c] << 1}) ^ state[1][c]) ^
                state[2][c] ^
                state[3][c];

            temp_column[1] =
                state[0][c] ^
                (if (state[1][c] >> 7) == 1 {(state[1][c] << 1) ^ 0x1b} else {state[1][c] << 1}) ^
                ((if (state[2][c] >> 7) == 1 {(state[2][c] << 1) ^ 0x1b} else {state[2][c] << 1}) ^ state[2][c]) ^
                state[3][c];

            temp_column[2] =
                state[0][c] ^
                state[1][c] ^
                (if (state[2][c] >> 7) == 1 {(state[2][c] << 1) ^ 0x1b} else {state[2][c] << 1}) ^
                ((if (state[3][c] >> 7) == 1 {(state[3][c] << 1) ^ 0x1b} else {state[3][c] << 1}) ^ state[3][c]);


            temp_column[3] =
                ((if (state[0][c] >> 7) == 1 {(state[0][c] << 1) ^ 0x1b} else {state[0][c] << 1}) ^ state[0][c]) ^
                state[1][c] ^
                state[2][c] ^
                (if (state[3][c] >> 7) == 1 {(state[3][c] << 1) ^ 0x1b} else {state[3][c] << 1});

            state[0][c] = temp_column[0];
            state[1][c] = temp_column[1];
            state[2][c] = temp_column[2];
            state[3][c] = temp_column[3];
        }
    }

    fn shift_rows(state: &mut [[u8; 4]; 4]) {
        //! Shifts the rows of the state.

        state[1].rotate_left(1);
        state[2].rotate_left(2);
        state[3].rotate_left(3);
    }

    fn sub_bytes(state: &mut [[u8; 4]; 4]) {
        //! Substitutes the bytes of the state with the S-Box.

        for r in 0..4 {
            for c in 0..4 {
                state[r][c] = S_BOX[(state[r][c] >> 4) as usize][(state[r][c] & 0b00001111) as usize];
            }
        }
    }

    fn inv_mix_columns(state: &mut [[u8; 4]; 4]) {
        //! Inverse mixes the columns of the state.
        
        let mut temp_column: [u8; 4] = [0; 4];
        let mut temp_mul: [[u8; 3]; 4] = [[0; 3]; 4];

        for c in 0..4 {
            for i in 0..4 {
                temp_mul[i][0] = if (state[i][c] >> 7) == 1 {(state[i][c] << 1) ^ 0x1b} else {state[i][c] << 1};
            }
            for i in 0..4 {
                for j in 1..3 {
                    temp_mul[i][j] = if (temp_mul[i][j - 1] >> 7) == 1 {
                        (temp_mul[i][j - 1] << 1) ^ 0x1b
                    } else {
                        temp_mul[i][j - 1] << 1
                    };
                }
            }

            // 09 = 01 + 08
            // 0b = 01 + 02 + 08
            // 0d = 01 + 04 + 08
            // 0e = 02 + 04 + 08
            // temp_mul = [[02, 04, 08]]

            temp_column[0] = 
                (temp_mul[0][0] ^ temp_mul[0][1] ^ temp_mul[0][2]) ^
                (state[1][c] ^ temp_mul[1][0] ^ temp_mul[1][2]) ^
                (state[2][c] ^ temp_mul[2][1] ^ temp_mul[2][2]) ^
                (state[3][c] ^ temp_mul[3][2]);

            temp_column[1] =
                (state[0][c] ^ temp_mul[0][2]) ^
                (temp_mul[1][0] ^ temp_mul[1][1] ^ temp_mul[1][2]) ^
                (state[2][c] ^ temp_mul[2][0] ^ temp_mul[2][2]) ^
                (state[3][c] ^ temp_mul[3][1] ^ temp_mul[3][2]);
            
            temp_column[2] =
                (state[0][c] ^ temp_mul[0][1] ^ temp_mul[0][2]) ^
                (state[1][c] ^ temp_mul[1][2]) ^
                (temp_mul[2][0] ^ temp_mul[2][1] ^ temp_mul[2][2]) ^
                (state[3][c] ^ temp_mul[3][0] ^ temp_mul[3][2]);
            
            temp_column[3] =
                (state[0][c] ^ temp_mul[0][0] ^ temp_mul[0][2]) ^
                (state[1][c] ^ temp_mul[1][1] ^ temp_mul[1][2]) ^
                (state[2][c] ^ temp_mul[2][2]) ^
                (temp_mul[3][0] ^ temp_mul[3][1] ^ temp_mul[3][2]);

            state[0][c] = temp_column[0];
            state[1][c] = temp_column[1];
            state[2][c] = temp_column[2];
            state[3][c] = temp_column[3];
        }
    }

    fn inv_shift_rows(state: &mut [[u8; 4]; 4]) {
        //! Inverse shifts the rows of the state.

        state[1].rotate_right(1);
        state[2].rotate_right(2);
        state[3].rotate_right(3);
    }

    fn inv_sub_bytes(state: &mut [[u8; 4]; 4]) {
        //! Inverse substitutes the bytes of the state with the inverse S-Box.

        for r in 0..4 {
            for c in 0..4 {
                state[r][c] = INV_S_BOX[(state[r][c] >> 4) as usize][(state[r][c] & 0b00001111) as usize];
            }
        }
    }
}

/// Key expansion functions for the AES algorithm.
impl AESCore {
    fn key_expansion(key: &AESKey) -> RoundKeys {
        //! Expands the key into a set of round keys.

        let mut round_keys = match key {
            AESKey::AES128(_) => RoundKeys::AES128([[0; 4]; 44]),
            AESKey::AES192(_) => RoundKeys::AES192([[0; 4]; 52]),
            AESKey::AES256(_) => RoundKeys::AES256([[0; 4]; 60]),
        };

        let mut position: usize = 0;

        match key {
            AESKey::AES128(key_seq) => {
                for i in (0..16).step_by(4) {
                    round_keys[position] = [
                        key_seq[i],
                        key_seq[i + 1],
                        key_seq[i + 2],
                        key_seq[i + 3]
                    ];
                    position += 1;
                }
            },
            AESKey::AES192(key_seq) => {
                for i in (0..24).step_by(4) {
                    round_keys[position] = [
                        key_seq[i],
                        key_seq[i + 1],
                        key_seq[i + 2],
                        key_seq[i + 3]
                    ];
                    position += 1;
                }
            },
            AESKey::AES256(key_seq) => {
                for i in (0..32).step_by(4) {
                    round_keys[position] = [
                        key_seq[i],
                        key_seq[i + 1],
                        key_seq[i + 2],
                        key_seq[i + 3]
                    ];
                    position += 1;
                }
            },
        }

        let nk: usize = match key {
            AESKey::AES128(_) => 4,
            AESKey::AES192(_) => 6,
            AESKey::AES256(_) => 8,
        };

        for i in position..round_keys.len() {
            let mut temp: [u8; 4] = round_keys[i - 1];
            if i % nk == 0 {
                Self::rot_word(&mut temp);
                Self::sub_word(&mut temp);
                temp[0] ^= (R_CON[(i / nk) - 1] >> 24) as u8;
            } else if nk == 8 && i % nk == 4 {
                Self::sub_word(&mut temp);
            }
            round_keys[position] = [
                round_keys[i - nk][0] ^ temp[0],
                round_keys[i - nk][1] ^ temp[1],
                round_keys[i - nk][2] ^ temp[2],
                round_keys[i - nk][3] ^ temp[3],
            ];
            position += 1;
        }

        round_keys
    }

    fn rot_word(word: &mut [u8; 4]) {
        //! Rotates the word to the left by one byte.

        word.rotate_left(1);
    }

    fn sub_word(word: &mut [u8; 4]) {
        //! Substitutes the bytes of the word with the S-Box.

        for i in 0..4 {
            word[i] = S_BOX[(word[i] >> 4) as usize][(word[i] & 0b00001111) as usize];
        }
    }
}





// CONSTANTS

/// The S-Box used in the AES algorithm.
pub const S_BOX: [[u8; 16]; 16] = [
    [0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76],
    [0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0],
    [0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15],
    [0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75],
    [0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84],
    [0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf],
    [0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8],
    [0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2],
    [0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73],
    [0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb],
    [0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79],
    [0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08],
    [0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a],
    [0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e],
    [0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf],
    [0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16],
];

/// The inverse S-Box used in the AES algorithm.
pub const INV_S_BOX: [[u8; 16]; 16] = [
    [0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb],
    [0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb],
    [0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e],
    [0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25],
    [0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92],
    [0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84],
    [0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06],
    [0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b],
    [0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73],
    [0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e],
    [0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b],
    [0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4],
    [0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f],
    [0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef],
    [0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61],
    [0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d],
];

/// The round constants used in the AES algorithm.
pub const R_CON: [u32; 10] = [
    0x01000000, 0x02000000, 0x04000000, 0x08000000, 0x10000000,
    0x20000000, 0x40000000, 0x80000000, 0x1b000000, 0x36000000,
];





// TESTS

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        //! Test the new function

        let aes128: AESCore = AESCore::new(AESKey::AES128(
            [0x2b, 0x7e, 0x15, 0x16,
             0x28, 0xae, 0xd2, 0xa6,
             0xab, 0xf7, 0x15, 0x88,
             0x09, 0xcf, 0x4f, 0x3c],
        ));

        match aes128.key {
            AESKey::AES128(_) => (),
            _ => panic!("AES128 not created correctly in new function"),
        }

        let aes192: AESCore = AESCore::new(AESKey::AES192(
            [0x8e, 0x73, 0xb0, 0xf7,
             0xda, 0x0e, 0x64, 0x52,
             0xc8, 0x10, 0xf3, 0x2b,
             0x80, 0x90, 0x79, 0xe5,
             0x62, 0xf8, 0xea, 0xd2,
             0x52, 0x2c, 0x6b, 0x7b],
        ));

        match aes192.key {
            AESKey::AES192(_) => (),
            _ => panic!("AES192 not created correctly in new function"),
        }

        let aes256: AESCore = AESCore::new(AESKey::AES256(
            [0x60, 0x3d, 0xeb, 0x10,
             0x15, 0xca, 0x71, 0xbe,
             0x2b, 0x73, 0xae, 0xf0,
             0x85, 0x7d, 0x77, 0x81,
             0x1f, 0x35, 0x2c, 0x07,
             0x3b, 0x61, 0x08, 0xd7,
             0x2d, 0x98, 0x10, 0xa3,
             0x09, 0x14, 0xdf, 0xf4],
        ));

        match aes256.key {
            AESKey::AES256(_) => (),
            _ => panic!("AES256 not created correctly in new function"),
        }
    }

    #[test]
    fn encrypt() {
        //! Test encryption with AES-128, AES-192, and AES-256

        let aes128_1: AESCore = AESCore::new(AESKey::AES128(
            [0x2b, 0x7e, 0x15, 0x16,
             0x28, 0xae, 0xd2, 0xa6,
             0xab, 0xf7, 0x15, 0x88,
             0x09, 0xcf, 0x4f, 0x3c],
        ));
        let result128_1: [u8; 16] = aes128_1.encrypt(&[
            0x32, 0x43, 0xf6, 0xa8,
            0x88, 0x5a, 0x30, 0x8d,
            0x31, 0x31, 0x98, 0xa2,
            0xe0, 0x37, 0x07, 0x34]);
        assert_eq!(result128_1, [
            0x39, 0x25, 0x84, 0x1d,
            0x02, 0xdc, 0x09, 0xfb,
            0xdc, 0x11, 0x85, 0x97,
            0x19, 0x6a, 0x0b, 0x32]);

        let aes128_2: AESCore = AESCore::new(AESKey::AES128(
            [0x00, 0x01, 0x02, 0x03,
             0x04, 0x05, 0x06, 0x07,
             0x08, 0x09, 0x0a, 0x0b,
             0x0c, 0x0d, 0x0e, 0x0f],
        ));
        let result128_2: [u8; 16] = aes128_2.encrypt(&[
            0x00, 0x11, 0x22, 0x33,
            0x44, 0x55, 0x66, 0x77,
            0x88, 0x99, 0xaa, 0xbb,
            0xcc, 0xdd, 0xee, 0xff]);
        assert_eq!(result128_2, [
            0x69, 0xc4, 0xe0, 0xd8,
            0x6a, 0x7b, 0x04, 0x30,
            0xd8, 0xcd, 0xb7, 0x80,
            0x70, 0xb4, 0xc5, 0x5a]);

        let aes192: AESCore = AESCore::new(AESKey::AES192(
            [0x00, 0x01, 0x02, 0x03,
             0x04, 0x05, 0x06, 0x07,
             0x08, 0x09, 0x0a, 0x0b,
             0x0c, 0x0d, 0x0e, 0x0f,
             0x10, 0x11, 0x12, 0x13,
             0x14, 0x15, 0x16, 0x17],
        ));
        let result192: [u8; 16] = aes192.encrypt(&[
            0x00, 0x11, 0x22, 0x33,
            0x44, 0x55, 0x66, 0x77,
            0x88, 0x99, 0xaa, 0xbb,
            0xcc, 0xdd, 0xee, 0xff]);
        assert_eq!(result192, [
            0xdd, 0xa9, 0x7c, 0xa4,
            0x86, 0x4c, 0xdf, 0xe0,
            0x6e, 0xaf, 0x70, 0xa0,
            0xec, 0x0d, 0x71, 0x91]);

        let aes256: AESCore = AESCore::new(AESKey::AES256(
            [0x00, 0x01, 0x02, 0x03,
             0x04, 0x05, 0x06, 0x07,
             0x08, 0x09, 0x0a, 0x0b,
             0x0c, 0x0d, 0x0e, 0x0f,
             0x10, 0x11, 0x12, 0x13,
             0x14, 0x15, 0x16, 0x17,
             0x18, 0x19, 0x1a, 0x1b,
             0x1c, 0x1d, 0x1e, 0x1f],
        ));
        let result256: [u8; 16] = aes256.encrypt(&[
            0x00, 0x11, 0x22, 0x33,
            0x44, 0x55, 0x66, 0x77,
            0x88, 0x99, 0xaa, 0xbb,
            0xcc, 0xdd, 0xee, 0xff]);
        assert_eq!(result256, [
            0x8e, 0xa2, 0xb7, 0xca,
            0x51, 0x67, 0x45, 0xbf,
            0xea, 0xfc, 0x49, 0x90,
            0x4b, 0x49, 0x60, 0x89]);
    }

    #[test]
    fn decrypt() {
        //! Test decryption with AES-128, AES-192, and AES-256.

        let aes128: AESCore = AESCore::new(AESKey::AES128([
            0x00, 0x01, 0x02, 0x03,
            0x04, 0x05, 0x06, 0x07,
            0x08, 0x09, 0x0a, 0x0b,
            0x0c, 0x0d, 0x0e, 0x0f,
        ]));
        let result128: [u8; 16] = aes128.decrypt(&[
            0x69, 0xc4, 0xe0, 0xd8,
            0x6a, 0x7b, 0x04, 0x30,
            0xd8, 0xcd, 0xb7, 0x80,
            0x70, 0xb4, 0xc5, 0x5a]);
        assert_eq!(result128, [
            0x00, 0x11, 0x22, 0x33,
            0x44, 0x55, 0x66, 0x77,
            0x88, 0x99, 0xaa, 0xbb,
            0xcc, 0xdd, 0xee, 0xff]);

        let aes192: AESCore = AESCore::new(AESKey::AES192([
            0x00, 0x01, 0x02, 0x03,
            0x04, 0x05, 0x06, 0x07,
            0x08, 0x09, 0x0a, 0x0b,
            0x0c, 0x0d, 0x0e, 0x0f,
            0x10, 0x11, 0x12, 0x13,
            0x14, 0x15, 0x16, 0x17,
        ]));
        let result192: [u8; 16] = aes192.decrypt(&[
            0xdd, 0xa9, 0x7c, 0xa4,
            0x86, 0x4c, 0xdf, 0xe0,
            0x6e, 0xaf, 0x70, 0xa0,
            0xec, 0x0d, 0x71, 0x91]);
        assert_eq!(result192, [
            0x00, 0x11, 0x22, 0x33,
            0x44, 0x55, 0x66, 0x77,
            0x88, 0x99, 0xaa, 0xbb,
            0xcc, 0xdd, 0xee, 0xff]);

        let aes256: AESCore = AESCore::new(AESKey::AES256([
            0x00, 0x01, 0x02, 0x03,
            0x04, 0x05, 0x06, 0x07,
            0x08, 0x09, 0x0a, 0x0b,
            0x0c, 0x0d, 0x0e, 0x0f,
            0x10, 0x11, 0x12, 0x13,
            0x14, 0x15, 0x16, 0x17,
            0x18, 0x19, 0x1a, 0x1b,
            0x1c, 0x1d, 0x1e, 0x1f,
        ]));
        let result256: [u8; 16] = aes256.decrypt(&[
            0x8e, 0xa2, 0xb7, 0xca,
            0x51, 0x67, 0x45, 0xbf,
            0xea, 0xfc, 0x49, 0x90,
            0x4b, 0x49, 0x60, 0x89]);
        assert_eq!(result256, [
            0x00, 0x11, 0x22, 0x33,
            0x44, 0x55, 0x66, 0x77,
            0x88, 0x99, 0xaa, 0xbb,
            0xcc, 0xdd, 0xee, 0xff]);
    }

    #[test]
    fn set_key() {
        //! Test changing the key

        let key = AESKey::AES128([
            0x00, 0x01, 0x02, 0x03,
            0x04, 0x05, 0x06, 0x07,
            0x08, 0x09, 0x0a, 0x0b,
            0x0c, 0x0d, 0x0e, 0x0f]);

        let mut aes_core = AESCore::new(key);
        let original_aes_core = aes_core;

        assert_eq!(aes_core.key, key);
        assert_eq!(aes_core.key(), key);
        assert_eq!(aes_core.round_keys, AESCore::key_expansion(&key));
        assert_eq!(aes_core, original_aes_core);

        let new_key = AESKey::AES128([
            0x10, 0x11, 0x12, 0x13,
            0x14, 0x15, 0x16, 0x17,
            0x18, 0x19, 0x1a, 0x1b,
            0x1c, 0x1d, 0x1e, 0x1f]);
        aes_core.set_key(new_key);
        assert_eq!(aes_core.key, new_key);
        assert_eq!(aes_core.key(), new_key);
        assert_eq!(aes_core.round_keys, AESCore::key_expansion(&new_key));
        assert_ne!(aes_core, original_aes_core);

        let new_key2 = AESKey::AES192([
            0x20, 0x21, 0x22, 0x23,
            0x24, 0x25, 0x26, 0x27,
            0x28, 0x29, 0x2a, 0x2b,
            0x2c, 0x2d, 0x2e, 0x2f,
            0x30, 0x31, 0x32, 0x33,
            0x34, 0x35, 0x36, 0x37]);
        aes_core.set_key(new_key2);
        assert_eq!(aes_core.key, new_key2);
        assert_eq!(aes_core.key(), new_key2);
        assert_eq!(aes_core.round_keys, AESCore::key_expansion(&new_key2));
        assert_ne!(aes_core, original_aes_core);

        let new_key3 = AESKey::AES256([
            0x40, 0x41, 0x42, 0x43,
            0x44, 0x45, 0x46, 0x47,
            0x48, 0x49, 0x4a, 0x4b,
            0x4c, 0x4d, 0x4e, 0x4f,
            0x50, 0x51, 0x52, 0x53,
            0x54, 0x55, 0x56, 0x57,
            0x58, 0x59, 0x5a, 0x5b,
            0x5c, 0x5d, 0x5e, 0x5f]);
        aes_core.set_key(new_key3);
        assert_eq!(aes_core.key, new_key3);
        assert_eq!(aes_core.key(), new_key3);
        assert_eq!(aes_core.round_keys, AESCore::key_expansion(&new_key3));
        assert_ne!(aes_core, original_aes_core);

        aes_core.set_key(key);
        assert_eq!(aes_core.key, key);
        assert_eq!(aes_core.key(), key);
        assert_eq!(aes_core.round_keys, AESCore::key_expansion(&key));
        assert_eq!(aes_core, original_aes_core);
    }

    #[test]
    fn add_round_key() {
        //! Test the add round key function

        let aes128: AESCore = AESCore::new(AESKey::AES128(
            [0x00, 0x01, 0x02, 0x03,
                0x04, 0x05, 0x06, 0x07,
                0x08, 0x09, 0x0a, 0x0b,
                0x0c, 0x0d, 0x0e, 0x0f],
        ));

        let aes192: AESCore = AESCore::new(AESKey::AES192(
            [0x00, 0x01, 0x02, 0x03,
                0x04, 0x05, 0x06, 0x07,
                0x08, 0x09, 0x0a, 0x0b,
                0x0c, 0x0d, 0x0e, 0x0f,
                0x10, 0x11, 0x12, 0x13,
                0x14, 0x15, 0x16, 0x17],
        ));

        let aes256: AESCore = AESCore::new(AESKey::AES256(
            [0x00, 0x01, 0x02, 0x03,
                0x04, 0x05, 0x06, 0x07,
                0x08, 0x09, 0x0a, 0x0b,
                0x0c, 0x0d, 0x0e, 0x0f,
                0x10, 0x11, 0x12, 0x13,
                0x14, 0x15, 0x16, 0x17,
                0x18, 0x19, 0x1a, 0x1b,
                0x1c, 0x1d, 0x1e, 0x1f],
        ));

        let state_aes128_original: [[u8; 4]; 4] = [
            [0x00, 0x44, 0x88, 0xcc],
            [0x11, 0x55, 0x99, 0xdd],
            [0x22, 0x66, 0xaa, 0xee],
            [0x33, 0x77, 0xbb, 0xff]
        ];
        let state_aes128_inverted: [[u8; 4]; 4] = [
            [0x00, 0x40, 0x80, 0xc0],
            [0x10, 0x50, 0x90, 0xd0],
            [0x20, 0x60, 0xa0, 0xe0],
            [0x30, 0x70, 0xb0, 0xf0]
        ];
        let mut state_aes128_temp: [[u8; 4]; 4] = state_aes128_original;
        assert_eq!(state_aes128_original, state_aes128_temp);
        AESCore::add_round_key(&mut state_aes128_temp, &aes128.round_keys[0..4]);
        assert_eq!(state_aes128_temp, state_aes128_inverted);
        AESCore::add_round_key(&mut state_aes128_temp, &aes128.round_keys[0..4]);
        assert_eq!(state_aes128_temp, state_aes128_original);

        let state_aes192_original: [[u8; 4]; 4] = [
            [0x00, 0x44, 0x88, 0xcc],
            [0x11, 0x55, 0x99, 0xdd],
            [0x22, 0x66, 0xaa, 0xee],
            [0x33, 0x77, 0xbb, 0xff]
        ];
        let state_aes192_inverted: [[u8; 4]; 4] = [
            [0x00, 0x40, 0x80, 0xc0],
            [0x10, 0x50, 0x90, 0xd0],
            [0x20, 0x60, 0xa0, 0xe0],
            [0x30, 0x70, 0xb0, 0xf0]
        ];
        let mut state_aes192_temp: [[u8; 4]; 4] = state_aes192_original;
        assert_eq!(state_aes192_original, state_aes192_temp);
        AESCore::add_round_key(&mut state_aes192_temp, &aes192.round_keys[0..4]);
        assert_eq!(state_aes192_temp, state_aes192_inverted);
        AESCore::add_round_key(&mut state_aes192_temp, &aes192.round_keys[0..4]);
        assert_eq!(state_aes192_temp, state_aes192_original);

        let state_aes256_original: [[u8; 4]; 4] = [
            [0x00, 0x44, 0x88, 0xcc],
            [0x11, 0x55, 0x99, 0xdd],
            [0x22, 0x66, 0xaa, 0xee],
            [0x33, 0x77, 0xbb, 0xff]
        ];
        let state_aes256_inverted: [[u8; 4]; 4] = [
            [0x00, 0x40, 0x80, 0xc0],
            [0x10, 0x50, 0x90, 0xd0],
            [0x20, 0x60, 0xa0, 0xe0],
            [0x30, 0x70, 0xb0, 0xf0]
        ];
        let mut state_aes256_temp: [[u8; 4]; 4] = state_aes256_original;
        assert_eq!(state_aes256_original, state_aes256_temp);
        AESCore::add_round_key(&mut state_aes256_temp, &aes256.round_keys[0..4]);
        assert_eq!(state_aes256_temp, state_aes256_inverted);
        AESCore::add_round_key(&mut state_aes256_temp, &aes256.round_keys[0..4]);
        assert_eq!(state_aes256_temp, state_aes256_original);
    }

    #[test]
    fn mix_columns() {
        //! Test the mix columns and inverse mix columns functions

        let original_state: [[u8; 4]; 4] = [
            [0xdb, 0xf2, 0x01, 0xc6],
            [0x13, 0x0a, 0x01, 0xc6],
            [0x53, 0x22, 0x01, 0xc6],
            [0x45, 0x5c, 0x01, 0xc6]
        ];
        let inverted_state: [[u8; 4]; 4] = [
            [0x8e, 0x9f, 0x01, 0xc6],
            [0x4d, 0xdc, 0x01, 0xc6],
            [0xa1, 0x58, 0x01, 0xc6],
            [0xbc, 0x9d, 0x01, 0xc6]
        ];

        let mut temp_state: [[u8; 4]; 4] = original_state;

        assert_eq!(original_state, temp_state);
        AESCore::mix_columns(&mut temp_state);
        assert_eq!(inverted_state, temp_state);
        AESCore::inv_mix_columns(&mut temp_state);
        assert_eq!(original_state, temp_state);
    }

    #[test]
    fn shift_rows() {
        //! Test the shift rows and inverse shift rows functions

        let original_state: [[u8; 4]; 4] = [
            [0x00, 0x01, 0x02, 0x03],
            [0x10, 0x11, 0x12, 0x13],
            [0x20, 0x21, 0x22, 0x23],
            [0x30, 0x31, 0x32, 0x33]
        ];
        let inverted_state: [[u8; 4]; 4] = [
            [0x00, 0x01, 0x02, 0x03],
            [0x11, 0x12, 0x13, 0x10],
            [0x22, 0x23, 0x20, 0x21],
            [0x33, 0x30, 0x31, 0x32]
        ];

        let mut temp_state: [[u8; 4]; 4] = original_state;

        assert_eq!(original_state, temp_state);
        AESCore::shift_rows(&mut temp_state);
        assert_eq!(inverted_state, temp_state);
        AESCore::inv_shift_rows(&mut temp_state);
        assert_eq!(original_state, temp_state);
    }

    #[test]
    fn sub_bytes() {
        //! Test the sub bytes and inverse sub bytes functions

        let original_state: [[u8; 4]; 4] = [
            [0x19, 0xa0, 0x9a, 0xe9],
            [0x3d, 0xf4, 0xc6, 0xf8],
            [0xe3, 0xe2, 0x8d, 0x48],
            [0xbe, 0x2b, 0x2a, 0x08]
        ];
        let inverted_state: [[u8; 4]; 4] = [
            [0xd4, 0xe0, 0xb8, 0x1e],
            [0x27, 0xbf, 0xb4, 0x41],
            [0x11, 0x98, 0x5d, 0x52],
            [0xae, 0xf1, 0xe5, 0x30]
        ];

        let mut temp_state: [[u8; 4]; 4] = original_state;

        assert_eq!(original_state, temp_state);
        AESCore::sub_bytes(&mut temp_state);
        assert_eq!(inverted_state, temp_state);
        AESCore::inv_sub_bytes(&mut temp_state);
        assert_eq!(original_state, temp_state);
    }

    #[test]
    fn key_expansion() {
        //! Test the key expansion function

        let aes128: AESCore = AESCore::new(AESKey::AES128(
            [0x2b, 0x7e, 0x15, 0x16,
             0x28, 0xae, 0xd2, 0xa6,
             0xab, 0xf7, 0x15, 0x88,
             0x09, 0xcf, 0x4f, 0x3c],
        ));

        let aes192: AESCore = AESCore::new(AESKey::AES192(
            [0x8e, 0x73, 0xb0, 0xf7,
             0xda, 0x0e, 0x64, 0x52,
             0xc8, 0x10, 0xf3, 0x2b,
             0x80, 0x90, 0x79, 0xe5,
             0x62, 0xf8, 0xea, 0xd2,
             0x52, 0x2c, 0x6b, 0x7b],
        ));

        let aes256: AESCore = AESCore::new(AESKey::AES256(
            [0x60, 0x3d, 0xeb, 0x10,
             0x15, 0xca, 0x71, 0xbe,
             0x2b, 0x73, 0xae, 0xf0,
             0x85, 0x7d, 0x77, 0x81,
             0x1f, 0x35, 0x2c, 0x07,
             0x3b, 0x61, 0x08, 0xd7,
             0x2d, 0x98, 0x10, 0xa3,
             0x09, 0x14, 0xdf, 0xf4],
        ));

        assert_eq!(aes128.round_keys[aes128.round_keys.len() - 1], [0xb6, 0x63, 0x0c, 0xa6]);
        assert_eq!(aes128.round_keys.len(), 44);
        assert_eq!(aes192.round_keys[aes192.round_keys.len() - 1], [0x01, 0x00, 0x22, 0x02]);
        assert_eq!(aes192.round_keys.len(), 52);
        assert_eq!(aes256.round_keys[aes256.round_keys.len() - 1], [0x70, 0x6c, 0x63, 0x1e]);
        assert_eq!(aes256.round_keys.len(), 60);
    }

    #[test]
    fn rotate_word() {
        //! Test the rotate word function

        let mut word: [u8; 4] = [0x01, 0x02, 0x03, 0x04];
        let rotated_word: [u8; 4] = [0x02, 0x03, 0x04, 0x01];
        AESCore::rot_word(&mut word);
        assert_eq!(word, rotated_word);
    }

    #[test]
    fn sub_word() {
        //! Test the sub word function

        let mut word: [u8; 4] = [0x19, 0xa0, 0x9a, 0xe9];
        let subbed_word: [u8; 4] = [0xd4, 0xe0, 0xb8, 0x1e];
        AESCore::sub_word(&mut word);
        assert_eq!(word, subbed_word);
    }
}
