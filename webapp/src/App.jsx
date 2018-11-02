import React, { Component } from "react";
import {
  CssBaseline,
  createMuiTheme,
  MuiThemeProvider,
  colors
} from "@material-ui/core";
import { Route } from "react-router-dom";
import ApolloClient from "apollo-boost";
import { ApolloProvider } from "react-apollo";
import NavigationBar from "./NavigationBar";
import Home from "./Home";
import Login from "./Login";
import SignUp from "./SignUp";
import VerifyAccount from "./VerifyAccount";
import GettingStarted from "./GettingStarted";
import Profile from "./profile/Profile";
import SolvedQuestions from "./SolvedQuestions";
import TestPaper from "./test/TestPaper";
import InfoBeforeTest from "./test/InfoBeforeTest";
import TestComplete from "./test/TestComplete";
import Dashboard from "./dashboard/Dashboard";
import Settings from "./settings/Settings";

const apolloClient = new ApolloClient({
  uri: "/graphql",
  request: operation => {
    const token = localStorage.getItem("paryxa-token");
    operation.setContext({
      headers: token
        ? {
            Authorization: `Basic ${token}`
          }
        : {}
    });
  }
});

const theme = createMuiTheme({
  palette: {
    primary: colors.blue
  },
  typography: {
    useNextVariants: true
  }
});

class App extends Component {
  render() {
    return (
      <ApolloProvider client={apolloClient}>
        <MuiThemeProvider theme={theme}>
          <CssBaseline />
          <NavigationBar />

          <Route path="/" component={Home} />
          <Route path="/login" component={Login} />
          <Route path="/signup" component={SignUp} />
          <Route path="/verify-your-account" component={VerifyAccount} />
          <Route path="/getting-started" component={GettingStarted} />
          <Route path="/profile" component={Profile} />
          <Route path="/solved-questions" component={SolvedQuestions} />
          <Route path="/test-paper" exact component={TestPaper} />
          <Route path="/test-paper/notice" exact component={InfoBeforeTest} />
          <Route path="/test-paper/completed" exact component={TestComplete} />
          <Route path="/dashboard" component={Dashboard} />
          <Route path="/settings" component={Settings} />
        </MuiThemeProvider>
      </ApolloProvider>
    );
  }
}

export default App;
