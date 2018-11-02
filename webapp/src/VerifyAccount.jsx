import React, { Component } from "react";
import { Typography, withStyles } from "@material-ui/core";
import PropTypes from "prop-types";

const styles = theme => ({
  margin: {
    margin: theme.spacing.unit * 2
  }
});

class VerifyAccount extends Component {
  static propTypes = {
    classes: PropTypes.object.isRequired
  };

  render() {
    const { classes } = this.props;

    return (
      <Typography className={classes.margin}>
        A verification link has been sent to your e-mail. Go to your inbox and
        click on the sent link to activate this account.
      </Typography>
    );
  }
}

export default withStyles(styles)(VerifyAccount);
