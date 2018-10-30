import React from 'react';
import { Query } from 'react-apollo';
import gql from 'graphql-tag';

export const ME = gql`
    query Me {
        me {
            id
            firstName
            lastName
            fullName
            email
            gender
            type
        }
    }
`;

// Query to get an authenticated user if possible.
const AuthenticatedUser = ({ children }) => (
    <Query query={ME}>
        {({ loading, error, data }) => {
            if (error && error.graphQLErrors[0].extensions.kind === 'UNAUTHORIZED') {
                return children({ loading, unauthorized: true });
            }
            return children({ data, loading });
        }}
    </Query>
);

export default AuthenticatedUser;
