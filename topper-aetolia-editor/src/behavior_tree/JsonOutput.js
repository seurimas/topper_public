import { Accordion, AccordionDetails, AccordionSummary, TextField } from '@mui/material';
import React from 'react';
import { useDispatch, useSelector } from 'react-redux';
import { loadJson } from './actions';
import { getJsonOutput } from './reducer_selectors';

export const JsonOutput = ({ treeName }) => {
    const dispatch = useDispatch();
    const jsonOutput = useSelector(getJsonOutput(treeName));
    const load = (event) => dispatch(loadJson(treeName, event.target.value));
    return (
        <TextField label={treeName} fullWidth multiline value={jsonOutput} onChange={load} />
    );
};