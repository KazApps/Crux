use crux_lib::notation::{usi::Usi, Notation};
use crux_lib::shogi::position::Position;

#[test]
fn parse_moves() {
    const MOVES: [&str; 256] = [
        "6g6f", "6c6d", "2h6h", "1c1d", "7g7f", "1d1e", "6h2h", "7a6b", "2g2f", "3a3b", "6i7h",
        "6b6c", "2f2e", "6c5d", "2e2d", "2c2d", "2h2d", "P*2c", "2d2h", "8c8d", "7i6h", "6a5b",
        "6h6g", "3c3d", "5g5f", "5a4b", "3i4h", "4b3a", "4i5h", "6d6e", "6f6e", "2b8h+", "7h8h",
        "5d6e", "8i7g", "6e5d", "5i6h", "3a2b", "8h7h", "8d8e", "4h5g", "8e8f", "8g8f", "8b8f",
        "P*8g", "8f8b", "9g9f", "4c4d", "4g4f", "3b3c", "P*6d", "4a3b", "6g6f", "P*6b", "9f9e",
        "5d4c", "6h7i", "9c9d", "9e9d", "P*8f", "8g8f", "9a9d", "9i9d", "8b8f", "P*9g", "P*9f",
        "P*8g", "8f8d", "9g9f", "B*9g", "7i6h", "8d9d", "6f6e", "9d9f", "P*9h", "9g6d+", "6e6d",
        "9f9h+", "L*2g", "P*8f", "8g8f", "P*8g", "6h6g", "8g8h+", "7h6h", "8h8g", "7g6e", "8g8f",
        "5h5i", "L*5a", "4f4e", "5b4b", "6d5e", "5c5d", "5e4d", "4c4d", "4e4d", "S*3i", "2g2c+",
        "3b2c", "S*3a", "2b3a", "2h2c+", "P*2b", "2c2f", "S*4h", "B*4c", "L*3b", "5g4h", "3i4h",
        "5i4h", "8f7f", "6g5h", "P*4g", "P*7h", "4g4h+", "5h4h", "S*3e", "2f2e", "P*4g", "4h3i",
        "9h9e", "B*6d", "G*4a", "4c3b+", "4a3b", "S*4c", "B*6f", "3i2h", "6f4d", "4c3b+", "3a3b",
        "P*4c", "3b4c", "P*4e", "9e6e", "4e4d", "3c4d", "6d4b+", "4c4b", "P*6f", "6e6f", "P*6g",
        "N*2f", "S*4i", "S*3h", "L*3i", "3h3i", "2h3i", "L*2g", "2e2b", "4b4c", "S*3b", "4c5c",
        "2b2f", "7f6g", "2f2c", "2a3c", "B*4b", "5c6c", "2c2g", "6g6h", "G*6d", "6f6d", "N*7e",
        "6c7d", "G*7f", "B*5g", "3i2h", "6d7e", "7f7e", "5g7e+", "L*7g", "5a5c", "7g7e", "7d7e",
        "4b3c+", "P*2f", "B*5g", "L*6f", "2g2f", "P*2g", "2f2g", "4d3c", "N*8g", "7e8f", "5g3e",
        "6h7h", "3e5c+", "8f7g", "P*7i", "7g8h", "7i7h", "B*7i", "R*9f", "N*3e", "5c9g", "8h7h",
        "9f7f", "G*7g", "2g3h", "G*4h", "9g7i", "7h7i", "7f7g", "G*7h", "4i4h", "P*2g", "2h3i",
        "7h7g", "B*5g", "B*6h", "3h4i", "7i8h", "G*7i", "8h8g", "L*8i", "P*8h", "8i8h", "8g7f",
        "G*8f", "7f6e", "5g3e", "4g4h+", "4i4h", "S*3h", "4h3h", "6h3e+", "3h4h", "P*4g", "4h4g",
        "B*5g", "S*4h", "R*5i", "N*4i", "6f6g+", "7i7h", "N*5e", "5f5e", "7g7h", "G*5f", "6e6d",
        "P*6e", "6d6c", "4h5g", "G*2h", "3i4h", "3e5g", "4g5g", "6g5g", "5f5g", "5i4i+", "4h4i",
        "S*3h", "4i4h", "R*4i",
    ];

    let mut pos = Position::startpos();

    for move_str in MOVES {
        pos.make_move(Usi::parse_move(move_str).unwrap());
    }

    assert_eq!(
        Usi::format_position(&pos),
        "1n6l/3p2S2/2pk2s2/4p1p2/3PP3p/1G7/4G1PpP/1Lg2Ksg1/5r1NL b R2BSNL5Pn3p 257"
    );
}

#[test]
fn position_parse_and_format() {
    const SFENS: [&str; 20] = [
        "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1",
        "8l/1l+R2P3/p2pBG1pp/kps1p4/Nn1P2G2/P1P1P2PP/1PS6/1KSG3+r1/LN2+p3L w Sbgn3p 124",
        "lnsgkgsnl/1r7/p1ppp1bpp/1p3pp2/7P1/2P6/PP1PPPP1P/1B3S1R1/LNSGKG1NL b - 9",
        "l4S2l/4g1gs1/5p1p1/pr2N1pkp/4Gn3/PP3PPPP/2GPP4/1K7/L3r+s2L w BS2N5Pb 1",
        "6n1l/2+S1k4/2lp4p/1np1B2b1/3PP4/1N1S3rP/1P2+pPP+p1/1p1G5/3KG2r1 b GSN2L4Pgs2p 1",
        "l6nl/5+P1gk/2np1S3/p1p4Pp/3P2Sp1/1PPb2P1P/P5GS1/R8/LN4bKL w RGgsn5p 1",
        "l1r6/4S2k1/p2pb1gsg/5pp2/1pBnp3p/5PP2/PP1P1G1S1/6GKL/L1R5L b Ps3n5p 93",
        "5+P+B+R1/1kg2+P1+P+R/1g1s2KG1/3g4p/2p1pS3/1+p+l1s4/4B1N1P/9/4P4 b S3N3L9P 221",
        "ln3g1nl/1r1sg1sk1/p1p1ppbp1/1p1p2p1p/2P6/3P4P/PP2PPPP1/1BRS2SK1/LNG2G1NL b - 23",
        "l1+R4nk/5rgs1/3pp1gp1/p4pp1l/1p5Pp/4PSP2/P4PNG1/4G4/L5K1L w 2BP2s2n4p 88",
        "6B1+S/2gg5/4lp1+P1/6p1p/4pP1R1/Ppk1P1P1P/2+p2GK2/5S3/1+n3+r2L b B2SN2L2Pg2n4p 149",
        "7nl/3+P1kg2/4pb1ps/2r2NP1p/l1P2P1P1/s7P/PN2P4/KGB2G3/1N1R4L w G5P2sl2p 98",
        "l4Grnl/1B2+B1gk1/p1n3sp1/4ppp1p/P1S2P1P1/1PGP2P1P/3pP2g1/1K4sR1/LN6L w 3Psn 78",
        "ln6l/2gkgr1s1/1p1pp1n1p/3s1pP2/p8/1P1PBPb2/PS2P1NpP/1K1G2R2/LN1G4L w 3Psp 58",
        "ln1gk2nl/1rs3g2/p3ppspp/2pp2p2/1p5PP/2P6/PPSPPPP2/2G3SR1/LN2KG1NL b Bb 21",
        "ln7/1r2g1g2/2pspk1bn/pp1p2PB1/5pp1p/P1P2P3/1PSPP3+l/3K2S2/LN1G1G3 b Srnl3p 59",
        "4g2nl/5skn1/p1pppp1p1/6p+b1/4P4/3+R1SL1p/P3GPPP1/1+r2SS1KP/3PL2NL w GPbgn2p 128",
        "lnsgk2nl/1r4gs1/p1pppp1pp/6p2/1p5P1/2P6/PPSPPPP1P/7R1/LN1GKGSNL b Bb 13",
        "ln1g1gsnl/1r1s2k2/p1pp1p1p1/6p1p/1p7/2P5P/PPS+b1PPP1/2B3K2/LN1GRGSNL w P2p 26",
        "l2sk2nl/2g2s1g1/2n1pp1pp/pr4p2/1p6P/P2+b+RP1P1/1P2PSP2/5K3/L2G1G1NL b SPbn3p 51",
    ];

    for sfen in SFENS {
        assert_eq!(
            sfen,
            Usi::format_position(&Usi::parse_position(sfen).unwrap())
        );
    }
}
