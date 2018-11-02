import React from "react";
import { Mutation } from "react-apollo";
import gql from "graphql-tag";

const query = gql`
  mutation UpdateMyCredentials($user: UserCredentialsUpdate!) {
    updateMyCredentials(user: $user) {
      id
      email
    }
  }
`;

const UpdateMyCredentials = ({ children }) => (
  <Mutation mutation={query}>
    {(mutator, result) => children(mutator, result)}
  </Mutation>
);

export default UpdateMyCredentials;
