import React, { Component, Fragment } from 'react';
import { List, ListItem, ListItemText, Divider } from '@material-ui/core';
import PropTypes from 'prop-types';
import ReportCard from './ReportCard';
import TestPreface from './TestPreface';

class TestList extends Component {
    static propTypes = {
        className: PropTypes.string,
        list: PropTypes.arrayOf(PropTypes.shape({
            title: PropTypes.string.isRequired,
            time: PropTypes.string.isRequired,
            liveTest: PropTypes.bool,
        })),
        newTest: PropTypes.bool,
    };

    state = {
        open: false,
        title: null,
        time: null,
        liveTest: false,
    };

    render() {
        const { className, list, newTest } = this.props;
        const { open, title, time, liveTest } = this.state;

        return (
            <Fragment>
                <List className={className}>
                    {list.map((item, index) =>
                        <Fragment key={index}>
                            <ListItem button onClick={() => this.openDialog(item)}>
                                <ListItemText primary={item.title} secondary={item.time}></ListItemText>
                            </ListItem>
                            {index !== list.length - 1 ? <Divider></Divider> : ''}
                        </Fragment>)}
                </List>

                {
                    newTest ?
                        <TestPreface
                            onClose={this.handleClose}
                            open={open}
                            title={title}
                            time={time}
                            liveTest={liveTest}
                        >
                        </TestPreface>
                        : <ReportCard
                            onClose={this.handleClose}
                            open={open}
                            title={title}
                            time={time}
                        ></ReportCard>
                }
            </Fragment>
        );
    }

    handleClose = () => {
        this.setState({
            open: false,
        });
    };

    openDialog = ({ title, time, liveTest }) => {
        this.setState({
            title,
            time,
            liveTest,
            open: true,
        });
    };
}

export default TestList;
