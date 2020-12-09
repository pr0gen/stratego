import React, { useState } from 'react';
import { Link } from 'react-router-dom';
import styles from './Counter.css';
import routes from '../../constants/routes.json';

export default function Create({ socket }) {
  const [code, setCode] = useState('');

  if (code === '') {
    // eslint-disable-next-line react/prop-types
    socket.emit('create-game');
  }

  socket.on('response-create-game', (code: string) => {
    setCode(code);
  });

  const quitGame = () => {
    socket.emit('leave-game');
  };

  return (
    <div>
      <div className={styles.backButton} data-tid="backButton">
        <Link onClick={quitGame} to={routes.HOME}>
          <i className="fa fa-arrow-left fa-3x" />
        </Link>
      </div>
      <div>
        <br />
        <br />
        <h1>Votre code est : {code}</h1>
        <p> Vous serez redirigé lorsque un joueur rejoindra la partie</p>
      </div>
    </div>
  );
}
