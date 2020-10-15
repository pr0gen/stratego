/* eslint react/jsx-props-no-spreading: off */
import React from 'react';
import { Switch, Route } from 'react-router-dom';
import routes from './constants/routes.json';
import App from './containers/App';
import HomePage from './containers/HomePage';
import CreatePage from "./containers/CreatePage";
import RulesPage from "./containers/RulesPage";
import SettingsPage from "./containers/SettingsPage";
import QuitPage from "./containers/QuitPage";
import JoinPage from "./containers/JoinPage";

// Lazily load routes and code split with webpack
const LazyCounterPage = React.lazy(() =>
  import(/* webpackChunkName: "CounterPage" */ './containers/CounterPage')
);

const CounterPage = (props: Record<string, any>) => (
  <React.Suspense fallback={<h1>Loading...</h1>}>
    <LazyCounterPage {...props} />
  </React.Suspense>
);

export default function Routes() {
  return (
    <App>
      <Switch>
        <Route path={routes.COUNTER} component={CounterPage} />
        <Route path={routes.HOME} component={HomePage} />
        <Route path={routes.CREATE_GAME} component={CreatePage} />
        <Route path={routes.RULES} component={RulesPage} />
        <Route path={routes.SETTINGS} component={SettingsPage} />
        <Route path={routes.QUIT} component={QuitPage} />
        <Route path={routes.JOIN_GAME} component={JoinPage} />
      </Switch>
    </App>
  );
}
