import React from 'react';
import {Link} from 'react-router-dom';
import routes from '../../constants/routes.json';

export default function Rules() {
  return (
    <div>
      <div data-tid="backButton">
        <Link to={routes.HOME}>
          <i className="fa fa-arrow-left fa-3x"/>
        </Link>
      </div>
      <h1>PAGE RULES </h1>
      <h2>I. STRATEGO GAME</h2>
      <p>
        Stratego is a game with imperfect information invented by
        the dutch Jacques Johan Mogendorff in 1942 [1]. The classical
        game takes place on a board of size 10x10. The goal is to
        capture the enemy’s flag [2]. In the centre of the board there
        are two lakes of size 2x2, where the pieces are not allowed.
        There are two players: red and blue. Each player has 40 pieces,
        initially placed in a rectangular area of size 4x10. The players
        can choose the way they place their pieces.
      </p>
      <p>
        There are two types of pieces: mobile and immobile. Immo-
        bile pieces are the flag and the bombs. Capturing the enemy’s
      </p>
      <p>
        flag means winning the game. By attacking the bomb of the
        enemy, the agent loses its piece, excepting the case when the
        attacker is a miner (e.g. a mobile piece).
        Mobile pieces have different ranks which are numbers
        between 1 and 10. Pieces of rank 1 are called spies and are
        the only pieces that can defeat the enemy’s piece of rank 10,
        but only when the spy is the one who is attacking. Pieces of
        rank 2 are called scouts and are the only pieces that can be
        moved on a distance greater than one square at a time. The
        way those pieces are moved is similar with the way the rook
        moves on a chess board: it can be moved on any square in
        the 4 directions (up, left, right, down) with the condition they
        do not jump over another piece or over a lake. The pieces of
        rank 3 are called miners with the role to diffuse bombs.
        For attacking a piece, the attacker must be moved in the
        cell where there is the enemy’s piece. The conflict between
        two pieces results in removing the piece with the lower rank
        or removing both of them if their ranks are equal. Attacking
        a piece also leads to revealing the rank of the two involved
        pieces. The game ends when one of the players captures the
        opponent’s flag. Also, the game ends if one of the players do
        not have what to move.
      </p>
    </div>
  );
}
