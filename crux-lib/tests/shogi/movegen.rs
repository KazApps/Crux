// use crux_lib::shogi::{
//     movegen::{generate, is_legal},
//     position::Position,
// };
//
// #[test]
// fn perft4() {
//     use crux_lib::notation::usi::Usi;
//     use crux_lib::notation::Notation;
//
//     let mut pos = Usi::parse_position(
//         "8l/1l+R2P3/p2pBG1pp/kps1p4/Nn1P2G2/P1P1P2PP/1PS6/1KSG3+r1/LN2+p3L w Sbgn3p 1",
//     )
//     .unwrap();
//     // pos = Position::startpos();
//
//     println!("{}", perft(&mut pos, 4));
// }
//
// fn perft(pos: &mut Position, depth: i32) -> u64 {
//     let mut total = 0;
//
//     for mv in generate(pos) {
//         if is_legal(pos, mv) {
//             if depth == 1 {
//                 total += 1;
//             } else {
//                 let captured = pos.make_move(mv);
//                 total += perft(pos, depth - 1);
//                 pos.unmake_move(mv, captured);
//             }
//         }
//     }
//
//     total
// }
