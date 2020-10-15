import React from 'react';
import styles from './Home.css';
import { Link } from 'react-router-dom';
import routes from '../constants/routes.json';


export default function Home(): JSX.Element {
  return (
    <div className={styles.container} data-tid="container">
      <h2>STRATEGO</h2>
      <div className={styles.choiceContainer}>
        <button><Link to={routes.CREATE_GAME}>Create a new game</Link></button>
        <button><Link to={routes.JOIN_GAME}>Join a new game</Link></button>
        <button><Link to={routes.SETTINGS}>Settings</Link></button>
        <button><Link to={routes.RULES}>Rules</Link></button>
        <button><Link to={routes.QUIT}>Quit</Link></button>
      </div>
    </div>
  );
}
