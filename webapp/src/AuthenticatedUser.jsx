import React from 'react';
import { Query } from 'react-apollo';
import gql from 'graphql-tag';

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
            if (errors && error.graphQLErrors[0].extensions.kind === 'UNAUTHENTICATED') {
                return children({ loading, isLogged: false });
            }
            return children({ data, loading, isLogged: true });
        }}
    </Query>
);

export default AuthenticatedUser;
