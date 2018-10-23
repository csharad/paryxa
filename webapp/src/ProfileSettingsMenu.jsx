import React, { Component } from 'react';
import { Dialog, List, ListItem, ListItemText, withStyles } from '@material-ui/core';
import PropTypes from 'prop-types';

const styles = theme => ({
    button: {
        textTransform: 'uppercase',
        padding: 0,
        textAlign: 'center',
    },
    dialogRoot: {
        alignItems: 'flex-start'
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
    };

    render() {
        const { open, onClose, classes } = this.props;

        return (
            <Dialog
                open={open}
                onClose={onClose}
                classes={{ root: classes.dialogRoot, paper: classes.paper }}
            >
                <List>
                    <ListItem button>
                        <ListItemText className={classes.button}>Settings</ListItemText>
                    </ListItem>
                    <ListItem button>
                        <ListItemText className={classes.button}>Logout</ListItemText>
                    </ListItem>
                </List>
            </Dialog>
        );
    }
}

export default withStyles(styles)(ProfileSettingsMenu);
