import { Accordion, AccordionDetails, AccordionSummary } from '@mui/material';
import React from 'react';
import { useSelector } from 'react-redux';
import { getJsonOutput } from './reducer_selectors';

export const JsonOutput = () => {
    const jsonOutput = useSelector(getJsonOutput);
    return (
        <Accordion>
            <AccordionSummary>JSON</AccordionSummary>
            <AccordionDetails>{jsonOutput}</AccordionDetails>
        </Accordion>
    );
}