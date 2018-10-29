import React, { Component } from 'react';
import { CssBaseline, createMuiTheme, MuiThemeProvider, colors } from '@material-ui/core';
import { Route } from 'react-router-dom';
import ApolloClient from 'apollo-boost';
import { ApolloProvider } from 'react-apollo';
import NavigationBar from './NavigationBar';
import Home from './Home';
import Login from './Login';
import SignUp from './SignUp';
import VerifyAccount from './VerifyAccount';
import GettingStarted from './GettingStarted';
import Profile from './Profile';
import SolvedQuestions from './SolvedQuestions';
import TestPaper from './TestPaper';
import InfoBeforeTest from './InfoBeforeTest';
import TestComplete from './TestComplete';
import Dashboard from './Dashboard';
import Settings from './Settings';

const apolloClient = new ApolloClient({
    uri: '/graphql',
    request: (operation) => {
        const token = localStorage.getItem('paryxa-token');
        operation.setContext({
            headers: token ? {
                Authorization: `Basic ${token}`
            } : {}
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
                    <CssBaseline></CssBaseline>
                    <NavigationBar></NavigationBar>

                    <Route path="/" component={Home}></Route>
                    <Route path="/login" component={Login}></Route>
                    <Route path="/signup" component={SignUp}></Route>
                    <Route path="/verify-your-account" component={VerifyAccount}></Route>
                    <Route path="/getting-started" component={GettingStarted}></Route>
                    <Route path="/profile" component={Profile}></Route>
                    <Route path="/solved-questions" component={SolvedQuestions}></Route>
                    <Route path="/test-paper" exact component={TestPaper}></Route>
                    <Route path="/test-paper/notice" exact component={InfoBeforeTest}></Route>
                    <Route path="/test-paper/completed" exact component={TestComplete}></Route>
                    <Route path="/dashboard" component={Dashboard}></Route>
                    <Route path="/settings" component={Settings}></Route>
                </MuiThemeProvider>
            </ApolloProvider>
        );
    }
}

export default App;
