import React from 'react';
import { Link } from 'react-router-dom';
import routes from '../constants/routes.json';
import styles from './Home.css';

export default function Home(): JSX.Element {
  return (
    <div className={styles.container} data-tid="container">
      <h2>STRATEGO</h2>
      <div className={styles.choiceContainer}>
        <button><Link to="/create">Create a new game</Link></button>
        <button><Link to="/join">Join a new game</Link></button>
        <button><Link to="/settings">Settings</Link></button>
        <button><Link to="/rules">Rules</Link></button>
        <button><Link to="/quit">Quit</Link></button>
      </div>
    </div>
  );
}
