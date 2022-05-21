import React from 'react';
import { IconButton, Paper, Stack } from '@mui/material';
import { Add, Delete } from '@mui/icons-material';
import { registerTypeRenderer, renderValueOfType } from './ValueTypes';
import { useDispatch, useSelector } from 'react-redux';
import { getVecPaths, leafPath, parentPath } from '../behavior_tree/reducer_selectors';
import { addToVec, removeFromVec } from '../behavior_tree/actions';

export const VecDeleteButton = ({ onClick }) => {
    return <IconButton color="secondary" onClick={onClick}><Delete /></IconButton>;
};

export const VecItem = ({ path, children }) => {
    const vecParentPath = parentPath(path);
    const vecIndex = leafPath(path);
    const dispatch = useDispatch();
    const deleteItem = () => dispatch(removeFromVec(vecParentPath, vecIndex));
    return <Paper>{children}<VecDeleteButton onClick={deleteItem} /></Paper>;
};

export const VecAddButton = ({ path }) => {
    const dispatch = useDispatch();
    const addItem = () => dispatch(addToVec(path));
    return <IconButton color="primary" onClick={addItem}><Add /></IconButton>;
};

export const VecList = ({
    path, typeDesc,
}) => {
    const { itemType } = typeDesc;
    const vecPaths = useSelector(getVecPaths(path));
    const items = [
        ...vecPaths.map(vecPath => <VecItem path={vecPath}>{renderValueOfType(vecPath, itemType)}</VecItem>),
        <VecAddButton path={path} />,
    ];
    console.log(itemType);
    return (
        <Stack direction="row" spacing={2}>{items}</Stack>
    );
};

registerTypeRenderer('Vec', VecList);