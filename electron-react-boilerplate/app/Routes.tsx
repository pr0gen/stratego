/* eslint react/jsx-props-no-spreading: off */
import React from 'react';
import { Switch, Route } from 'react-router-dom';
import App from './containers/App';
import HomePage from './containers/HomePage';
import RulesPage from "./containers/RulesPage";
import SettingsPage from "./containers/SettingsPage";
import QuitPage from "./containers/QuitPage";
import JoinPage from "./containers/JoinPage";

// Lazily load routes and code split with webpack
const LazyCounterPage = React.lazy(() =>
  import(/* webpackChunkName: "CounterPage" */ './containers/CounterPage')
);

const LazyCreatePage = React.lazy(() =>
  import(/* webpackChunkName: "CounterPage" */ './containers/CreatePage')
);

const CounterPage = (props: Record<string, any>) => (
  <React.Suspense fallback={<h1>Loading...</h1>}>
    <LazyCounterPage {...props} />
  </React.Suspense>
);

const CreatePage = (props: Record<string, any>) => (
  <React.Suspense fallback={<h1>Loading...</h1>}>
    <LazyCreatePage {...props} />
  </React.Suspense>
);

export default function Routes() {
  return (
    <App>
      <Switch>
        <Route name="counterRouter" path="/counter" component={CounterPage} />
        <Route name="createRouteur" path="/create" component={CreatePage} />
        <Route name="homeRouter" path="/" component={HomePage} />
        <Route name="rulesRouter" path="rules" component={RulesPage} />
        <Route name="settingRouter" path="/settings" component={SettingsPage} />
        <Route name="quitRouter" path="/quit" component={QuitPage} />
        <Route name="joinRouter" path="/join" component={JoinPage} />
      </Switch>
    </App>
  );
}
