use arrayvec::ArrayVec;

use crate::shogi::{
    attacks::{
        bishop_attacks, dragon_attacks, gold_attacks, horse_attacks, king_attacks, knight_attacks,
        lance_attacks, multi_pawn_attacks, pawn_attacks, piece_attacks, ray_between,
        ray_intersecting, rook_attacks, silver_attacks,
    },
    bitboard::{pawn_drop_mask, promotion_area, Bitboard},
    core::{Color, PieceType, Rank, Square},
    position::{mv::Move, Position},
};

/// Fixed-capacity buffer for pseudo-legal moves.
///
/// The capacity (600) exceeds the theoretical maximum number of legal
/// moves in shogi (593), ensuring it never overflows.
pub type MoveList = ArrayVec<Move, 600>;

/// Generates pseudo-legal moves for the given position.
///
/// "Pseudo-legal" moves include:
/// - All legal moves
/// - Moves that put the king in check
/// - Illegal pawn drops (uchifuzume)
///
/// Note: Before passing these moves to [`Position::make_move`], you should
/// verify legality using [`is_legal`].
#[must_use]
pub fn generate(pos: &Position) -> MoveList {
    let mut moves = MoveList::new();

    let stm = pos.side_to_move();
    let checkers = pos.checkers();

    let mut dst_mask = !pos.color_bb(stm);
    let mut drop_mask = !pos.occupancy();

    generate_kings(&mut moves, pos, dst_mask);

    if checkers.is_multiple() {
        return moves;
    }

    if checkers.has_any() {
        let checker = checkers.lsb();
        let check_ray = ray_between(pos.king_square(stm).unwrap(), checker);

        dst_mask &= check_ray | checker.bit();
        drop_mask &= check_ray;
    }

    generate_pawns(&mut moves, pos, dst_mask);
    generate_lances(&mut moves, pos, dst_mask);
    generate_knights(&mut moves, pos, dst_mask);
    generate_silvers(&mut moves, pos, dst_mask);
    generate_golds(&mut moves, pos, dst_mask);
    generate_bishops(&mut moves, pos, dst_mask);
    generate_rooks(&mut moves, pos, dst_mask);
    generate_horses(&mut moves, pos, dst_mask);
    generate_dragons(&mut moves, pos, dst_mask);
    generate_drops(&mut moves, pos, drop_mask);

    moves
}

/// Returns `true` if `mv` is a legal move in `pos`.
///
/// Only pseudo-legal moves should be passed to this function.
/// This checks full move legality, but does **not** consider perpetual checks.
///
/// # Debug assertions
/// In debug builds, panics if `mv` is not pseudo-legal in `pos`.
#[must_use]
pub fn is_legal(pos: &mut Position, mv: Move) -> bool {
    debug_assert!(is_pseudo_legal(pos, mv));

    let to = mv.to();
    let stm = pos.side_to_move();
    let nstm = stm.opposite();

    if mv.is_drop() {
        debug_assert!(!pos.checkers().is_multiple());

        if mv.drop_piece_type() == PieceType::Pawn
            && let Some(king_square) = pos.king_square(nstm)
            && pawn_attacks(stm, to).contains(king_square)
        {
            let mut legal = false;

            pos.make_move(mv);

            if generate(pos).iter().any(|&mv| is_legal(pos, mv)) {
                legal = true;
            }

            pos.unmake_move(mv, None);

            return legal;
        }

        return true;
    }

    let from = mv.from();
    let moving_piece = pos.piece_at(from).unwrap();

    if moving_piece.piece_type() == PieceType::King {
        let king_square = pos.king_square(stm).unwrap();
        let nstm_pieces = pos.color_bb(nstm);

        let pawns = pos.piece_bb(PieceType::Pawn.with_color(nstm));
        let lances = pos.piece_bb(PieceType::Lance.with_color(nstm));
        let knights = pos.piece_bb(PieceType::Knight.with_color(nstm));
        let silvers = pos.piece_bb(PieceType::Silver.with_color(nstm));
        let golds = (pos.piece_type_bb(PieceType::Gold)
            | pos.piece_type_bb(PieceType::ProPawn)
            | pos.piece_type_bb(PieceType::ProLance)
            | pos.piece_type_bb(PieceType::ProKnight)
            | pos.piece_type_bb(PieceType::ProSilver))
            & nstm_pieces;
        let horses = pos.piece_type_bb(PieceType::Horse);
        let dragons = pos.piece_type_bb(PieceType::Dragon);
        let bishops = (pos.piece_type_bb(PieceType::Bishop) | horses) & nstm_pieces;
        let rooks = (pos.piece_type_bb(PieceType::Rook) | dragons) & nstm_pieces;
        let kings = (pos.piece_type_bb(PieceType::King) | horses | dragons) & nstm_pieces;

        let occ = pos.occupancy() ^ king_square.bit();

        if (pawn_attacks(stm, to) & pawns).has_any() {
            return false;
        }

        if (lance_attacks(stm, to, occ) & lances).has_any() {
            return false;
        }

        if (knight_attacks(stm, to) & knights).has_any() {
            return false;
        }

        if (silver_attacks(stm, to) & silvers).has_any() {
            return false;
        }

        if (gold_attacks(stm, to) & golds).has_any() {
            return false;
        }

        if (bishop_attacks(to, occ) & bishops).has_any() {
            return false;
        }

        if (rook_attacks(to, occ) & rooks).has_any() {
            return false;
        }

        if (king_attacks(to) & kings).has_any() {
            return false;
        }
    }

    if pos.pinned().contains(from) {
        let king_square = pos.king_square(stm).unwrap();
        let pin_ray = ray_intersecting(from, king_square);

        if !pin_ray.contains(to) {
            return false;
        }
    }

    true
}

/// Returns `true` if `mv` is pseudo-legal in `pos`.
///
/// See [`generate`] for the definition of pseudo-legal moves.
#[must_use]
pub const fn is_pseudo_legal(pos: &Position, mv: Move) -> bool {
    let to = mv.to();
    let stm = pos.side_to_move();
    let checkers = pos.checkers();

    const fn promotion_required_zone(color: Color, piece_type: PieceType) -> Bitboard {
        let mut zone = Bitboard::empty();

        if matches!(
            piece_type,
            PieceType::Pawn | PieceType::Lance | PieceType::Knight
        ) {
            zone |= Rank::Rank1.relative(color).bit();
        }

        if piece_type == PieceType::Knight {
            zone |= Rank::Rank2.relative(color).bit();
        }

        zone
    }

    if mv.is_drop() {
        if pos.has_any(to) {
            return false;
        }

        if checkers.is_multiple() {
            return false;
        }

        let hand = pos.hand(stm);
        let piece_type = mv.drop_piece_type();

        if hand.count(piece_type) == 0 {
            return false;
        }

        if promotion_required_zone(stm, piece_type).contains(to) {
            return false;
        }

        if piece_type == PieceType::Pawn
            && !pawn_drop_mask(stm, pos.piece_bb(PieceType::Pawn.with_color(stm))).contains(to)
        {
            return false;
        }

        if checkers.has_any() {
            let king_square = pos.king_square(stm).unwrap();
            let check_ray = ray_between(king_square, checkers.lsb());

            if !check_ray.contains(to) {
                return false;
            }
        }

        return true;
    }

    let from = mv.from();
    let moving_piece = match pos.piece_at(from) {
        Some(piece) => piece,
        None => return false,
    };

    if moving_piece.color() != stm {
        return false;
    }

    if checkers.is_multiple() && moving_piece.piece_type() != PieceType::King {
        return false;
    }

    if let Some(captured) = pos.piece_at(to)
        && captured.color() == stm
    {
        return false;
    }

    if mv.is_promotion() {
        if !moving_piece.can_promote()
            || (!from.rank().can_promote(stm) && !to.rank().can_promote(stm))
        {
            return false;
        }
    } else {
        if promotion_required_zone(stm, moving_piece.piece_type()).contains(to) {
            return false;
        }
    }

    if checkers.has_any() && moving_piece.piece_type() != PieceType::King {
        let king_square = pos.king_square(stm).unwrap();
        let checker = checkers.lsb();
        let check_ray = ray_between(king_square, checker) | checker.bit();

        if !check_ray.contains(to) {
            return false;
        }
    }

    let attacks = piece_attacks(moving_piece, from, pos.occupancy());

    if !attacks.contains(to) {
        return false;
    }

    true
}

fn generate_pawns(dst: &mut MoveList, pos: &Position, dst_mask: Bitboard) {
    let stm = pos.side_to_move();
    let pawns = pos.piece_bb(PieceType::Pawn.with_color(stm));
    let attacks = multi_pawn_attacks(stm, pawns);
    let non_promo_restriction = attacks & !Rank::Rank1.relative(stm).bit();

    serialize_pawn_normals(dst, stm, non_promo_restriction, dst_mask);
    serialize_pawn_promotions(dst, stm, attacks & promotion_area(stm), dst_mask);
}

fn generate_lances(dst: &mut MoveList, pos: &Position, dst_mask: Bitboard) {
    let stm = pos.side_to_move();
    let lances = pos.piece_bb(PieceType::Lance.with_color(stm));

    generate_precalculated_with_color_and_occ::<true, _>(
        dst,
        pos,
        lances,
        lance_attacks,
        dst_mask,
        !Rank::Rank1.relative(stm).bit(),
    );
}

fn generate_knights(dst: &mut MoveList, pos: &Position, dst_mask: Bitboard) {
    let stm = pos.side_to_move();
    let knights = pos.piece_bb(PieceType::Knight.with_color(stm));

    generate_precalculated_with_color::<true, _>(
        dst,
        pos,
        knights,
        knight_attacks,
        dst_mask,
        !(Rank::Rank1.relative(stm).bit() | Rank::Rank2.relative(stm).bit()),
    );
}

fn generate_silvers(dst: &mut MoveList, pos: &Position, dst_mask: Bitboard) {
    let silvers = pos.piece_bb(PieceType::Silver.with_color(pos.side_to_move()));

    generate_precalculated_with_color::<true, _>(
        dst,
        pos,
        silvers,
        silver_attacks,
        dst_mask,
        Bitboard::all(),
    );
}

fn generate_golds(dst: &mut MoveList, pos: &Position, dst_mask: Bitboard) {
    let stm = pos.side_to_move();
    let golds = pos.piece_bb(PieceType::Gold.with_color(stm))
        | pos.piece_bb(PieceType::ProPawn.with_color(stm))
        | pos.piece_bb(PieceType::ProLance.with_color(stm))
        | pos.piece_bb(PieceType::ProKnight.with_color(stm))
        | pos.piece_bb(PieceType::ProSilver.with_color(stm));

    generate_precalculated_with_color::<false, _>(
        dst,
        pos,
        golds,
        gold_attacks,
        dst_mask,
        Bitboard::all(),
    );
}

fn generate_bishops(dst: &mut MoveList, pos: &Position, dst_mask: Bitboard) {
    let bishops = pos.piece_bb(PieceType::Bishop.with_color(pos.side_to_move()));

    generate_precalculated_with_occ::<true, _>(
        dst,
        pos,
        bishops,
        bishop_attacks,
        dst_mask,
        Bitboard::all(),
    );
}

fn generate_rooks(dst: &mut MoveList, pos: &Position, dst_mask: Bitboard) {
    let rooks = pos.piece_bb(PieceType::Rook.with_color(pos.side_to_move()));

    generate_precalculated_with_occ::<true, _>(
        dst,
        pos,
        rooks,
        rook_attacks,
        dst_mask,
        Bitboard::all(),
    );
}

fn generate_horses(dst: &mut MoveList, pos: &Position, dst_mask: Bitboard) {
    let horses = pos.piece_bb(PieceType::Horse.with_color(pos.side_to_move()));

    generate_precalculated_with_occ::<false, _>(
        dst,
        pos,
        horses,
        horse_attacks,
        dst_mask,
        Bitboard::all(),
    );
}

fn generate_dragons(dst: &mut MoveList, pos: &Position, dst_mask: Bitboard) {
    let dragons = pos.piece_bb(PieceType::Dragon.with_color(pos.side_to_move()));

    generate_precalculated_with_occ::<false, _>(
        dst,
        pos,
        dragons,
        dragon_attacks,
        dst_mask,
        Bitboard::all(),
    );
}

fn generate_kings(dst: &mut MoveList, pos: &Position, dst_mask: Bitboard) {
    let kings = pos.piece_bb(PieceType::King.with_color(pos.side_to_move()));
    generate_precalculated::<false, _>(dst, pos, kings, king_attacks, dst_mask, Bitboard::all());
}

fn generate_drops(dst: &mut MoveList, pos: &Position, dst_mask: Bitboard) {
    let stm = pos.side_to_move();
    let hand = pos.hand(stm);

    let mut generate = |piece_type: PieceType, restriction: Bitboard| {
        if hand.count(piece_type) > 0 {
            serialize_drops(dst, piece_type, restriction, dst_mask);
        }
    };

    let pawns = pos.piece_bb(PieceType::Pawn.with_color(stm));
    let pawn_restriction = pawn_drop_mask(stm, pawns);
    let lance_restriction = !Rank::Rank1.relative(stm).bit();
    let knight_restriction = !(Rank::Rank1.relative(stm).bit() | Rank::Rank2.relative(stm).bit());

    generate(PieceType::Pawn, pawn_restriction);
    generate(PieceType::Lance, lance_restriction);
    generate(PieceType::Knight, knight_restriction);
    generate(PieceType::Silver, Bitboard::all());
    generate(PieceType::Gold, Bitboard::all());
    generate(PieceType::Bishop, Bitboard::all());
    generate(PieceType::Rook, Bitboard::all());
}

fn generate_precalculated<const CAN_PROMOTE: bool, F: Fn(Square) -> Bitboard>(
    dst: &mut MoveList,
    pos: &Position,
    pieces: Bitboard,
    attack_getter: F,
    dst_mask: Bitboard,
    non_promo_restriction: Bitboard,
) {
    generate_precalculated_with_color_and_occ::<CAN_PROMOTE, _>(
        dst,
        pos,
        pieces,
        |_color, square, _occ| -> Bitboard { attack_getter(square) },
        dst_mask,
        non_promo_restriction,
    );
}

fn generate_precalculated_with_color<const CAN_PROMOTE: bool, F: Fn(Color, Square) -> Bitboard>(
    dst: &mut MoveList,
    pos: &Position,
    pieces: Bitboard,
    attack_getter: F,
    dst_mask: Bitboard,
    non_promo_restriction: Bitboard,
) {
    generate_precalculated_with_color_and_occ::<CAN_PROMOTE, _>(
        dst,
        pos,
        pieces,
        |color, square, _occ| -> Bitboard { attack_getter(color, square) },
        dst_mask,
        non_promo_restriction,
    );
}

fn generate_precalculated_with_occ<const CAN_PROMOTE: bool, F: Fn(Square, Bitboard) -> Bitboard>(
    dst: &mut MoveList,
    pos: &Position,
    pieces: Bitboard,
    attack_getter: F,
    dst_mask: Bitboard,
    non_promo_restriction: Bitboard,
) {
    generate_precalculated_with_color_and_occ::<CAN_PROMOTE, _>(
        dst,
        pos,
        pieces,
        |_color, square, occ| -> Bitboard { attack_getter(square, occ) },
        dst_mask,
        non_promo_restriction,
    );
}

fn generate_precalculated_with_color_and_occ<
    const CAN_PROMOTE: bool,
    F: Fn(Color, Square, Bitboard) -> Bitboard,
>(
    dst: &mut MoveList,
    pos: &Position,
    pieces: Bitboard,
    attack_getter: F,
    dst_mask: Bitboard,
    non_promo_restriction: Bitboard,
) {
    let stm = pos.side_to_move();
    let occ = pos.occupancy();
    let promo_area = promotion_area(stm);

    let mut movable = pieces;

    while movable.has_any() {
        let from = movable.pop_lsb();
        let attacks = attack_getter(stm, from, occ) & non_promo_restriction;

        serialize_normals(dst, from, attacks, dst_mask);
    }

    if CAN_PROMOTE {
        let mut promotable = pieces;

        while promotable.has_any() {
            let from = promotable.pop_lsb();
            let attacks = attack_getter(stm, from, occ) & promo_area;

            serialize_promotions(dst, from, attacks, dst_mask);
        }

        promotable = pieces & promo_area;

        while promotable.has_any() {
            let from = promotable.pop_lsb();
            let attacks = attack_getter(stm, from, occ) & !promo_area;

            serialize_promotions(dst, from, attacks, dst_mask);
        }
    }
}

fn serialize_normals(dst: &mut MoveList, from: Square, mut attacks: Bitboard, dst_mask: Bitboard) {
    attacks &= dst_mask;

    while attacks.has_any() {
        let to = attacks.pop_lsb();
        dst.push(Move::normal(from, to));
    }
}

fn serialize_pawn_normals(
    dst: &mut MoveList,
    color: Color,
    mut attacks: Bitboard,
    dst_mask: Bitboard,
) {
    attacks &= dst_mask;

    while attacks.has_any() {
        let to = attacks.pop_lsb();
        let from = to.relative_south(color);
        dst.push(Move::normal(from, to));
    }
}

fn serialize_promotions(
    dst: &mut MoveList,
    from: Square,
    mut attacks: Bitboard,
    dst_mask: Bitboard,
) {
    attacks &= dst_mask;

    while attacks.has_any() {
        let to = attacks.pop_lsb();
        dst.push(Move::promote(from, to));
    }
}

fn serialize_pawn_promotions(
    dst: &mut MoveList,
    color: Color,
    mut attacks: Bitboard,
    dst_mask: Bitboard,
) {
    attacks &= dst_mask;

    while attacks.has_any() {
        let to = attacks.pop_lsb();
        let from = to.relative_south(color);
        dst.push(Move::promote(from, to));
    }
}

fn serialize_drops(
    dst: &mut MoveList,
    piece_type: PieceType,
    mut targets: Bitboard,
    dst_mask: Bitboard,
) {
    targets &= dst_mask;

    while targets.has_any() {
        let target = targets.pop_lsb();
        dst.push(Move::drop(piece_type, target));
    }
}
