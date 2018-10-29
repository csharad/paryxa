import React from 'react';
import { Query } from 'react-apollo';
import gql from 'graphql-tag';

// Query to get an authenticated user if possible.
const AuthenticatedUser = ({ children }) => (
    <Query query={gql`
        {
            me {
                id
                firstName
                lastName
                email
                gender
                type
            }
        }
    `}>
        {({ loading, error, data }) => {
            if (!loading) {
                if (error && error.graphQLErrors[0].extensions.kind === 'UNAUTHORIZED') {
                    return children({ loading, isLogged: false });
                }
                return children({ data, loading, isLogged: true });
            } else {
                return children({ loading, isLogged: false });
            }
        }}
    </Query>
);

export default AuthenticatedUser;
