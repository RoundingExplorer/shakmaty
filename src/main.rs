#![feature(asm, cfg_target_feature)]
#![feature(zero_one)]

mod square;
mod bitboard;

use square::Square;
use bitboard::Bitboard;
use std::num::Zero;

fn sliding_attack(sq: Square, occupied: Bitboard, deltas: &[i8]) -> Bitboard {
    let mut attack = Bitboard::zero();

    for delta in deltas {
        let Square(mut s) = sq;

        loop {
            s += *delta;
            if s < 0 || s >= 64 || square::distance(Square(s), Square(s - delta)) > 2 {
                break;
            }

            attack.add(Square(s));

            if occupied.contains(Square(s)) {
                break;
            }
        }
    }

    attack
}

static mut rook_masks: [Bitboard; 64] = [Bitboard(0); 64];

static rook_indexes : [usize; 64] =
    [0, 4096, 6144, 8192, 10240, 12288, 14336, 16384, 20480, 22528, 23552, 24576, 25600, 26624,
    27648, 28672, 30720, 32768, 33792, 34816, 35840, 36864, 37888, 38912, 40960, 43008, 44032,
    45056, 46080, 47104, 48128, 49152, 51200, 53248, 54272, 55296, 56320, 57344, 58368, 59392,
    61440, 63488, 64512, 65536, 66560, 67584, 68608, 69632, 71680, 73728, 74752, 75776, 76800,
    77824, 78848, 79872, 81920, 86016, 88064, 90112, 92160, 94208, 96256, 98304];

static mut rook_table : [Bitboard; 0x19000] = [Bitboard(0); 0x19000];

fn magic_index(indexes: &[usize], masks: &[Bitboard], Square(sq): Square, occupied: Bitboard) -> usize {
    indexes[sq as usize] + occupied.pext(masks[sq as usize]) as usize
}

fn init_rook_tables() {
    let rook_deltas = [8, 1, -8, -1];

    for s in 0..64 {
        let sq = Square(s);

        let edges = ((Bitboard::rank(0) | Bitboard::rank(7)) & !Bitboard::rank(sq.rank())) |
                    ((Bitboard::file(0) | Bitboard::file(7)) & !Bitboard::file(sq.file()));

        let mask = sliding_attack(sq, Bitboard::zero(), &rook_deltas) & !edges;
        unsafe { rook_masks[s as usize] = mask; }

        for subset in mask.subsets() {
            let attacks = sliding_attack(sq, subset, &rook_deltas);
            let index = magic_index(&rook_indexes, unsafe { &rook_masks }, sq, subset);

            unsafe { rook_table[index] = attacks; }
        }
    }
}

fn rook_attacks(sq: Square, occupied: Bitboard) -> Bitboard {
   unsafe {
       rook_table[magic_index(&rook_indexes, &rook_masks, sq, occupied)]
   }
}

fn main() {
    init_rook_tables();
}

#[test]
fn test_rook_attacks() {
    init_rook_tables();
    assert_eq!(rook_attacks(Square(43), Bitboard(4575420277326280121)), Bitboard(2312307447169024));
}
