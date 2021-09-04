use super::action::{Action, ActionList};
use super::bitboard::*;
use super::gamestate::GameState;
use super::piece::Piece;

#[rustfmt::skip]
pub const SEAL_PATTERN: [u64; 64] = [132096, 329728, 659712, 1319424, 2638848, 5277696, 10489856, 4202496, 33816580, 84410376, 168886289, 337772578, 675545156, 1351090312, 2685403152, 1075839008, 8657044482, 21609056261, 43234889994, 86469779988, 172939559976, 345879119952, 687463207072, 275414786112, 2216203387392, 5531918402816, 11068131838464, 22136263676928, 44272527353856, 88545054707712, 175990581010432, 70506185244672, 567348067172352, 1416171111120896, 2833441750646784, 5666883501293568, 11333767002587136, 22667534005174272, 45053588738670592, 18049583422636032, 145241105196122112, 362539804446949376, 725361088165576704, 1450722176331153408, 2901444352662306816, 5802888705324613632, 11533718717099671552, 4620693356194824192, 288234782788157440, 576469569871282176, 1224997833292120064, 2449995666584240128, 4899991333168480256, 9799982666336960512, 1152939783987658752, 2305878468463689728, 1128098930098176, 2257297371824128, 4796069720358912, 9592139440717824, 19184278881435648, 38368557762871296, 4679521487814656, 9077567998918656];
#[rustfmt::skip]
pub const STARFISH_PATTERN: [u64; 128] = [514, 1284, 2568, 5136, 10272, 20544, 41088, 16384, 131586, 328709, 657418, 1314836, 2629672, 5259344, 10518688, 4194368, 33686016, 84149504, 168299008, 336598016, 673196032, 1346392064, 2692784128, 1073758208, 8623620096, 21542273024, 43084546048, 86169092096, 172338184192, 344676368384, 689352736768, 274882101248, 2207646744576, 5514821894144, 11029643788288, 22059287576576, 44118575153152, 88237150306304, 176474300612608, 70369817919488, 565157566611456, 1411794404900864, 2823588809801728, 5647177619603456, 11294355239206912, 22588710478413824, 45177420956827648, 18014673387388928, 144680337052532736, 361419367654621184, 722838735309242368, 1445677470618484736, 2891354941236969472, 5782709882473938944, 11565419764947877888, 4611756387171565568, 144678138029277184, 289637751035265024, 579275502070530048, 1158551004141060096, 2317102008282120192, 4634204016564240384, 9268408033128480768, 18014398509481984, 512, 1281, 2562, 5124, 10248, 20496, 40992, 16448, 131074, 327941, 655882, 1311764, 2623528, 5247056, 10494112, 4210752, 33554944, 83952896, 167905792, 335811584, 671623168, 1343246336, 2686492672, 1077952512, 8590065664, 21491941376, 42983882752, 85967765504, 171935531008, 343871062016, 687742124032, 275955843072, 2199056809984, 5501936992256, 11003873984512, 22007747969024, 44015495938048, 88030991876096, 176061983752192, 70644695826432, 562958543355904, 1408495870017536, 2816991740035072, 5633983480070144, 11267966960140288, 22535933920280576, 45071867840561152, 18085042131566592, 144117387099111424, 360574942724489216, 721149885448978432, 1442299770897956864, 2884599541795913728, 5769199083591827456, 11538398167183654912, 4629770785681047552, 562949953421312, 73464968921481216, 146929937842962432, 293859875685924864, 587719751371849728, 1175439502743699456, 2350879005487398912, 4629700416936869888];
#[rustfmt::skip]
pub const COCKLE_PATTERN: [u64; 128] = [512, 1024, 2048, 4096, 8192, 16384, 32768, 0, 131074, 262148, 524296, 1048592, 2097184, 4194368, 8388736, 0, 33554944, 67109888, 134219776, 268439552, 536879104, 1073758208, 2147516416, 0, 8590065664, 17180131328, 34360262656, 68720525312, 137441050624, 274882101248, 549764202496, 0, 2199056809984, 4398113619968, 8796227239936, 17592454479872, 35184908959744, 70369817919488, 140739635838976, 0, 562958543355904, 1125917086711808, 2251834173423616, 4503668346847232, 9007336693694464, 18014673387388928, 36029346774777856, 0, 144117387099111424, 288234774198222848, 576469548396445696, 1152939096792891392, 2305878193585782784, 4611756387171565568, 9223512774343131136, 0, 562949953421312, 1125899906842624, 2251799813685248, 4503599627370496, 9007199254740992, 18014398509481984, 36028797018963968, 0, 0, 256, 512, 1024, 2048, 4096, 8192, 16384, 0, 65537, 131074, 262148, 524296, 1048592, 2097184, 4194368, 0, 16777472, 33554944, 67109888, 134219776, 268439552, 536879104, 1073758208, 0, 4295032832, 8590065664, 17180131328, 34360262656, 68720525312, 137441050624, 274882101248, 0, 1099528404992, 2199056809984, 4398113619968, 8796227239936, 17592454479872, 35184908959744, 70369817919488, 0, 281479271677952, 562958543355904, 1125917086711808, 2251834173423616, 4503668346847232, 9007336693694464, 18014673387388928, 0, 72058693549555712, 144117387099111424, 288234774198222848, 576469548396445696, 1152939096792891392, 2305878193585782784, 4611756387171565568, 0, 281474976710656, 562949953421312, 1125899906842624, 2251799813685248, 4503599627370496, 9007199254740992, 18014398509481984];

/*
pub fn is_game_over(state: &GameState) -> bool {
    state.ply > 60 || ((state.ambers[0] > 1 || state.ambers[1] > 1) && state.ply & 0b1 == 0)
}*/

pub fn do_action(state: &mut GameState, action: Action) {
    let color = state.get_current_color();
    let other_color = color ^ 1;
    let to_bit = 1 << action.to;
    let from_bit = 1 << action.from;
    let mut m = to_bit | from_bit;
    if action.piece as usize != Piece::Seal as usize && to_bit & FINISH_LINES[color] > 0 {
        m ^= to_bit;
        state.ambers[color] += 1;
    }
    if to_bit & state.stacked > 0 {
        state.ambers[color] += 1;
        state.stacked &= !m;
        m &= !to_bit;
    } else {
        state.stacked ^= m;
    }
    state.board[color][action.piece as usize] ^= m;
    state.occupied[color] ^= m;
    if to_bit & state.occupied[other_color] > 0 {
        let mask = !to_bit;
        state.occupied[other_color] &= mask;
        for piece in 0..4 {
            state.board[other_color][piece] &= mask;
        }
    }
    state.ply += 1;
}

pub fn get_legal_actions(state: &GameState, al: &mut ActionList) {
    al.clear();
    let color = state.get_current_color();
    append_cockle_actions(state, al, color);
    append_starfish_actions(state, al, color);
    append_seal_actions(state, al, color);
    append_gull_actions(state, al, color);
}

pub fn append_cockle_actions(state: &GameState, al: &mut ActionList, color: usize) {
    let occupied_by_color = state.occupied[color];
    let mut cockles = state.board[color][Piece::Cockle as usize];
    while cockles > 0 {
        let from = cockles.trailing_zeros();
        cockles ^= 1 << from;
        let mut destinations = COCKLE_PATTERN[from as usize | color << 6] & !occupied_by_color;
        while destinations > 0 {
            let to = destinations.trailing_zeros();
            destinations ^= 1 << to;
            al.push(Action {
                from: from as u8,
                to: to as u8,
                piece: Piece::Cockle,
            });
        }
    }
}

pub fn append_gull_actions(state: &GameState, al: &mut ActionList, color: usize) {
    let mut gulls = state.board[color][Piece::Gull as usize];
    let occupied_by_color = state.occupied[color];
    while gulls > 0 {
        let from = gulls.trailing_zeros();
        let bit = 1 << from;
        gulls ^= bit;
        let mut destinations =
            ((bit & !SHIFT_RIGHT_MASK) << 1 | (bit & !SHIFT_LEFT_MASK) >> 1 | bit >> 8 | bit << 8)
                & !occupied_by_color;
        while destinations > 0 {
            let to = destinations.trailing_zeros();
            destinations ^= 1 << to;
            al.push(Action {
                to: to as u8,
                from: from as u8,
                piece: Piece::Gull,
            });
        }
    }
}

pub fn append_starfish_actions(state: &GameState, al: &mut ActionList, color: usize) {
    let occupied_by_color = state.occupied[color];
    let mut starfish = state.board[color][Piece::Starfish as usize];
    while starfish > 0 {
        let from = starfish.trailing_zeros();
        let bit = 1 << from;
        starfish ^= bit;
        let mut destinations = STARFISH_PATTERN[from as usize | color << 6] & !occupied_by_color;
        while destinations > 0 {
            let to = destinations.trailing_zeros();
            destinations ^= 1 << to;
            al.push(Action {
                from: from as u8,
                to: to as u8,
                piece: Piece::Starfish,
            });
        }
    }
}

pub fn append_seal_actions(state: &GameState, al: &mut ActionList, color: usize) {
    let occupied_by_color = state.occupied[color];
    let mut seals = state.board[color][Piece::Seal as usize];
    while seals > 0 {
        let from = seals.trailing_zeros();
        let bit = 1 << from;
        seals ^= bit;
        let mut destinations = SEAL_PATTERN[from as usize] & !occupied_by_color;
        while destinations > 0 {
            let to = destinations.trailing_zeros();
            destinations ^= 1 << to;
            al.push(Action {
                from: from as u8,
                to: to as u8,
                piece: Piece::Seal,
            });
        }
    }
}