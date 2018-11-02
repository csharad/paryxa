import React, { Component } from "react";
import { Paper, TextField, withStyles } from "@material-ui/core";
import PropTypes from "prop-types";

const styles = theme => ({});

class NewTest extends Component {
  static propTypes = {
    styles: PropTypes.object.isRequired
  };

  render() {
    return (
      <Paper>
        <form>
          <TextField label="Name" />
        </form>
      </Paper>
    );
  }
}

export default withStyles(styles)(NewTest);
