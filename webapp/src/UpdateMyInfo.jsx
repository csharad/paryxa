import React from 'react';
import { Mutation } from 'react-apollo';
import gql from 'graphql-tag';

const query = gql`
    mutation UpdateMyInfo($user: UserInfoUpdate!) {
        updateMe(user: $user) {
            id
            firstName
            lastName
            fullName
            gender
            contact
        }
    }
`;

const UpdateMyInfo = ({ children }) => (
    <Mutation
        mutation={query}
    >
        {(mutator, result) => children(mutator, result)}
    </Mutation>
);

export default UpdateMyInfo;
