import React, { useState } from 'react';
import { Link, Redirect } from 'react-router-dom';
import routes from '../../constants/routes.json';

export default function Join({ socket }) {
  const [code, setCode] = useState('');
  const [foundGame, setFoundGame] = useState(false);

  function handleChangeCode(event) {
    setCode(event.target.value);
  }

  function checkCode() {
    // eslint-disable-next-line react/prop-types
    socket.emit('join-game', code);
  }

  // eslint-disable-next-line react/prop-types
  socket.on('response-join-game', (response) => {
    if (!foundGame) {
      setFoundGame(response.valid);
    }
  });

  return (
    <div>
      {foundGame ? <Redirect to="/settings" /> : null}
      <div data-tid="backButton">
        <Link to={routes.HOME}>
          <i className="fa fa-arrow-left fa-3x" />
        </Link>
      </div>
      <div>Join a game page </div>
      {/* eslint-disable-next-line jsx-a11y/label-has-associated-control */}
      <label>Code de la partie</label>
      <input
        type="text"
        value={code}
        onChange={handleChangeCode}
        id="gameCode"
        name="gameCode"
      />
      <br />
      {/* eslint-disable-next-line react/button-has-type */}
      <button onClick={checkCode}>Rejoindre la partie</button>
    </div>
  );
}
