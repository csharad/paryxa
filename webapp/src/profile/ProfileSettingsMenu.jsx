import React, { Component } from "react";
import {
  Dialog,
  List,
  ListItem,
  ListItemText,
  withStyles
} from "@material-ui/core";
import PropTypes from "prop-types";
import { Link } from "react-router-dom";
import { ApolloConsumer } from "react-apollo";
import { withRouter } from "react-router-dom";

const styles = theme => ({
  button: {
    textTransform: "uppercase",
    padding: 0,
    textAlign: "center"
  },
  dialogRoot: {
    alignItems: "flex-start"
  },
  paper: {
    width: 250,
    marginTop: theme.spacing.unit * 12
  }
});

class ProfileSettingsMenu extends Component {
  static propTypes = {
    open: PropTypes.bool.isRequired,
    onClose: PropTypes.func,
    classes: PropTypes.object.isRequired,
    history: PropTypes.object.isRequired
  };

  render() {
    const { open, onClose, classes, history } = this.props;

    const menu = client => (
      <List>
        <ListItem button component={Link} to="/settings">
          <ListItemText className={classes.button}>Settings</ListItemText>
        </ListItem>
        <ListItem
          button
          onClick={() => {
            // Flush the token and state.
            localStorage.removeItem("paryxa-token");
            client.resetStore();
            // Redirect to the home page.
            history.push("/");
          }}
        >
          <ListItemText className={classes.button}>Logout</ListItemText>
        </ListItem>
      </List>
    );

    return (
      <Dialog
        open={open}
        onClose={onClose}
        classes={{ root: classes.dialogRoot, paper: classes.paper }}
      >
        <ApolloConsumer>{menu}</ApolloConsumer>
      </Dialog>
    );
  }
}

export default withRouter(withStyles(styles)(ProfileSettingsMenu));
