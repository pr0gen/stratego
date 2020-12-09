/* eslint react/jsx-props-no-spreading: off */
import React from 'react';
import { Switch, Route } from 'react-router-dom';
import App from './containers/App';
import HomePage from './containers/HomePage';

const LazyCreatePage = React.lazy(() =>
  import(/* webpackChunkName: "CounterPage" */ './containers/CreatePage')
);
const LazyRulesPage = React.lazy(() =>
  import(/* webpackChunkName: "CounterPage" */ './containers/RulesPage')
);
const LazySettingsPage = React.lazy(() =>
  import(/* webpackChunkName: "CounterPage" */ './containers/SettingsPage')
);
const LazyJoinPage = React.lazy(() =>
  import(/* webpackChunkName: "CounterPage" */ './containers/JoinPage')
);
const LazyQuitPage = React.lazy(() =>
  import(/* webpackChunkName: "CounterPage" */ './containers/QuitPage')
);
const CreatePage = (props: Record<string, any>) => (
  <React.Suspense fallback={<h1>Loading...</h1>}>
    <LazyCreatePage {...props} />
  </React.Suspense>
);
const RulesPage = (props: Record<string, any>) => (
  <React.Suspense fallback={<h1>Loading...</h1>}>
    <LazyRulesPage {...props} />
  </React.Suspense>
);
const SettingsPage = (props: Record<string, any>) => (
  <React.Suspense fallback={<h1>Loading...</h1>}>
    <LazySettingsPage {...props} />
  </React.Suspense>
);
const JoinPage = (props: Record<string, any>) => (
  <React.Suspense fallback={<h1>Loading...</h1>}>
    <LazyJoinPage {...props} />
  </React.Suspense>
);
const QuitPage = (props: Record<string, any>) => (
  <React.Suspense fallback={<h1>Loading...</h1>}>
    <LazyQuitPage {...props} />
  </React.Suspense>
);
export default function Routes() {
  return (
    <App>
      <Switch>
        <Route name="createRouter" path="/create" component={CreatePage} />
        <Route name="rulesRouter" path="/rules" component={RulesPage} />
        <Route name="settingRouter" path="/settings" component={SettingsPage} />
        <Route name="joinRouter" path="/join" component={JoinPage} />
        <Route name="quitRouter" path="/quit" component={QuitPage} />
        <Route name="homeRouter" path="/" component={HomePage} />
      </Switch>
    </App>
  );
}
