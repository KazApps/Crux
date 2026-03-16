const TEST_SFENS: &[(&str, u64, u64)] = &[
    (
        "l4s2l/1+S1+Pskg2/4p1p+Rp/p3np1p1/6+rn1/2+BSPP3/PP2G3P/2GKN4/L7L b BN3Pg3p 123",
        15723,
        2055177,
    ),
    (
        "lr5+Pl/3gskg2/2n1p1s2/2pP1p1pp/P5p2/1PbSSPP1P/2N1PBN2/L1GK1G3/7RL w N2P2p 62",
        6478,
        434321,
    ),
    (
        "lnsg4l/2k6/ppp5p/1l1p1B3/5n3/7r1/PPPPPP1p+r/2KSpS3/LN2G1G2 b G3Pbsn2p 83",
        19053,
        1514149,
    ),
    (
        "+Bn5n1/7k1/p2pG1psl/2p1pp1pp/2PP1PG2/7LP/P1+r1+b2P1/6GSL/L+r2P1GNK b 3P2sn 117",
        6669,
        323238,
    ),
    (
        "+Bs6l/6g2/pp2kpnp1/1Np3s2/1NP2Pp1p/9/PS1P1GPPP/1KG1p1S2/L3R2NL b RG2Pblp 73",
        16374,
        1900946,
    ),
    (
        "l5k1l/3+P1gsp1/p2pps2p/9/1ps4NP/3PP4/P1p2S3/3BG4/LN1KN+r2L b RBN2g6p 89",
        16886,
        2286095,
    ),
    (
        "1n1gkg2l/1+Bs2p3/1pppp1n1p/l5rs1/p6p1/3P2p2/PP1GS3P/LKG4R1/1NS2b1NL b 5p 63",
        1385,
        54315,
    ),
    (
        "l6+Pl/3s3G1/p4p1S1/2p4p1/3ppP+R1k/P1P1G3+b/1G2NS3/3K5/LN6L w RBGS2N6P2p 122",
        352,
        11223,
    ),
    (
        "l4g1nl/3g1ks2/p1n1pp2p/2pB5/6S1b/P2Ps1P2/1P3PN1P/1SG6/LN1K1G1+rL b R2P5p 61",
        7027,
        587272,
    ),
    (
        "1rs1k4/l1g4bl/2npp1spn/ppp1g4/5pp1p/2PP5/PPN1PPPPP/1S2RGK2/L3BGSNL w - 80",
        665,
        23408,
    ),
    (
        "lnp2g1nk/5b2l/ppgps2sp/4ppR2/3P5/7pr/PPB1PPP1N/2S2KS2/LN1G1G2L b 4p 51",
        1572,
        64582,
    ),
    (
        "1+RR4nl/2+B4gk/2p1pssp1/5pp2/8p/2PPPPP2/1PN2SN2/+p4G3/1sK5L b 2G2L5Pbn 111",
        17472,
        2638135,
    ),
    (
        "ln5nl/1rg2kgs1/p4p1p1/2pp2p1p/9/PPP5P/1S1PPPP2/3K2SR1/2S2G1NL b 2B2Pgnlp 51",
        13793,
        1163184,
    ),
    (
        "lnk1g4/2s1l2R1/p1pp+L+B3/9/2P3P2/PpNP3pS/3KPP3/1P1S1+bg1S/LNg6 w 6Prgn 122",
        11167,
        1492818,
    ),
    (
        "ln7/2S5+S/p1p+R4p/2kpNpgp1/5b3/7r1/P1SPpPP1P/1PG6/LNK1L+s1NL w GPbg4p 82",
        419,
        56335,
    ),
    (
        "l6nl/6gk1/p1+B3sp1/2p1p1pP1/1p1n1rg2/8S/PPPP1pPS1/2G1PsGK1/LNR4NL b b4p 83",
        4021,
        167476,
    ),
    (
        "ln1g4l/r3k1gpb/pppp1p2p/5P3/2SN3+rB/1N1P2p1P/PPKSP4/2G6/L4+n2L w Sgs4p 96",
        11853,
        1294529,
    ),
    (
        "ln3g1nl/1P1sg1ks1/p1pppp1pp/6p2/P8/1BP6/Ln1PPPPPP/2s1+rgKR1/7NL b Pbgs 51",
        0,
        0,
    ),
    (
        "l3b2nl/1r2g1gk1/p1n1ps1p1/2p1spp1p/Pp5P1/2PPPPP1P/1PNSBSN2/2G1KG3/LR6L w p 58",
        1582,
        66677,
    ),
    (
        "l5s1l/1Rn1gkgp1/p4pn1p/1p1pp2P1/P1r3P2/LPpP5/2N1KPN1P/bS4S1B/2G2G2L b S2Pp 83",
        3384,
        261467,
    ),
    (
        "l2S1kbnl/4g2g1/3p3pp/1r3P3/pn1Pp1B2/2sSP3P/PP4N2/3KGG3/L+r7 w S4Pnl3p 116",
        15547,
        2055942,
    ),
    (
        "l6n1/6k2/2n1gsbsl/pr1p1pp2/1pp1R2p1/P2P1BG2/1PSG1P2+p/1KG6/LN6L w SPn5p 92",
        10228,
        719897,
    ),
    (
        "lnk4nl/7+r1/pppp4p/4g1p2/5P3/2PP5/PP2S1P1P/L1SG5/KN1G4+r b G2S3P2bnl2p 71",
        26731,
        3156304,
    ),
    (
        "l6nl/2g1ksg2/p1npppb1p/1Psr3p1/2p2S3/1G2PB3/P1NP1GN1P/3SK4/L6RL w 6p 78",
        2364,
        120631,
    ),
    (
        "ln5nl/sR1R1sgk1/p2+Ppp1p1/5sp1p/2pb2b2/P7P/4PPP2/1G2GKS2/L4G1NL w Pn4p 72",
        6325,
        457868,
    ),
    (
        "l4B1+L1/l8/N1+N3G2/Pr2p1pkp/1P1p1P1p1/2PSS3s/R1K1P2+p1/4G4/2G6 w GNL4Pbsn3p 162",
        40307,
        5970146,
    ),
    (
        "4S2nl/9/1k6p/1ppp2pp1/Pn3S3/1KP3P1P/2+rPPPR2/N8/3b4L b S3Pb4gsn2l2p 145",
        0,
        0,
    ),
    (
        "lnB4nl/1r2gkg2/p1p1pppp1/3p3sp/5B1N1/2s2P2P/PPNPP1P2/K1+s3S2/L4G1RL b 2Pgp 55",
        4707,
        244746,
    ),
    (
        "l2Rpk2l/2p1gsg2/p2s1pP1p/1+B5P1/1pPP1n1p1/2Sl1S3/PP1N3RP/3K5/1+b6L w N3P2gnp 80",
        11017,
        1189908,
    ),
    (
        "ln6l/1k1S2+Pp1/2g1+Sp2p/p1+r3p2/2S1+B2nP/P1p3P2/B2PPK3/4G4/1N5+lL w GNPrgs5p 112",
        32031,
        5022705,
    ),
    (
        "l4gknl/4g1s2/p1n1pp1p1/6p1p/9/2PSP1P1P/PP3PN2/1G1r1S3/LN2SK2L w R2B5Pg 60",
        11099,
        650092,
    ),
    (
        "ln1g4l/2sk3s1/1b1gp1npp/2Pp1rp2/p1p1P4/3PSPP2/PP2+B2PP/1KS1GR3/LN1G3NL w 2P 56",
        1375,
        49618,
    ),
    (
        "1l4+Rnl/2spP2gk/+P1pb1p3/6pgp/4p4/1pGB2P1P/2PG5/1KS6/LNNL+rPsP1 w sn4p 160",
        5934,
        650580,
    ),
    (
        "ln1gk2nl/1r4gs1/2p1+S4/p5spp/3p5/1pP2P2P/P1SPP1+p2/1PG3GR1/LN2K2NL b BPb3p 61",
        9517,
        657047,
    ),
    (
        "7nl/6gk1/n1rppgsp1/ps3pp1p/2pS3P1/PG1P2P1P/3SPP3/2KG3R1/LN5NL w BL2Pbp 68",
        10645,
        580927,
    ),
    (
        "l4k3/2g2sg2/pp+P1p1n1p/3pb4/3n3r1/1LR+BPSppP/P4P1S1/2SG1K2G/L6NL b N5Pp 123",
        5206,
        504746,
    ),
    (
        "ln1g4l/1ks6/pppsg1+B1p/7Pr/4spP2/P1P1p2pP/1P1P1PNG1/LS5R1/K1G3b1L b NPn2p 103",
        6911,
        462499,
    ),
    (
        "ln6l/1+PR4g1/4s1kp1/2lpn1p1p/ps1nG4/2pSp+bP1P/P+BGP5/N1PGP4/L1KS5 b 2Pr2p 131",
        5813,
        372350,
    ),
    (
        "l+R1Gs1knl/4gsgp1/p1BpNp2p/2s3pP1/7R1/5PP2/P+n+p1P3P/5G3/3s1K2L b NLPb4p 87",
        15746,
        1922300,
    ),
    (
        "kng4nl/ls1g1r3/ppps2b1p/6pp1/3p1pP2/2P1p4/PPSP1P2P/LSGB3R1/KNG4NL b 2p 53",
        814,
        20959,
    ),
    (
        "ln5nl/1+B2g1kp1/p2ppp2p/5s3/1ps2N2B/4SP3/PPSPP3P/2G4+p1/LN2K3L w RG2Prg2p 60",
        20792,
        2070247,
    ),
    (
        "2b1kp1R1/l1+Ps2gp1/3pp3l/p2nnPps1/n2GSS2p/PR1P2P2/2GKP3P/3G+n4/L7L b B3Pp 129",
        4682,
        454734,
    ),
    (
        "l7l/1r1sk4/p2p1gb+Rp/2p1pp3/5nN1P/1PPB2P2/P2P1P3/2S+n5/LNK2+p2L b 2GS3Pgsp 69",
        319,
        52249,
    ),
    (
        "l5S1l/1+R7/1n3G1g1/p1p+B2p1p/1NkPpN1R1/Pp+s3P1N/8P/5K3/L2G4L w G2S4Pb4p 198",
        605,
        60282,
    ),
    (
        "2p4kl/9/g1N1+Rs1p1/4K1N1p/p3Bp1P1/1Pn5P/2S6/2s2G3/8L b RB2GSN2L6P4p 159",
        343,
        137007,
    ),
    (
        "3s1rk1l/l3p1g2/p+R2g1npb/1p3pS1p/3+bN2Ps/4PN3/PKPP1P2P/L2G5/7+l+n b GS3P2p 115",
        404,
        49940,
    ),
    (
        "knsg1g1nl/5r3/lppppb+P1p/pl5R1/P6N1/4PP3/1s1PS4/1S1KG4/LN1G5 w B3P4p 68",
        7622,
        479963,
    ),
    (
        "8l/lg2k1gs1/nbppppnpp/r5p2/p8/1PPS2S1P/P1NP1PP2/2GKG2R1/L6NL w Sb3p 54",
        6607,
        406003,
    ),
    (
        "ln5nl/3+S1sgk1/p4sbp1/1r1g1pp2/3Pp3p/2p2BPP1/PP3PN1P/2G3SK1/LN2RG2L w 3Pp 62",
        2623,
        115563,
    ),
    (
        "l2gk2nl/1r3s3/p1np2g1p/2pbpp1p1/6SB1/1P5R1/PGNPPPP1P/4G4/L2K3NL w 2S3Pp 68",
        4826,
        222817,
    ),
    (
        "l5knl/4r1g2/5g1pp/2pp1p3/pp2b1PPP/2sP1s3/SPBG3S1/1KG6/LN2R2NL w N5Pp 90",
        4881,
        277173,
    ),
    (
        "l5gnk/7sl/6gpp/2p3p2/p6P1/1P2b3P/2PP1g1sN/2R1R4/LN1BSK2L b N6Pgs2p 105",
        14456,
        931101,
    ),
    (
        "l+P1g1g3/3spsk2/p6p1/3pP1p1l/3+r5/7N1/P2P1PPPP/+p2S1BS2/4GG2K w BNL3Pr2nlp 94",
        37834,
        6753820,
    ),
    (
        "ln1g3+S1/2kg4l/1psp1b1pp/6r2/p3B2P1/2P1PSp2/PP1P4P/1KS3G2/LN1G1P1NL b RN2P2p 91",
        6987,
        748859,
    ),
    (
        "l6nl/1B1gksg2/p1nsppppp/2r6/1p1p2S2/P3PB3/1P+lP1PN1P/1S4G1+r/LN1K2P2 b g3p 91",
        3676,
        164917,
    ),
    (
        "l6rl/b3gkg1n/4ps3/3p2p2/pp2Pp1Np/2pS2Pp1/PP1P+B1N1P/1KGG1S1R1/LN6L b s3p 93",
        3237,
        125421,
    ),
    (
        "lnR1g1k1l/6pss/p3sp2p/2pp2PP1/bp1n2S1P/2P1P1+R2/PP1PnP3/2G1G4/L2K4L b BGN2P 101",
        70,
        11275,
    ),
    (
        "lng2g1nl/2P2k3/p1+P1ppspp/1P1+R2p2/1g7/PS6P/1K2PPPP1/4+bS1R1/L+b1G3NL w SNP2p 52",
        7393,
        443139,
    ),
    (
        "l6+Pl/2+R2G3/p1pp1p1gp/1N1kp4/3bnnS2/2rP2P2/P1N1PP2P/2GSK4/L4G1+bL b 3P2s2p 59",
        6553,
        391757,
    ),
    (
        "l2g3nl/1ps2kg2/p3p3p/2pp1spp1/P4pB2/2S1P3P/2G3P2/3KS2+r1/L6NL w RBGNn5p 84",
        18855,
        1214267,
    ),
    (
        "4k1+R2/6+S2/3sg1pgp/1r1pp2P1/1pPP1KPp1/2p1PP1gP/PP3SN2/3+b5/2b2+n2L w G2N3L2Ps 128",
        466,
        34572,
    ),
    (
        "l2g3nl/1k1r3b1/4p2+R1/2s1PPP1p/G8/PS1p2pBP/1+sp1S4/3G5/K1P4NL b G2N3Pl4p 103",
        16512,
        2131381,
    ),
    (
        "2+N2g1nl/3gkrs2/6bpp/3p1pp2/1p2p2P1/4P4/1L1P1PP1P/P4S1R1/K1SG1G1NL b SPbnl3p 53",
        15584,
        1073853,
    ),
    (
        "l1lk4l/1s1g1S+R2/p+P1ppp1p1/4g1Nbp/2B2n3/2PSP1P2/P1GP1P2P/2K3+p2/LRS2+n1+n1 w 3Pg 98",
        5474,
        368038,
    ),
    (
        "ln4k1l/1S1+P1sg2/p3p2p1/7bp/2pNPp3/P1P1b3P/1P3PPP1/2G1G1K2/L3R2NL w RGN2s3p 70",
        17954,
        1728634,
    ),
    (
        "ln1g4l/1ks6/ppp4pp/3p2R2/P8/2P1p4/1PSP1+p2P/2K1GS3/LN1G3+b1 b RGSNPbnl4p 69",
        47940,
        10721488,
    ),
    (
        "l3+P2nl/5sk2/p2Sppgp1/6p2/P4P1Pp/1pSs2P2/4P1N1P/1PG1G3R/LNK2bb1L w RGN2Pp 82",
        9736,
        509960,
    ),
    (
        "lnRRs2nl/1+P2kb1g1/p1pg1ppsp/3p5/1p4P2/2P6/P1BPPP2P/2S6/LN1K1GS1L w Gn3p 60",
        6522,
        311716,
    ),
    (
        "ln1Rsk3/2g3g2/p2ppp1pp/2P3p2/+BS1P5/4P4/PP1g2P1P/3+s4B/LN1K2gNL b SNL4Pr 73",
        80,
        14112,
    ),
    (
        "l2r5/1+Lsg1+R3/p2kpp3/2pps1+B1p/S5Pn1/2PL1P3/P2P+s3P/2G4+b1/LNK2+n3 w GN6Pg 140",
        13268,
        891926,
    ),
    (
        "l4k1nl/3rg1g2/p3bp1pp/1p2ssp2/2Pp5/PBsPGSP2/1P3PN1P/2G4R1/LNK5L b N2P2p 61",
        4598,
        340047,
    ),
    (
        "ln1g1l2l/2k1s4/1+bpp5/p2n2+B1p/3S2p2/PP1PS3P/2G2N3/4KG3/+l6+R1 w RS3Pgn6p 134",
        27137,
        3185184,
    ),
    (
        "1n1k3nl/3p3p1/pPp+P1+B1gp/5pPR1/2n6/Ps1sPP2P/4bS1GK/2S6/+pN4g1L w RG2L3Pp 122",
        610,
        29628,
    ),
    (
        "l2g2s1l/2s1k1g2/p2pp1npp/2p1Sp3/6p2/1n1BPK3/3G2NSP/1+r1+n1L3/5+r2L w bg8p 122",
        5439,
        608426,
    ),
    (
        "2S4nl/+r+P2SG1p1/n1N+N1p1gp/1+P+R6/1p1Pp4/G1KS2k2/P8/9/L1P+p2+bL1 b B4Pgsl3p 187",
        27526,
        3398797,
    ),
    (
        "lbn4nk/2p3+B1l/p6pp/4ppp2/8P/1p3PP2/P4GNR1/SP3SK2/L4s2L b RGSNP2g4p 131",
        309,
        74983,
    ),
    (
        "3g1g1nl/3l1ks2/p3pp1pp/2P1n1p2/2GP+B4/3r1P2+r/PPK1P1P1N/1SGN2S2/1+b4S1L w 3Pl2p 84",
        4483,
        360189,
    ),
    (
        "1n1g4l/1ksg5/1sp1p+B+Ppp/1p1p5/2P1N1rP1/2S1PP+b2/1P1P4P/+l1K1G2R1/1N1G3NL b Sl4p 65",
        8954,
        625286,
    ),
    (
        "l+B3k1nl/4r1g2/p2p1p1pp/2ps5/4pn3/1SP1+b1P2/PP1P4P/2G1PSG2/LN2KG1RL w N3Psp 60",
        8280,
        587445,
    ),
    (
        "l6nl/3g1kgs1/2s1pp1p1/3p1Pp1p/P2n5/2r6/1PNPPGN1P/LG1K2S2/2S4RL w B4Pb2p 68",
        10723,
        906877,
    ),
    (
        "ln2k1snl/1r2g1g2/p4p1p1/1pps4p/5SP2/1+b2P3P/P2P1P1P1/2+l1GKGR1/+n5SNL w Pb4p 56",
        3572,
        340136,
    ),
    (
        "ln5n1/1r3gk2/p2gsp1p1/2pp1bp2/1p3s1P1/2RPpP2l/PPP3P2/1BGSG4/LN4KN1 w sl3p 52",
        3758,
        426734,
    ),
    (
        "lr6l/4g1gk1/2n1pbnpp/p1pp2p2/5P1PP/P1P1R1P2/1pNP1S3/2G2GK2/LP6L b B2SPsnp 71",
        17432,
        2083868,
    ),
    (
        "l7l/4P2g1/r1+P5p/p2+SSp1k1/1pg4n1/2+b4G1/P1NP1PPNP/1P1S+pS3/L3G2KL b Prbn4p 113",
        10076,
        505218,
    ),
    (
        "l+B6l/4+Rngk1/p1n2pP1p/2pp3S1/9/Ps7/3PP3P/KG1+b5/L7L w RSN6P2gsn2p 144",
        44828,
        7814349,
    ),
    (
        "6k1l/4g4/p2spp1pp/2p1l2s1/2Gp1bpN1/P5nPP/3PP4/1PGSG4/LNK4+bL b RSN2Pr2p 93",
        14876,
        2059050,
    ),
    (
        "l1sg3nl/4gkgs1/pPnpp2pp/5pP2/9/P2P1R2P/3bPPK2/2p6/2R2G1NL b B2SLPn3p 79",
        16604,
        2611339,
    ),
    (
        "kn1g5/lss5l/pp1Pbpn1p/2Gs2rp1/P1pppPp2/1PP2G3/1SB1P1P1P/1KG6/LN1R3NL w p 72",
        1232,
        36020,
    ),
    (
        "l+R2nk1nl/7s1/p3pg1pp/3p1pp2/9/P1pPP3P/5PPP1/3S1K3/L+r2G1SNL b GN2bgs3p 89",
        23083,
        2146354,
    ),
    (
        "ln1g4l/1ks3gb1/1ppp1pnpp/p3r4/4P1S2/2PG1P3/PP1P4P/1BK3R2/LN1G3NL w SPs3p 70",
        8033,
        530448,
    ),
    (
        "1+R1ngk1nl/5Ss2/2pp1+B2p/p5p2/9/9/P2PPPP1P/1sS1KG3/+b2G4+n w RG3L4Pn3p 80",
        224,
        31683,
    ),
    (
        "ln3k2l/1r1sg1r2/p1ppp1+Bpp/1p3pN2/4+b4/2P6/PP2PPnPP/3K1S3/LNSG1s3 b 2GPl2p 51",
        6848,
        563600,
    ),
    (
        "l3k3+B/1r1s2s2/p1n1pg1p1/2ppn1Psp/9/1PP4RP/P1NPPS3/2K6/L2G4L w 2GNL3Pb2p 80",
        18513,
        1408224,
    ),
    (
        "l6nl/4gkgp1/3pp1sNp/p1s2p1P1/1r4p1P/P1p1SP3/1P1PP1P2/2G4R1/L3KG1NL b BNbs2p 75",
        15947,
        1413888,
    ),
    (
        "1n1g2r1l/1ks1g4/pppp1s+P1p/l2b3S1/5p3/PPP1pPR2/3PB3P/1SK6/LN1G4L b 2NPg3p 79",
        9651,
        941674,
    ),
    (
        "l4+R1nl/1ks6/pn1gp3p/1Lp3p2/Pp1p3s1/2P1S+bPP1/1P1PP3P/2GS5/L1K1G4 b GN2Prbnp 85",
        22195,
        2249236,
    ),
    (
        "ln5+Rl/3s1k3/p1ppp1g1p/5p3/1N5p1/2P3P2/PP1PPP2P/1SGK2+br1/L4GGNL b SNbs3p 59",
        16177,
        1484876,
    ),
    (
        "ln4s2/1r1sk2bl/pp2g1n1p/2p4p1/3p1p3/3sg1P2/PP5PP/3Kr4/LNS1g3g b bnl6p 53",
        0,
        0,
    ),
    (
        "l4+S2l/5g1k1/p2Bp2pp/1b4g2/1p3p3/2P1P3P/PP1P1PPP1/4G1S1K/L4+rsNL b R3Pgs3n 89",
        14798,
        842381,
    ),
    (
        "ln1g4l/1ks3gbp/p1ppp4/4spr1P/7P1/2P1PBp2/PPNP1PS2/2GS3R1/L1K2G1NL b N3p 67",
        3170,
        204675,
    ),
];

use std::time::Instant;

use crux_lib::{
    notation::{usi::Usi, Notation},
    shogi::{
        movegen::{generate, is_legal, is_pseudo_legal},
        position::Position,
    },
};

#[test]
fn perft2() {
    for (sfen, expected, _) in TEST_SFENS {
        let mut pos = Usi::parse_position(sfen).unwrap();
        assert_eq!(perft::<false>(&mut pos, 2), *expected);
    }
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn perft3() {
    for (sfen, _, expected) in TEST_SFENS {
        let mut pos = Usi::parse_position(sfen).unwrap();
        assert_eq!(perft::<false>(&mut pos, 3), *expected);
    }
}

#[test]
#[ignore]
fn perft_speedtest() {
    let mut total_nodes = 0;

    let start = Instant::now();

    for (sfen, _, _) in TEST_SFENS {
        let mut pos = Usi::parse_position(sfen).unwrap();
        total_nodes += perft::<true>(&mut pos, 4);
    }

    let elapsed = start.elapsed();
    let nps = total_nodes as f64 / elapsed.as_secs_f64();

    println!("nodes: {}", total_nodes);
    println!("time: {:?}", elapsed);
    println!("nps: {:.0}", nps);
}

fn perft<const SPEED_TEST: bool>(pos: &mut Position, depth: i32) -> u64 {
    let mut total = 0;

    for mv in generate(pos) {
        if !SPEED_TEST {
            assert!(is_pseudo_legal(pos, mv));
        }

        if is_legal(pos, mv) {
            if depth == 1 {
                total += 1;
            } else {
                let captured = pos.make_move(mv);
                total += perft::<SPEED_TEST>(pos, depth - 1);
                pos.unmake_move(mv, captured);
            }
        }
    }

    total
}
