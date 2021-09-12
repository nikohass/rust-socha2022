use super::action::*;
use super::bitboard::*;
use super::gamestate::*;
use super::piece;

#[rustfmt::skip]
pub const SEAL_PATTERN: [u64; 64] = [132096, 329728, 659712, 1319424, 2638848, 5277696, 10489856, 4202496, 33816580, 84410376, 168886289, 337772578, 675545156, 1351090312, 2685403152, 1075839008, 8657044482, 21609056261, 43234889994, 86469779988, 172939559976, 345879119952, 687463207072, 275414786112, 2216203387392, 5531918402816, 11068131838464, 22136263676928, 44272527353856, 88545054707712, 175990581010432, 70506185244672, 567348067172352, 1416171111120896, 2833441750646784, 5666883501293568, 11333767002587136, 22667534005174272, 45053588738670592, 18049583422636032, 145241105196122112, 362539804446949376, 725361088165576704, 1450722176331153408, 2901444352662306816, 5802888705324613632, 11533718717099671552, 4620693356194824192, 288234782788157440, 576469569871282176, 1224997833292120064, 2449995666584240128, 4899991333168480256, 9799982666336960512, 1152939783987658752, 2305878468463689728, 1128098930098176, 2257297371824128, 4796069720358912, 9592139440717824, 19184278881435648, 38368557762871296, 4679521487814656, 9077567998918656];
#[rustfmt::skip]
pub const STARFISH_PATTERN: [u64; 128] = [514, 1284, 2568, 5136, 10272, 20544, 41088, 16384, 131586, 328709, 657418, 1314836, 2629672, 5259344, 10518688, 4194368, 33686016, 84149504, 168299008, 336598016, 673196032, 1346392064, 2692784128, 1073758208, 8623620096, 21542273024, 43084546048, 86169092096, 172338184192, 344676368384, 689352736768, 274882101248, 2207646744576, 5514821894144, 11029643788288, 22059287576576, 44118575153152, 88237150306304, 176474300612608, 70369817919488, 565157566611456, 1411794404900864, 2823588809801728, 5647177619603456, 11294355239206912, 22588710478413824, 45177420956827648, 18014673387388928, 144680337052532736, 361419367654621184, 722838735309242368, 1445677470618484736, 2891354941236969472, 5782709882473938944, 11565419764947877888, 4611756387171565568, 144678138029277184, 289637751035265024, 579275502070530048, 1158551004141060096, 2317102008282120192, 4634204016564240384, 9268408033128480768, 18014398509481984, 512, 1281, 2562, 5124, 10248, 20496, 40992, 16448, 131074, 327941, 655882, 1311764, 2623528, 5247056, 10494112, 4210752, 33554944, 83952896, 167905792, 335811584, 671623168, 1343246336, 2686492672, 1077952512, 8590065664, 21491941376, 42983882752, 85967765504, 171935531008, 343871062016, 687742124032, 275955843072, 2199056809984, 5501936992256, 11003873984512, 22007747969024, 44015495938048, 88030991876096, 176061983752192, 70644695826432, 562958543355904, 1408495870017536, 2816991740035072, 5633983480070144, 11267966960140288, 22535933920280576, 45071867840561152, 18085042131566592, 144117387099111424, 360574942724489216, 721149885448978432, 1442299770897956864, 2884599541795913728, 5769199083591827456, 11538398167183654912, 4629770785681047552, 562949953421312, 73464968921481216, 146929937842962432, 293859875685924864, 587719751371849728, 1175439502743699456, 2350879005487398912, 4629700416936869888];
#[rustfmt::skip]
pub const COCKLE_PATTERN: [u64; 128] = [512, 1024, 2048, 4096, 8192, 16384, 32768, 0, 131074, 262148, 524296, 1048592, 2097184, 4194368, 8388736, 0, 33554944, 67109888, 134219776, 268439552, 536879104, 1073758208, 2147516416, 0, 8590065664, 17180131328, 34360262656, 68720525312, 137441050624, 274882101248, 549764202496, 0, 2199056809984, 4398113619968, 8796227239936, 17592454479872, 35184908959744, 70369817919488, 140739635838976, 0, 562958543355904, 1125917086711808, 2251834173423616, 4503668346847232, 9007336693694464, 18014673387388928, 36029346774777856, 0, 144117387099111424, 288234774198222848, 576469548396445696, 1152939096792891392, 2305878193585782784, 4611756387171565568, 9223512774343131136, 0, 562949953421312, 1125899906842624, 2251799813685248, 4503599627370496, 9007199254740992, 18014398509481984, 36028797018963968, 0, 0, 256, 512, 1024, 2048, 4096, 8192, 16384, 0, 65537, 131074, 262148, 524296, 1048592, 2097184, 4194368, 0, 16777472, 33554944, 67109888, 134219776, 268439552, 536879104, 1073758208, 0, 4295032832, 8590065664, 17180131328, 34360262656, 68720525312, 137441050624, 274882101248, 0, 1099528404992, 2199056809984, 4398113619968, 8796227239936, 17592454479872, 35184908959744, 70369817919488, 0, 281479271677952, 562958543355904, 1125917086711808, 2251834173423616, 4503668346847232, 9007336693694464, 18014673387388928, 0, 72058693549555712, 144117387099111424, 288234774198222848, 576469548396445696, 1152939096792891392, 2305878193585782784, 4611756387171565568, 0, 281474976710656, 562949953421312, 1125899906842624, 2251799813685248, 4503599627370496, 9007199254740992, 18014398509481984];
#[rustfmt::skip]
pub const GULL_PATTERN: [u64; 64] = [258, 517, 1034, 2068, 4136, 8272, 16544, 32832, 66049, 132354, 264708, 529416, 1058832, 2117664, 4235328, 8405120, 16908544, 33882624, 67765248, 135530496, 271060992, 542121984, 1084243968, 2151710720, 4328587264, 8673951744, 17347903488, 34695806976, 69391613952, 138783227904, 277566455808, 550837944320, 1108118339584, 2220531646464, 4441063292928, 8882126585856, 17764253171712, 35528506343424, 71057012686848, 141014513745920, 283678294933504, 568456101494784, 1136912202989568, 2273824405979136, 4547648811958272, 9095297623916544, 18190595247833088, 36099715518955520, 72621643502977024, 145524761982664704, 291049523965329408, 582099047930658816, 1164198095861317632, 2328396191722635264, 4656792383445270528, 9241527172852613120, 144396663052566528, 360850920143060992, 721701840286121984, 1443403680572243968, 2886807361144487936, 5773614722288975872, 11547229444577951744, 4647714815446351872];

pub fn is_game_over(state: &GameState) -> bool {
    state.ply >= 59 || ((state.ambers[0] > 1 || state.ambers[1] > 1) && state.ply & 0b1 == 0)
}

pub fn game_result(state: &GameState) -> i16 {
    if state.ambers[0] == state.ambers[1] {
        for i in 0..8 {
            let red = (FINISH_LINES[RED] >> i & state.occupied[RED]).count_ones();
            let blue = (FINISH_LINES[BLUE] << i & state.occupied[BLUE]).count_ones();
            if red > blue {
                return 1;
            } else if blue > red {
                return -1;
            }
        }
        0
    } else if state.ambers[0] > state.ambers[1] {
        1
    } else {
        -1
    }
}

pub fn do_action(state: &mut GameState, action: Action) {
    let color = state.get_current_color();
    let mut undo_info = UndoInfo::default();
    let other_color = color ^ 0b1;
    let to_bit = 1 << action.to();
    let from_bit = 1 << action.from();
    let changed_fields = to_bit | from_bit;
    let piece = action.piece() as usize;
    let is_piece_stacked = state.stacked & from_bit > 0;
    if to_bit & state.occupied[other_color] > 0 {
        let changed_fields_that_are_stacked = changed_fields & state.stacked;
        if changed_fields_that_are_stacked > 0 {
            state.ambers[color] += 1;
            state.stacked &= !changed_fields;
            state.occupied[color] &= !changed_fields;
            state.board[color][piece] ^= from_bit;
        } else {
            state.stacked |= to_bit;
            state.occupied[color] ^= changed_fields;
            state.board[color][piece] ^= changed_fields;
        }
        let mask = !to_bit;
        state.occupied[other_color] &= mask;
        let mut capture_info: u8 = 0;
        if changed_fields_that_are_stacked & to_bit > 0 {
            capture_info |= CAPTURED_PIECE_WAS_STACKED;
        }
        if changed_fields_that_are_stacked & from_bit > 0 {
            capture_info |= MOVED_PIECE_WAS_STACKED;
        }
        for piece in 0..4 {
            if state.board[other_color][piece] & to_bit > 0 {
                state.board[other_color][piece] &= mask;
                undo_info.set_capture(piece as u8, capture_info);
                break;
            }
        }
    } else {
        state.occupied[color] ^= changed_fields;
        state.board[color][piece] ^= changed_fields;
        if state.stacked & from_bit > 0 {
            state.stacked ^= from_bit | to_bit;
        }
    }
    if piece as usize != piece::SEAL as usize && to_bit & FINISH_LINES[color] > 0 {
        if is_piece_stacked {
            undo_info.set_finish_line_info(MOVED_PIECE_WAS_STACKED);
        }
        let mask = !to_bit;
        state.board[color][piece] &= mask;
        state.occupied[color] &= mask;
        state.ambers[color] += 1;
        state.stacked &= mask;
    }
    state.undo[state.ply as usize] = undo_info;
    state.ply += 1;
}

pub fn undo_action(state: &mut GameState, action: Action) {
    state.ply -= 1;
    let color = state.get_current_color();
    let other_color = color ^ 1;
    let undo_info = state.undo[state.ply as usize];
    let to_bit = 1 << action.to();
    let from_bit = 1 << action.from();
    let piece = action.piece() as usize;
    let changed_fields = to_bit | from_bit;
    state.occupied[color] &= !to_bit;
    state.occupied[color] |= from_bit;
    state.board[color][piece] &= !to_bit;
    state.board[color][piece] |= from_bit;
    if let Some((piece, capture_info)) = undo_info.get_capture() {
        state.stacked &= !changed_fields;
        if capture_info > 0 {
            state.ambers[color] -= 1;
        }
        if capture_info & CAPTURED_PIECE_WAS_STACKED > 0 {
            state.stacked |= to_bit;
        }
        if capture_info & MOVED_PIECE_WAS_STACKED > 0 {
            state.stacked |= from_bit;
        }
        state.board[other_color][piece as usize] |= to_bit;
        state.occupied[other_color] |= to_bit;
    } else if state.stacked & to_bit > 0 {
        state.stacked ^= changed_fields;
    }
    if piece != piece::SEAL as usize && to_bit & FINISH_LINES[color] > 0 {
        state.ambers[color] -= 1;
        if undo_info.get_finish_line_info() & MOVED_PIECE_WAS_STACKED > 0 {
            state.stacked |= from_bit;
        }
    }
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
    let mut cockles = state.board[color][piece::COCKLE as usize];
    while cockles > 0 {
        let from = cockles.trailing_zeros();
        cockles ^= 1 << from;
        let mut destinations = COCKLE_PATTERN[from as usize | color << 6] & !state.occupied[color];
        while destinations > 0 {
            let to = destinations.trailing_zeros();
            destinations ^= 1 << to;
            al.push(Action::new(from as u16, to as u16, piece::COCKLE));
        }
    }
}

pub fn append_gull_actions(state: &GameState, al: &mut ActionList, color: usize) {
    let mut gulls = state.board[color][piece::GULL as usize];
    while gulls > 0 {
        let from = gulls.trailing_zeros();
        gulls ^= 1 << from;
        let mut destinations = GULL_PATTERN[from as usize] & !state.occupied[color];
        while destinations > 0 {
            let to = destinations.trailing_zeros();
            destinations ^= 1 << to;
            al.push(Action::new(from as u16, to as u16, piece::GULL));
        }
    }
}

pub fn append_starfish_actions(state: &GameState, al: &mut ActionList, color: usize) {
    let mut starfish = state.board[color][piece::STARFISH as usize];
    while starfish > 0 {
        let from = starfish.trailing_zeros();
        starfish ^= 1 << from;
        let mut destinations =
            STARFISH_PATTERN[from as usize | color << 6] & !state.occupied[color];
        while destinations > 0 {
            let to = destinations.trailing_zeros();
            destinations ^= 1 << to;
            al.push(Action::new(from as u16, to as u16, piece::STARFISH));
        }
    }
}

pub fn append_seal_actions(state: &GameState, al: &mut ActionList, color: usize) {
    let mut seals = state.board[color][piece::SEAL as usize];
    while seals > 0 {
        let from = seals.trailing_zeros();
        seals ^= 1 << from;
        let mut destinations = SEAL_PATTERN[from as usize] & !state.occupied[color];
        while destinations > 0 {
            let to = destinations.trailing_zeros();
            destinations ^= 1 << to;
            al.push(Action::new(from as u16, to as u16, piece::SEAL));
        }
    }
}
