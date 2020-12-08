import React from 'react';
import { Link } from 'react-router-dom';
import routes from '../../constants/routes.json';

export default function Join() {
  return (
    <div>
      <div data-tid="backButton">
        <Link to={routes.HOME}>
          <i className="fa fa-arrow-left fa-3x" />
        </Link>
      </div>
      <div>Join a game page </div>
      <form action="" method="POST">
        <label>Code de la partie</label>
        <input type="text" id="gameCode" name="gameCode"/>
      </form>
    </div>
  );
}
